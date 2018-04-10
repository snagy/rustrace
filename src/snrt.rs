use snmath::Vector3;
use snmath::Ray;

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

#[derive(Clone,Copy,PartialEq,Default,Debug)]
pub struct HitResult {
    pub t: f64,
    pub pos: Vector3,
    pub normal: Vector3,
}

pub trait Hitable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> (bool, HitResult);
}

#[derive(Clone,Copy,PartialEq,Default,Debug)]
pub struct Sphere {
    pub pos: Vector3,
    pub radius: f64,
}

impl Hitable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> (bool, HitResult) {
        let oc = r.origin - self.pos;

        let a = r.direction.dot(&r.direction);
        let b = oc.dot(&r.direction);
        let c = oc.dot(&oc) - self.radius*self.radius;

        let discriminant = b*b - a*c;

        if discriminant < 0.0 {
            return (false, HitResult::default());
        }
        
        let get_result = |t| {
            let hit_pos = r.point_at_parameter(t);
            let hit_normal = (hit_pos - self.pos).normalize();
            HitResult {t:t, pos:hit_pos, normal:hit_normal}
        };

        let t = (-b - discriminant.sqrt()) / a ; 
        if t > t_min && t < t_max {
            return (true, get_result(t));
        }

        let t = (-b + discriminant.sqrt()) / a ; 
        if t > t_min && t < t_max {
            return (true, get_result(t));
        }

        // todo: check other t;
        return (false, HitResult::default());
    }
}