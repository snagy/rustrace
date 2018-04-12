use snmath::Ray;
use snmath::Vector3;


pub trait Material {
    fn scatter(&self, r_in: &Ray, pos: Vector3, normal: Vector3) -> (bool, Ray, Vector3);
}

#[derive(Clone,Copy,PartialEq,Default,Debug)]
pub struct Lambertian {
    pub albedo: Vector3,
}

impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, pos: Vector3, normal: Vector3) -> (bool, Ray, Vector3) {
        let target = pos + normal + Vector3::generate_random_unit_vector();
        let scattered = Ray{origin:pos, direction:target-pos};
        let attenuation = self.albedo;
        (true,scattered,attenuation)
    }
}


#[derive(Clone,Copy,PartialEq,Default,Debug)]
pub struct Metallic {
    pub albedo: Vector3,
}

impl Material for Metallic {
    fn scatter(&self, r_in: &Ray, pos: Vector3, normal: Vector3) -> (bool, Ray, Vector3) {
        let reflected_vec = r_in.direction.normalize().reflect_on(&normal);
        let scattered_ray = Ray{origin:pos, direction:reflected_vec};
        let attenuation = self.albedo;
        (true, scattered_ray, attenuation)
    }
}