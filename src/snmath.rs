use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
use std::ops::Div;

extern crate rand;
use rand::{thread_rng, Rng};

#[derive(Clone,Copy,PartialEq,Default,Debug)]
pub struct Vector3 { 
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Add for Vector3 {
    type Output = Vector3;

    fn add(self, other: Vector3) -> Vector3 {
        Vector3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl<'a> Sub<&'a Vector3> for &'a Vector3 {
    type Output = Vector3;

    fn sub(self, other: &Vector3) -> Vector3 {
        Vector3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl<'a> Sub<&'a Vector3> for Vector3 {
    type Output = Vector3;

    fn sub(self, other: &Vector3) -> Vector3 {
        Vector3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl<'a> Sub<Vector3> for &'a Vector3 {
    type Output = Vector3;

    fn sub(self, other: Vector3) -> Vector3 {
        Vector3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Sub for Vector3 {
    type Output = Vector3;

    fn sub(self, other: Vector3) -> Vector3 {
        Vector3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl<'a> Mul<f32> for &'a Vector3 {
    type Output = Vector3;

    fn mul(self, other: f32) -> Vector3 {
        Vector3 {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other
        }
    }
}

impl Mul<f32> for Vector3 {
    type Output = Vector3;

    fn mul(self, other: f32) -> Vector3 {
        Vector3 {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other
        }
    }
}


impl Mul<Vector3> for f32 {
    type Output = Vector3;

    fn mul(self, other: Vector3) -> Vector3 {
        Vector3 {
            x: self * other.x,
            y: self * other.y,
            z: self * other.z
        }
    }
}

impl<'a> Mul<&'a Vector3> for f32 {
    type Output = Vector3;

    fn mul(self, other: &Vector3) -> Vector3 {
        Vector3 {
            x: self * other.x,
            y: self * other.y,
            z: self * other.z
        }
    }
}

impl Div<f32> for Vector3 {
    type Output = Vector3;

    fn div(self, other: f32) -> Vector3 {
        let inv = 1.0 / other;
        Vector3 {
            x: self.x * inv,
            y: self.y * inv,
            z: self.z * inv
        }
    }
}

impl<'a> Div<f32> for &'a Vector3 {
    type Output = Vector3;

    fn div(self, other: f32) -> Vector3 {
        let inv = 1.0 / other;
        Vector3 {
            x: self.x * inv,
            y: self.y * inv,
            z: self.z * inv
        }
    }
}

// don't like having these
impl Mul<Vector3> for Vector3 {
    type Output = Vector3;

    fn mul(self, other: Vector3) -> Vector3 {
        Vector3 {
            x: self.x*other.x,
            y: self.y*other.y,
            z: self.z*other.z,
        }
    }
}

impl Div<Vector3> for Vector3 {
    type Output = Vector3;

    fn div(self, other: Vector3) -> Vector3 {
        Vector3 {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
        }
    }
}

impl Vector3 {
    pub fn powf(&self, p: f32) -> Vector3 {
        Vector3 {
            x: self.x.powf(p),
            y: self.y.powf(p),
            z: self.z.powf(p),
        }
    }
    pub fn dot(&self, other: &Vector3) -> f32 {
        self.x*other.x + self.y*other.y + self.z*other.z
    }
    pub fn cross(&self, other: &Vector3) -> Vector3 {
        Vector3 {
            x: self.y*other.z - self.z*other.y,
            y: self.z*other.x - self.x*other.z,
            z: self.x*other.y - self.y*other.x,
        }
    }
    pub fn length(&self) -> f32 {
        self.length_sq().sqrt()
    }
    pub fn length_sq(&self) -> f32 {
        self.x*self.x + self.y*self.y + self.z*self.z
    }
    pub fn normalize(&self) -> Vector3 {
        self / self.length()
    }

    pub fn lerp(a: &Vector3, b: &Vector3, t: f32) -> Vector3 {
        a*(1.0-t)+b*t
    }

    pub fn generate_random_unit_vector() -> Vector3 {
        let mut rng = thread_rng();
        let mut p = Vector3 {x:100.0,y:0.0,z:0.0};
        let ones = Vector3 {x:1.0, y:1.0, z:1.0};
        while p.length_sq() > 1.0 {
            p = Vector3{x:rng.gen_range::<f32>(0.0,1.0),y:rng.gen_range::<f32>(0.0,1.0),z:rng.gen_range::<f32>(0.0,1.0)}*2.0 - ones;
        }
        return p;
    }

    pub fn generate_random_unit_disc() -> Vector3 {
        let mut rng = thread_rng();
        let mut p = Vector3 {x:100.0,y:0.0,z:0.0};
        let ones = Vector3 {x:1.0, y:1.0, z:0.0};
        while p.length_sq() > 1.0 {
            p = Vector3{x:rng.gen_range::<f32>(0.0,1.0),y:rng.gen_range::<f32>(0.0,1.0),z:0.0} - ones;
        }
        return p;
    }

    pub fn reflect_on(&self, normal: &Vector3) -> Vector3 {
        self - 2.0*self.dot(normal)*normal
    }

    pub fn refract(&self, normal: &Vector3, ni_over_nt: f32) -> Option<Vector3> {
        let uv = self.normalize();
        let dt = uv.dot(normal);
        let discriminant = 1.0 - ni_over_nt*ni_over_nt*(1.0-dt*dt);
        if discriminant > 0.0 {
            return Some(ni_over_nt*(uv - normal*dt) - normal*discriminant.sqrt());
        }
        return None;
    }
}

#[derive(Clone,Copy,PartialEq,Default,Debug)]
pub struct Ray {
    pub origin: Vector3,
    pub direction: Vector3,
}

impl Ray {
    pub fn point_at_parameter(&self, t: f32) -> Vector3 {
        self.origin + self.direction * t
    }
}
