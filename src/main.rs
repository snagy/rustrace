use std::fs::File;

extern crate image;
pub use image::png::PNGEncoder;
pub use image::ImageFormat::PNG;

extern crate time;

extern crate rand;
use rand::{thread_rng, Rng};

mod snmath;
mod snrt;

use snmath::Vector3;
use snmath::Ray;

use snrt::Sphere;
use snrt::Hitable;
use snrt::Camera;
use snrt::material::Lambertian;
use snrt::material::Metallic;
use snrt::material::Dielectric;

fn color(r: Ray, world: &Vec<Box<Hitable>>, bounce: i32) -> Vector3 {
    let world_iter = world.iter();

    if bounce > 50 {
        return Vector3::default();
    }

    let max_t = 100000.0;
    let min_t = 0.001;

    let mut best:(f32, Option<&Box<Hitable>>) = (std::f32::MAX, None);
    for hitable in world_iter {
        let res = hitable.hit_check(&r, min_t, max_t);
        if res.is_some() {
            let new_t = res.unwrap();

            if new_t < best.0 {
                best.0 = new_t;
                best.1 = Some(hitable);
            }
        }
    }

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
    let args: Vec<String> = std::env::args().collect();
    let mut rng = thread_rng();

    let start_time = time::precise_time_s();

    let width = match args[1].parse::<u32>() {
        Ok(n) => n,
        Err(_e) => 256,
    };

    let height = match args[2].parse::<u32>() {
        Ok(n) => n,
        Err(_e) => 256,
    };

    let f_width = width as f32;
    let f_height = height as f32;

    let n_samples = 150;

    println!("width {}, height {}", width, height);


    let look_from = Vector3 {x:7.0,y:2.0,z:2.0};
    let look_at = Vector3 {x:0.0,y:0.0,z:0.0};
    let focal_dist = (look_from-look_at).length();

    let cam = Camera::create_camera(look_from, look_at, Vector3 {x:0.0,y:1.0,z:0.0}, 40.0, f_width/f_height, 0.3, focal_dist);

    let mut world: Vec<Box<Hitable>> = Vec::new();
    world.push(Box::new(Sphere {pos: Vector3 {x:0.0, y:-1000.0, z:0.0}, radius: 1000.0, 
                                material:Box::new(Lambertian{albedo:Vector3{x:0.4,y:0.4,z:0.5}})}));

    let ball_min = -4;
    let ball_max = 4;
    for a in ball_min..ball_max {
        for b in ball_min..ball_max {
            let mat_val = rng.gen_range::<f32>(0.0,1.0);
            let rad = rng.gen_range::<f32>(0.0,1.0) * 0.1 + 0.2;
            let center = Vector3 {x:a as f32 + 0.9 * rng.gen_range::<f32>(0.0,1.0), y:rad, z:b as f32 + 0.9*rng.gen_range::<f32>(0.0,1.0) };
            if mat_val < 0.8 {
                world.push(Box::new(Sphere {pos:center, radius: rad,
                                material:Box::new(Lambertian{albedo:Vector3{x:rng.gen_range::<f32>(0.0,1.0)*rng.gen_range::<f32>(0.0,1.0),
                                                                            y:rng.gen_range::<f32>(0.0,1.0)*rng.gen_range::<f32>(0.0,1.0),
                                                                            z:rng.gen_range::<f32>(0.0,1.0)*rng.gen_range::<f32>(0.0,1.0)}})}));
            }
            else if mat_val < 0.95 {
                world.push(Box::new(Sphere {pos:center, radius: rad,
                                material:Box::new(Metallic{ albedo:Vector3{ x:0.5*(1.0+rng.gen_range::<f32>(0.0,1.0)),
                                                                            y:0.5*(1.0+rng.gen_range::<f32>(0.0,1.0)),
                                                                            z:0.5*(1.0+rng.gen_range::<f32>(0.0,1.0))},
                                                            roughness:0.5*rng.gen_range::<f32>(0.0,1.0)})}));

            }
            else {
                world.push(Box::new(Sphere {pos:center, radius: rad,
                                material:Box::new(Dielectric{ior:1.5})}));
            }
        }
    }


    world.push(Box::new(Sphere {pos: Vector3 {x:-4.0, y:1.0, z:-1.0}, radius: 1.0,
                                material:Box::new(Lambertian{albedo:Vector3{x:0.1,y:0.2,z:0.5}})}));
    world.push(Box::new(Sphere {pos: Vector3 {x:4.0, y:1.0, z:-1.0}, radius: 1.0,
                                material:Box::new(Metallic{albedo:Vector3{x:0.7,y:0.6,z:0.5},roughness:0.1})}));
    world.push(Box::new(Sphere {pos: Vector3 {x:0.0, y:1.0, z:-1.0}, radius: 1.0, 
                                material:Box::new(Dielectric{ior:1.5})}));

    /* PPM write
    let mut file = File::create("out.ppm")?;
    write!(file, "P3\n")?;
    write!(file, "{} {}\n255\n", width, height)?;
    */

    let buffer_stride = 3;
    let num_pixels = height*width;
    let mut buffer_rgb: Vec<u8> = vec![0;(num_pixels*buffer_stride) as usize];

    for y in 0..height {
        for x in 0..width {
            let mut c = Vector3{x:0.0,y:0.0,z:0.0};
            for _sample in 0..n_samples {
                let u = (x as f32 + rng.gen_range::<f32>(0.0,1.0)) / f_width;
                let v = (y as f32 + rng.gen_range::<f32>(0.0,1.0)) / f_height;
                let r = cam.get_ray(u,v);
                c = c + color(r, &world, 0);
            }

            let c = (c / n_samples as f32).powf(1.0/2.2) * 255.99;
            let pixel_idx = (((height-y-1)*width + x)*buffer_stride) as usize;
            buffer_rgb[pixel_idx+0] = c.x as u8;
            buffer_rgb[pixel_idx+1] = c.y as u8;
            buffer_rgb[pixel_idx+2] = c.z as u8;
        } 
    }

    let file_png = File::create("out.png")?;
    let encoder = PNGEncoder::new(file_png);
    encoder.encode(&buffer_rgb,width,height,image::ColorType::RGB(8))?;

    /* PPM write
    for p in (0..num_pixels).rev() {
        let pixel_idx = (p*buffer_stride) as usize;
        write!(file, "{} {} {}\n", buffer_rgb[pixel_idx+0] as i32, buffer_rgb[pixel_idx+1] as i32, buffer_rgb[pixel_idx+2] as i32)?;
    }
    */

    println!("Execution time: {}", time::precise_time_s()-start_time);

    Ok(())
}