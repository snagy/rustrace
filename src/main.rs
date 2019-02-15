use std::fs::File;

extern crate image;
pub use image::png::PNGEncoder;
pub use image::ImageFormat::PNG;

extern crate time;

extern crate rand;
use rand::{thread_rng, Rng};

extern crate scoped_threadpool;
use scoped_threadpool::Pool;

#[macro_use]
extern crate rustacuda;

#[macro_use]
extern crate rustacuda_derive;
extern crate rustacuda_core;

mod snmath;
mod snrt;

use snmath::Vector3;
use snmath::Ray;

use snrt::Camera;
use snrt::world::World;

fn cuda_test() -> Result<(), Box<dyn std::error::Error>> {
    use rustacuda::prelude::*;
    use rustacuda::memory::DeviceBox;

    // Initialize the CUDA API
    rustacuda::init(CudaFlags::empty()).expect("CUDA init failed");
    
    // Get the first device
    let device = Device::get_device(0).expect("CUDA device creation failed");

    // Create a context associated to this device
    let _context = Context::create_and_push(ContextFlags::MAP_HOST | ContextFlags::SCHED_AUTO, device).expect("CUDA context failed");

    // Load the module containing the function we want to call
    let module_data = std::ffi::CString::new(include_str!("../cuda_resources/add.ptx")).unwrap();
    let module = Module::load_from_string(&module_data).expect("CUDA module load failed");

    // Create a stream to submit work to
    let stream = Stream::new(StreamFlags::NON_BLOCKING, None).expect("Stream create failed");

    // Allocate space on the device and copy numbers to it.
    let mut x = DeviceBox::new(&10.0f32)?;
    let mut y = DeviceBox::new(&20.0f32)?;
    let mut result = DeviceBox::new(&0.0f32)?;

    // Launching kernels is unsafe since Rust can't enforce safety - think of kernel launches
    // as a foreign-function call. In this case, it is - this kernel is written in CUDA C.
    unsafe {
        // Launch the `sum` function with one block containing one thread on the given stream.
        launch!(module.sum<<<1, 1, 0, stream>>>(
            x.as_device_ptr(),
            y.as_device_ptr(),
            result.as_device_ptr(),
            1 // Length
        )).expect("CUDA launch failed");
    }

    // The kernel launch is asynchronous, so we wait for the kernel to finish executing
    stream.synchronize().expect("CUDA synchronize failed");

    // Copy the result back to the host
    let mut result_host = 0.0f32;
    result.copy_to(&mut result_host).expect("CUDA result copy failed");
    
    println!("Sum is {}", result_host);

    Ok(())
}

fn color(r: Ray, world: &World, bounce: i32) -> Vector3 {
    if bounce > 50 {
        return Vector3::default();
    }

    let max_t = 100000.0;
    let min_t = 0.001;

    let best = world.trace(r, min_t, max_t);

    match best.1 {
        Some(b) => {
            let scat = b.hit_process(&r,best.0);
            if scat.0 {
                return scat.2*color(scat.1, world, bounce+1);
            }
            return Vector3::default();
        },
        None => {
            //fake sky
            let dir_norm = r.direction.normalize();
            let t = 0.5*(dir_norm.y + 1.0);
            return Vector3::lerp(&Vector3{x:1.0,y:1.0,z:1.0}, &Vector3{x:0.5,y:0.7,z:1.0}, t);
        }
    }
}

fn main() -> std::io::Result<()> {
    cuda_test().unwrap();

    let args: Vec<String> = std::env::args().collect();

    if args.len() < 3 {
        println!("run with two digits for width and height");
        return Ok(());
    }


    let start_time = time::precise_time_s();

    let width = match args[1].parse::<u32>() {
        Ok(n) => n,
        Err(_e) => 256,
    };

    let height = match args[2].parse::<u32>() {
        Ok(n) => n,
        Err(_e) => 256,
    };

    let box_pct = match args[3].parse::<f32>() {
        Ok(n) => n,
        Err(_e) => 0.5,
    };

    let f_width = width as f32;
    let f_height = height as f32;

    let n_samples = 150;

    println!("width {}, height {}", width, height);


    let look_from = Vector3 {x:7.0,y:2.0,z:2.0};
    let look_at = Vector3 {x:0.0,y:0.0,z:0.0};
    let focal_dist = (look_from-look_at).length();
    let cam = Camera::create_camera(look_from, look_at, Vector3 {x:0.0,y:1.0,z:0.0}, 40.0, f_width/f_height, 0.3, focal_dist);

    let world = World::create(box_pct);

    let buffer_stride = 3;
    let mut buffer_rgb: Vec<u8> = Vec::new();

    let mut thread_jobs = Vec::new();
    for y in 0..height {
        let mut line: Vec<u8> = vec![0;(width*buffer_stride) as usize];
        thread_jobs.push((y,line,&world)); // should world be an arc? is this copying all of world or just the pointer?
    }

    let mut pool = Pool::new(16);

    pool.scoped( |scope| {
        for e in &mut thread_jobs {
            scope.execute(move || {
                let mut rng = thread_rng();
                let y = e.0;
                for x in 0..width {
                    let mut c = Vector3{x:0.0,y:0.0,z:0.0};
                    for _sample in 0..n_samples {
                        let u = (x as f32 + rng.gen_range::<f32>(0.0,1.0)) / f_width;
                        let v = (y as f32 + rng.gen_range::<f32>(0.0,1.0)) / f_height;
                        let r = cam.get_ray(u,v);
                        c = c + color(r, e.2, 0);
                    }

                    let c = (c / n_samples as f32).powf(1.0/2.2) * 255.99;
                    let pixel_idx = (x*buffer_stride) as usize;
                    e.1[pixel_idx+0] = c.x as u8;
                    e.1[pixel_idx+1] = c.y as u8;
                    e.1[pixel_idx+2] = c.z as u8;
                }
            });
        }
    });

    let trace_time = time::precise_time_s();

    for mut line in thread_jobs.drain(..).rev() {
        buffer_rgb.append(&mut line.1);
    }

    let file_png = File::create(format!("out{}x{}.png", width, height))?;
    let encoder = PNGEncoder::new(file_png);
    encoder.encode(&buffer_rgb,width,height,image::ColorType::RGB(8))?;

    println!("Execution time: {} (tracing took {})", time::precise_time_s()-start_time, trace_time-start_time);

    Ok(())
}