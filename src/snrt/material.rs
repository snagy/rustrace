use snmath::Ray;
use snmath::Vector3;

extern crate rand;
use rand::{thread_rng, Rng};


pub trait Material {
    fn scatter(&self, r_in: &Ray, pos: Vector3, normal: Vector3) -> (bool, Ray, Vector3);
}

#[derive(Clone,Copy,PartialEq,Default,Debug)]
pub struct Lambertian {
    pub albedo: Vector3,
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, pos: Vector3, normal: Vector3) -> (bool, Ray, Vector3) {
        let target = pos + normal + Vector3::generate_random_unit_vector();
        let scattered = Ray{origin:pos, direction:target-pos};
        let attenuation = self.albedo;
        (true,scattered,attenuation)
    }
}

#[derive(Clone,Copy,PartialEq,Default,Debug)]
pub struct Metallic {
    pub albedo: Vector3,
    pub roughness: f32,
}

impl Material for Metallic {
    fn scatter(&self, r_in: &Ray, pos: Vector3, normal: Vector3) -> (bool, Ray, Vector3) {
        let reflected_vec = r_in.direction.normalize().reflect_on(&normal);
        let scattered_ray = Ray{origin:pos, direction:reflected_vec + self.roughness*Vector3::generate_random_unit_vector()};
        let attenuation = self.albedo;
        (normal.dot(&scattered_ray.direction) > 0.0, scattered_ray, attenuation)
    }
}


#[derive(Clone,Copy,PartialEq,Default,Debug)]
pub struct Dielectric {
    pub ior: f32,
}

impl Dielectric {
    fn schlick(cosine: f32, ior: f32) -> f32 {
        let r0 = (1.0-ior) / (1.0+ior);
        let r0 = r0 * r0;
        return r0 + (1.0-r0)*(1.0-cosine).powi(5);
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, pos: Vector3, normal: Vector3) -> (bool, Ray, Vector3) {
        let attenuation = Vector3 {x: 1.0, y: 1.0, z: 1.0};
        let reflected = r_in.direction.reflect_on(&normal);
        let facing = r_in.direction.dot(&normal) > 0.0;

        let ni_over_nt = if facing { self.ior } else { 1.0 / self.ior };
        let outward_normal = if facing { normal * -1.0 } else { normal };
        let cosine = if facing {
            self.ior * r_in.direction.dot(&normal) / r_in.direction.length()
        } else {
            -r_in.direction.dot(&normal) / r_in.direction.length()
        };

        let refraction = r_in.direction.refract(&outward_normal, ni_over_nt);

        let reflect_probability = if refraction.is_some() {
            Dielectric::schlick(cosine, self.ior)
        } else {
            1.0
        };

        if thread_rng().gen_range::<f32>(0.0, 1.0) < reflect_probability {
            return (true, Ray {origin:pos, direction:reflected}, attenuation);
        }
        return (true, Ray {origin:pos, direction:refraction.expect("some kind of dielectric probability error")}, attenuation);
    }
}