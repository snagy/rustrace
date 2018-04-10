use std::fs::File;
use std::io::prelude::*;

extern crate rand;
use rand::{thread_rng, ThreadRng, Rng};

mod snmath;
mod snrt;

use snmath::Vector3;
use snmath::Ray;

use snrt::Sphere;
use snrt::Hitable;
use snrt::Camera;

fn random_vec_from_unit_sphere(rng: &mut ThreadRng) -> Vector3 {
    let mut p = Vector3 {x:100.0,y:0.0,z:0.0};
    let ones = Vector3 {x:1.0, y:1.0, z:1.0};
    while p.length_sq() > 1.0 {
        p = Vector3{x:rng.gen_range::<f64>(0.0,1.0),y:rng.gen_range::<f64>(0.0,1.0),z:rng.gen_range::<f64>(0.0,1.0)}*2.0 - ones;
    }
    return p;
}

fn color(r: Ray, world: &Vec<&Hitable>, rng: &mut ThreadRng) -> Vector3 {
    let world_iter = world.iter();

    // todo:  fix this to pick the nearest point!
    for hitable in world_iter {
        let res = hitable.hit(&r, 0.0001, 10000.0);
        if res.0 {
            let target = res.1.pos + res.1.normal + random_vec_from_unit_sphere(rng);
            return  color(Ray{origin:res.1.pos, direction:target-res.1.pos}, world, rng)*0.5;
        }
    }
    
    //fake sky
    let dir_norm = r.direction.normalize();
    let t = 0.5*(dir_norm.y + 1.0);
    Vector3::lerp(&Vector3{x:1.0,y:1.0,z:1.0}, &Vector3{x:0.5,y:0.7,z:1.0}, t)
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let mut file = File::create("out.ppm")?;
    let mut rng = thread_rng();


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

    let cam = Camera {
        lower_left_corner: Vector3{x:-2.0,y:-1.0,z:-1.0},
        horizontal: Vector3{x:4.0,y:0.0,z:0.0},
        vertical: Vector3{x:0.0,y:2.0,z:0.0},
        origin: Vector3{x:0.0,y:0.0,z:0.0},
    };

    // this feels...uncomfortable
    let mut world: Vec<&Hitable> = Vec::new();
    world.push(&Sphere {pos: Vector3 {x:0.0, y:0.0, z:-1.0}, radius: 0.5});
    world.push(&Sphere {pos: Vector3 {x:0.0, y:-100.5, z:-1.0}, radius: 100.0});

    for y in (0..height).rev() {
        for x in 0..width {
            let mut c = Vector3{x:0.0,y:0.0,z:0.0};
            for _sample in 0..n_samples {
                let u = (x as f64 + rng.gen_range::<f64>(0.0,1.0)) / f_width;
                let v = (y as f64 + rng.gen_range::<f64>(0.0,1.0)) / f_height;
                let r = cam.get_ray(u,v);
                c = c + color(r, &world, &mut rng);
            }

            let c = c * 255.99 / n_samples as f64;
            write!(file, "{} {} {}\n", c.x as i32, c.y as i32, c.z as i32)?;
        } 
    }

    Ok(())
}