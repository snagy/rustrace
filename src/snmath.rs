use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
use std::ops::Div;

#[derive(Clone,Copy,PartialEq,Default,Debug)]
pub struct Vector3 { 
    pub x: f64,
    pub y: f64,
    pub z: f64,
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

impl<'a> Mul<f64> for &'a Vector3 {
    type Output = Vector3;

    fn mul(self, other: f64) -> Vector3 {
        Vector3 {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other
        }
    }
}

impl Mul<f64> for Vector3 {
    type Output = Vector3;

    fn mul(self, other: f64) -> Vector3 {
        Vector3 {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other
        }
    }
}

impl Div<f64> for Vector3 {
    type Output = Vector3;

    fn div(self, other: f64) -> Vector3 {
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
    pub fn dot(&self, other: &Vector3) -> f64 {
        self.x*other.x + self.y*other.y + self.z*other.z
    }
    pub fn cross(&self, other: &Vector3) -> Vector3 {
        Vector3 {
            x: self.y*other.z - self.z*other.y,
            y: self.z*other.x - self.x*other.z,
            z: self.x*other.y - self.y*other.x,
        }
    }
    pub fn length(&self) -> f64 {
        self.length_sq().sqrt()
    }
    pub fn length_sq(&self) -> f64 {
        self.x*self.x + self.y*self.y + self.z*self.z
    }
    pub fn normalize(self) -> Vector3 {
        // todo:  implement scalar multiplication trait
        self / self.length()
    }

    pub fn lerp(a: &Vector3, b: &Vector3, t: f64) -> Vector3 {
        a*(1.0-t)+b*t
    }
}

#[derive(Clone,Copy,PartialEq,Default,Debug)]
pub struct Ray {
    pub origin: Vector3,
    pub direction: Vector3,
}

impl Ray {
    pub fn point_at_parameter(&self, t: f64) -> Vector3 {
        self.origin + self.direction * t
    }
}
