// Most of the file will be focused on the Vec3 class.
// Quaternions can be added and subtracted.
// Quaternions can be multiplied
// Quaternions cannot be divided
use std::ops::{
    Add, AddAssign,
    Sub, SubAssign,
    Mul, MulAssign
};
use std::cmp::{PartialEq};
use super::vector3::Vec3;

#[derive(Clone, Copy, Debug)]
pub struct Quaternion {
    pub v: Vec3,
    pub s: f64
}

impl PartialEq for Quaternion {
    fn eq(&self, other: &Quaternion) -> bool {
        self.v == other.v && self.s == other.s
    }
}

impl Quaternion {
    pub fn new(v: Vec3, s: f64) -> Quaternion {
        Quaternion { v: v, s: s }
    }

    pub fn copy(&self) -> Quaternion {
        Quaternion { v: self.v, s: self.s }
    }

    pub fn norm(&self) -> f64 {
        (self.s * self.s + self.v * self.v).sqrt()
    }

    pub fn normalize(&mut self) {
        let norm = self.norm();
        if norm > 0.0 {
            let nv = 1.0 / norm;
            self.v *= nv;
            self.s *= nv;
        }
    }

    pub fn get_normliaze(&mut self) -> Quaternion {
        let norm = self.norm();
        return if norm > 0.0 {
            let nv = 1.0 / norm;
            Quaternion {
                v: self.v * nv,
                s: self.s * nv
            }
        } else {
            self.copy()
        }
    }

    pub fn convert_to_unit_norm(&mut self) {
        let angle = self.s * std::f64::consts::PI / 180.0;
        self.normalize();
        self.s = (angle * 0.5).cos();
        self.v = self.v * (angle * 0.5).sin();
    }

    pub fn conjugate(&self) -> Quaternion {
        Quaternion {
            v: -self.v,
            s: self.s
        }
    }

    pub fn inverse(&self) -> Quaternion {
        let norm = self.norm();
        let conj = self.conjugate();
        Quaternion {
            v: conj.v * (1.0 / (norm * norm)),
            s: conj.s * norm
        }
    }

    pub fn rotate(&self, rhs: &Quaternion) -> Vec3 {
        let mut q = rhs.copy();

        q.v.normalize();
        q.convert_to_unit_norm();
        return (q * *self * q.inverse()).v;
    }
}

impl Add<Quaternion> for Quaternion {
    type Output = Quaternion;

    fn add(self, other: Quaternion) -> Quaternion {
        Quaternion {
            v: self.v + other.v,
            s: self.s + other.s
        }
    }
}

impl AddAssign<Quaternion> for Quaternion {
    fn add_assign(&mut self, other: Quaternion) {
        self.v += other.v;
        self.s += other.s;
    }
}

impl Sub<Quaternion> for Quaternion {
    type Output = Quaternion;

    fn sub(self, other: Quaternion) -> Quaternion {
        Quaternion {
            v: self.v - other.v,
            s: self.s - other.s
        }
    }
}

impl SubAssign<Quaternion> for Quaternion {
    fn sub_assign(&mut self, other: Quaternion) {
        self.v -= other.v;
        self.s -= other.s;
    }
}

// Implemantion of Quaternions multiplication
impl Mul<Quaternion> for Quaternion {
    type Output = Quaternion;

    fn mul(self, other: Quaternion) -> Quaternion {
        Quaternion {
            v: other.v * self.s + self.v * other.s + self.v.cross(&other.v),
            s: self.s * other.s - self.v.dot(&other.v)
        }
    }
}

impl MulAssign<Quaternion> for Quaternion {
    fn mul_assign(&mut self, other: Quaternion) {
        *self = *self * other;
    }
}

// Implementation of scalar multiplication
impl Mul<f64> for Quaternion {
    type Output = Quaternion;

    fn mul(self, other: f64) -> Quaternion {
        Quaternion {
            v: self.v * other,
            s: self.s * other
        }
    }
}

impl MulAssign<f64> for Quaternion {
    fn mul_assign(&mut self, other: f64) {
        self.v *= other;
        self.s *= other;
    }
}
