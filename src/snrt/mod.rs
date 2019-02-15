use std::boxed::Box;
use std::f32;

use snmath::Vector3;
use snmath::Ray;

pub mod material;
pub mod world;

#[derive(Clone,Copy,PartialEq,Default,Debug)]
pub struct Camera {
    pub origin: Vector3,
    pub lower_left_corner: Vector3,
    pub horizontal: Vector3,
    pub vertical: Vector3,
    u: Vector3,
    v: Vector3,
    w: Vector3,
    lens_radius: f32,
}

impl Camera {
    pub fn create_camera(look_from: Vector3, look_at: Vector3, v_up: Vector3, v_fov: f32, aspect_ratio: f32, aperture: f32, focal_dist: f32) -> Camera {
        let lens_radius = aperture / 2.0;
        let theta = v_fov * f32::consts::PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect_ratio * half_height;
        
        let w = (look_from-look_at).normalize();
        let u = v_up.cross(&w).normalize();
        let v = w.cross(&u);

        Camera {
            lower_left_corner: -half_width*focal_dist*u-half_height*focal_dist*v-focal_dist*w,
            horizontal: 2.0*half_width*focal_dist*u,
            vertical: 2.0*half_height*focal_dist*v,
            origin: look_from,
            u: u, v: v, w: w,
            lens_radius: lens_radius
        }
    }

    pub fn get_ray(&self, s: f32, t: f32) -> Ray {
        let rd = self.lens_radius * Vector3::generate_random_unit_disc();
        let jitter = self.u * rd.x + self.v * rd.y;
        Ray {origin:self.origin+jitter, direction:self.lower_left_corner + self.horizontal * s + self.vertical * t - jitter}
    }
}


pub trait Hitable {
    fn hit_process(&self, r: &Ray, t: f32) -> (bool, Ray, Vector3);
    fn hit_check(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<f32>;
}

pub struct Sphere {
    pub pos: Vector3,
    pub radius: f32,
    pub material: Box<material::Material + Sync>,
}

impl Hitable for Sphere {
    fn hit_process(&self, r: &Ray, t: f32) -> (bool, Ray, Vector3) {
        let hit_pos = r.point_at_parameter(t);
        let hit_normal = (hit_pos - self.pos).normalize();

        self.material.scatter(r, hit_pos, hit_normal)
    }

    fn hit_check(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<f32> {
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


pub struct AABox {
    pub pos: Vector3,
    pub dims: Vector3,
    pub material: Box<material::Material + Sync>,
}


impl Hitable for AABox {
    fn hit_process(&self, r: &Ray, t: f32) -> (bool, Ray, Vector3) {
        let hit_pos = r.point_at_parameter(t);
        let offset = (hit_pos - self.pos)/self.dims;

        let hit_normal = if offset.x.abs() > offset.y.abs() || offset.z.abs() > offset.y.abs() {
            if offset.z.abs() > offset.x.abs() {
                Vector3 {x:0.0, y:0.0, z:if offset.z > 0.0 { 1.0 } else { -1.0 } }
            }
            else {
                Vector3 {x:if offset.x > 0.0 {1.0} else {-1.0}, y:0.0, z:0.0}
            }
        }
        else {
            Vector3 {x:0.0, y:if offset.y > 0.0 {1.0} else {-1.0}, z:0.0}
        };

        self.material.scatter(r, hit_pos, hit_normal)
    }

    fn hit_check(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<f32> {
        let mins = self.pos - self.dims;
        let maxs = self.pos + self.dims;

        let tmins = (mins - r.origin) / r.direction;
        let tmaxs = (maxs - r.origin) / r.direction;


        let xmin = tmins.x.min(tmaxs.x);
        let ymin = tmins.y.min(tmaxs.y);
        let zmin = tmins.z.min(tmaxs.z);

        let xmax = tmins.x.max(tmaxs.x);
        let ymax = tmins.y.max(tmaxs.y);
        let zmax = tmins.z.max(tmaxs.z);

        if xmax > ymin && xmin < ymax && zmax > xmin && zmin < xmax && ymax > zmin && ymin < zmax {
            let hit_t = xmin.max(ymin.max(zmin));
            if hit_t > t_min && hit_t < t_max {
                return Some(hit_t);
            }
        }

        return None;
    }
}
