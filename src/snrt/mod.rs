use std::boxed::Box;

use snmath::Vector3;
use snmath::Ray;

pub mod material;

pub struct Camera {
    pub lower_left_corner: Vector3,
    pub horizontal: Vector3,
    pub vertical: Vector3,
    pub origin: Vector3,
}

impl Camera {
    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray {origin:self.origin, direction:self.lower_left_corner + self.horizontal * u + self.vertical * v}
    }
}


pub trait Hitable {
    fn hit_process(&self, r: &Ray, t: f64) -> (bool, Ray, Vector3);
    fn hit_check(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<f64>;
}

pub struct Sphere {
    pub pos: Vector3,
    pub radius: f64,
    pub material: Box<material::Material>,
}

impl Hitable for Sphere {
    fn hit_process(&self, r: &Ray, t: f64) -> (bool, Ray, Vector3) {
        let hit_pos = r.point_at_parameter(t);
        let hit_normal = (hit_pos - self.pos).normalize();

        self.material.scatter(r, hit_pos, hit_normal)
    }

    fn hit_check(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<f64> {
        let oc = r.origin - self.pos;

        let a = r.direction.dot(&r.direction);
        let b = oc.dot(&r.direction);
        let c = oc.dot(&oc) - self.radius*self.radius;

        let discriminant = b*b - a*c;

        if discriminant < 0.0 {
            return None;
        }

        let t = (-b - discriminant.sqrt()) / a ; 
        if t > t_min && t < t_max {
            return Some(t);
        }

        let t = (-b + discriminant.sqrt()) / a ; 
        if t > t_min && t < t_max {
            return Some(t);
        }

        return None;
    }
}