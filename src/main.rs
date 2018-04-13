use std::fs::File;
use std::io::prelude::*;

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

    let mut best:(f64, Option<&Box<Hitable>>) = (std::f64::MAX, None);
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
    let mut file = File::create("out.ppm")?;
    let mut rng = thread_rng();

    let start_time = time::precise_time_s();


    write!(file, "P3\n")?;

    let width = match args[1].parse::<i32>() {
        Ok(n) => n,
        Err(_e) => 256,
    };
    let f_width = width as f64;

    let height = match args[2].parse::<i32>() {
        Ok(n) => n,
        Err(_e) => 256,
    };
    let f_height = height as f64;

    let n_samples = 100;

    println!("width {}, height {}", width, height);

    write!(file, "{} {}\n255\n", width, height)?;

    let look_from = Vector3 {x:3.0,y:3.0,z:2.0};
    let look_at = Vector3 {x:0.0,y:0.0,z:-1.0};
    let focal_dist = (look_from-look_at).length();

    let cam = Camera::create_camera(look_from, look_at, Vector3 {x:0.0,y:1.0,z:0.0}, 15.0, f_width/f_height, 2.0, focal_dist);

    let mut world: Vec<Box<Hitable>> = Vec::new();
    world.push(Box::new(Sphere {pos: Vector3 {x:0.0, y:-1000.0, z:0.0}, radius: 1000.0, 
                                material:Box::new(Lambertian{albedo:Vector3{x:0.2,y:0.8,z:0.0}})}));
    world.push(Box::new(Sphere {pos: Vector3 {x:0.0, y:0.0, z:-1.0}, radius: 0.5,
                                material:Box::new(Lambertian{albedo:Vector3{x:0.1,y:0.2,z:0.5}})}));
    world.push(Box::new(Sphere {pos: Vector3 {x:1.0, y:0.0, z:-1.0}, radius: 0.5,
                                material:Box::new(Metallic{albedo:Vector3{x:0.8,y:0.6,z:0.2},roughness:0.3})}));
    world.push(Box::new(Sphere {pos: Vector3 {x:-1.0, y:0.0, z:-1.0}, radius: 0.5, 
                                material:Box::new(Dielectric{ior:1.5})}));

    for y in (0..height).rev() {
        for x in 0..width {
            let mut c = Vector3{x:0.0,y:0.0,z:0.0};
            for _sample in 0..n_samples {
                let u = (x as f64 + rng.gen_range::<f64>(0.0,1.0)) / f_width;
                let v = (y as f64 + rng.gen_range::<f64>(0.0,1.0)) / f_height;
                let r = cam.get_ray(u,v);
                c = c + color(r, &world, 0);
            }

            let c = c * 255.99 / n_samples as f64;
            write!(file, "{} {} {}\n", c.x as i32, c.y as i32, c.z as i32)?;
        } 
    }

    println!("Execution time: {}", time::precise_time_s()-start_time);

    Ok(())
}