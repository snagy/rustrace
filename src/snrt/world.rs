extern crate rand;
use rand::{thread_rng, Rng};

use snmath::Vector3;
use snmath::Ray;

use snrt::Hitable;
use snrt::Sphere;
use snrt::AABox;
use snrt::material::Material;
use snrt::material::Lambertian;
use snrt::material::Metallic;
use snrt::material::Dielectric;

pub struct World {
    pub entities: Vec<Box<Hitable + Sync>>,
}

impl World {
    pub fn trace(&self, r: Ray, min_t: f32, max_t: f32) -> (f32, Option<&Box<Hitable + Sync>>) {
        let world_iter = self.entities.iter();
        let mut best:(f32, Option<&Box<Hitable + Sync>>) = (::std::f32::MAX, None);
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

        return best;
    }

    pub fn create(box_pct : f32) -> World {
        let mut new_world = World { entities: Vec::new() };
        let mut world_rng = thread_rng();

        new_world.entities.push(Box::new(Sphere {pos: Vector3 {x:0.0, y:-1000.0, z:0.0}, radius: 1000.0, 
                            material:Box::new(Lambertian{albedo:Vector3{x:0.4,y:0.4,z:0.5}})}));

        let ball_min = -4;
        let ball_max = 4;
        for a in ball_min..ball_max {
            for b in ball_min..ball_max {
                let mat_val = world_rng.gen_range::<f32>(0.0,1.0);
                let type_val = world_rng.gen_range::<f32>(0.0,1.0);
                let rands = Vector3{x:world_rng.gen_range::<f32>(0.0,1.0) * 0.1 + 0.2,
                                    y:world_rng.gen_range::<f32>(0.0,1.0) * 0.1 + 0.2,
                                    z:world_rng.gen_range::<f32>(0.0,1.0) * 0.1 + 0.2};
                let center = Vector3 {x:a as f32 + 0.6 * world_rng.gen_range::<f32>(0.0,1.0), y:rands.z, z:b as f32 + 0.6 * world_rng.gen_range::<f32>(0.0,1.0) };
                let mut mat : Box<Material + Sync>;
                if mat_val < 0.8 {
                    mat = Box::new(Lambertian{albedo:Vector3{x:world_rng.gen_range::<f32>(0.0,1.0)*world_rng.gen_range::<f32>(0.0,1.0),
                                                             y:world_rng.gen_range::<f32>(0.0,1.0)*world_rng.gen_range::<f32>(0.0,1.0),
                                                             z:world_rng.gen_range::<f32>(0.0,1.0)*world_rng.gen_range::<f32>(0.0,1.0)}});
                }
                else if mat_val < 0.95 {
                    mat = Box::new(Metallic{ albedo:Vector3{ x:0.5*(1.0+world_rng.gen_range::<f32>(0.0,1.0)),
                                                             y:0.5*(1.0+world_rng.gen_range::<f32>(0.0,1.0)),
                                                             z:0.5*(1.0+world_rng.gen_range::<f32>(0.0,1.0))},
                                                             roughness:0.5*world_rng.gen_range::<f32>(0.0,1.0)});

                }
                else {
                    mat = Box::new(Dielectric{ior:1.5});
                }

                if type_val < box_pct {
                    new_world.entities.push(Box::new(AABox {pos:center, dims:rands, material:mat}))
                }
                else {
                    new_world.entities.push(Box::new(Sphere {pos:center, radius:rands.z, material:mat}));
                }

            }
        }


        new_world.entities.push(Box::new(AABox {pos: Vector3 {x:-4.0, y:1.0, z:-1.0}, dims: Vector3 {x:1.0,y:2.0,z:1.0},
                                    material:Box::new(Lambertian{albedo:Vector3{x:0.1,y:0.2,z:0.5}})}));
        //new_world.entities.push(Box::new(Sphere {pos: Vector3 {x:-4.0, y:1.0, z:-1.0}, radius: 1.0,
        //                            material:Box::new(Lambertian{albedo:Vector3{x:0.1,y:0.2,z:0.5}})}));
        new_world.entities.push(Box::new(AABox {pos: Vector3 {x:4.0, y:1.0, z:-1.0}, dims: Vector3 {x:1.0,y:1.0,z:1.0},
                                    material:Box::new(Metallic{albedo:Vector3{x:0.7,y:0.6,z:0.5},roughness:0.1})}));
        new_world.entities.push(Box::new(Sphere {pos: Vector3 {x:0.0, y:1.0, z:-1.0}, radius: 1.0, 
                                    material:Box::new(Dielectric{ior:1.5})}));

        return new_world;
    }
}