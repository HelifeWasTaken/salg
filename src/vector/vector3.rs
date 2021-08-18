use std::ops::{
    Add, AddAssign,
    Sub, SubAssign,
    Mul, MulAssign,
    Div, DivAssign,
    Rem, RemAssign,
    Neg
};
use std::cmp::{PartialEq};

#[derive(Clone, Copy, Debug)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

impl std::fmt::Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Vec3(x: {:.2}, y: {:.2}, z: {:.2})",
            self.x,
            self.y,
            self.z
        )
    }
}

impl PartialEq<Vec3> for Vec3 {
    fn eq(&self, other: &Vec3) -> bool {
        self.x == other.x &&
        self.y == other.y &&
        self.z == other.z
    }
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x: x, y: y, z: z }
    }

    pub fn copy(&self) -> Vec3 {
        Vec3::new(self.x, self.y, self.z)
    }

    pub fn dot(&self, v: &Vec3) -> f64 {
        *self * *v
    }

    pub fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn norm(&self) -> f64 {
        self.magnitude()
    }

    pub fn get_normalize(&self) -> Vec3 {
        let magnitude = self.magnitude();
        return if magnitude > 0.0 {
            *self * (1.0 / magnitude)
        } else {
            self.copy()
        }
    }

    pub fn normalize(&mut self) {
        let magnitude = self.magnitude();
        if magnitude > 0.0 {
            *self = *self * (1.0 / magnitude)
        }
    }

    pub fn cross(&self, v: &Vec3) -> Vec3 {
        *self % *v
    }

    pub fn perpendicular(&self, v: &Vec3) -> Vec3 {
        *self * self.dot(v)
    }
}

impl Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, v: Vec3) -> Vec3 {
        Vec3::new(
            self.x + v.x,
            self.y + v.y,
            self.z + v.z
        )
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, v: Vec3) {
        self.x += v.x;
        self.y += v.y;
        self.z += v.z;
    }
}

impl Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, v: Vec3) -> Vec3 {
        Vec3::new(
            self.x - v.x,
            self.y - v.y,
            self.z - v.z
        )
    }
}

impl SubAssign<Vec3> for Vec3 {
    fn sub_assign(&mut self, v: Vec3) {
        self.x -= v.x;
        self.y -= v.y;
        self.z -= v.z;
    }
}

// Scalar multiplication or division
// Multiply a |v| . s (or divide)
// where v is a vector and s a real number (a scalar)
// Relation is |r| = |v| . s or |r| = |v| / s
// Returns a Vector
// Do not mismatch with vector multiplication it returns a scalar
impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, s: f64) -> Vec3 {
        Vec3::new(
            self.x * s,
            self.y * s,
            self.z * s
        )
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, s: f64) {
        self.x *= s;
        self.y *= s;
        self.z *= s;
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, s: f64) -> Vec3 {
        Vec3::new(
            self.x / s,
            self.y / s,
            self.z / s
        )
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, s: f64) {
        self.x /= s;
        self.y /= s;
        self.z /= s;
    }
}

// Vector multiplication or division
// Multiply or (divide) a |v| . |v2| where v is a vector
// Returns a scalar
// Relation is s = |v| . |v2| or s = |v| / |v2|
// Do not mismatch with s * |v| multiplication it returns a vector

impl Mul<Vec3> for Vec3 {
    type Output = f64;

    fn mul(self, v: Vec3) -> f64 {
        self.x * v.x + self.y * v.y + self.z * v.z
    }
}

// MulAssign is not implemented
// Because you must return a scalar from |v| * |v2|
// -> You can't assign the result of a vector multiplication to a Vec3
// Duh math are weird
/*
impl MulAssign<Vec3> for Vec3 {
    fn mul_assign(&mut self, v: Vec3) {
    }
}
*/

// Modulo will be used to implemement the cross product
/*
It means that for a vector v1 and v2
where u = [1, 2, 3] and v = [4, 5, 6]
the result of u % v is [
    uy*vz-uz*vy,
    uz*vx-ux*vz,
    ux*vy-uy*vx
]
so u % v = [
    5*6-3*4,
    3*6-1*4,
    1*6-5*4
]
*/

impl Rem<Vec3> for Vec3 {
    type Output = Vec3;

    fn rem(self, v: Vec3) -> Vec3 {
        Vec3::new(
            self.y * v.z - self.z * v.y,
            self.z * v.x - self.x * v.z,
            self.x * v.y - self.y * v.x
        )
    }
}

impl RemAssign<Vec3> for Vec3 {
    fn rem_assign(&mut self, v: Vec3) {
        *self = *self % v
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3::new(
            -self.x,
            -self.y,
            -self.z
        )
    }
}

#[cfg(test)]
mod test {
    use super::Vec3;

    #[test]
    fn create_basic_vec3() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(v.x, 1.0);
        assert_eq!(v.y, 2.0);
        assert_eq!(v.z, 3.0);
    }

    #[test]
    fn test_eq() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(v1, v2);
    }

    #[test]
    fn test_neq() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(1.0, 2.0, 4.0);
        assert_ne!(v1, v2);
    }

    #[test]
    fn test_copy() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = v1.copy();
        assert_eq!(v1, v2);
    }

    #[test]
    fn test_add() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(4.0, 5.0, 6.0);
        let v3 = v1 + v2;
        assert_eq!(v3, Vec3::new(5.0, 7.0, 9.0));
    }

    #[test]
    fn test_add_assign() {
        let mut v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(4.0, 5.0, 6.0);
        v1 += v2;
        assert_eq!(v1, Vec3::new(5.0, 7.0, 9.0));
    }

    #[test]
    fn test_sub() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(4.0, 5.0, 6.0);
        let v3 = v1 - v2;
        assert_eq!(v3, Vec3::new(-3.0, -3.0, -3.0));
    }

    #[test]
    fn test_sub_assign() {
        let mut v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(4.0, 5.0, 6.0);
        v1 -= v2;
        assert_eq!(v1, Vec3::new(-3.0, -3.0, -3.0));
    }

    #[test]
    fn test_mul_scalar() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = v1 * 2.0;
        assert_eq!(v2, Vec3::new(2.0, 4.0, 6.0));
    }

    #[test]
    fn test_mul_scalar_assign() {
        let mut v1 = Vec3::new(1.0, 2.0, 3.0);
        v1 *= 2.0;
        assert_eq!(v1, Vec3::new(2.0, 4.0, 6.0));
    }

    #[test]
    fn test_mul_vec3() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(4.0, 5.0, 6.0);
        let s = v1 * v2;
        let r = v1.x * v2.x + v1.y * v2.y + v1.z * v2.z;
        assert_eq!(s, r);
    }

    /*
    Not implemented
    #[test]
    fn test_mul_vec3_assign() {
    }
    */

    #[test]
    fn test_div_scalar() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = v1 / 2.0;
        assert_eq!(v2, Vec3::new(0.5, 1.0, 1.5));
    }

    #[test]
    fn test_div_scalar_assign() {
        let mut v1 = Vec3::new(1.0, 2.0, 3.0);
        v1 /= 2.0;
        assert_eq!(v1, Vec3::new(0.5, 1.0, 1.5));
    }

    // Note: Division with vec3 is not implemented
    /*
    #[test]
    fn test_div_vec3() {
    }

    #[test]
    fn test_div_vec3_assign() {
    }
    */

    #[test]
    fn test_neg() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = -v1;
        assert_eq!(v2, Vec3::new(-1.0, -2.0, -3.0));
    }

    #[test]
    fn test_dot() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(4.0, 5.0, 6.0);
        let r = v1.dot(&v2);
        assert_eq!(r, v1.x * v2.x + v1.y * v2.y + v1.z * v2.z);
    }

    #[test]
    fn test_cross() {
        let v1 = Vec3::new(4.24, 242.21, 12.);
        let v2 = Vec3::new(1.1422, 124., 0.52);
        let r = v1.cross(&v2);
        assert_approx_eq::assert_approx_eq!(r.x, -1362.0508, 0.0001);
        assert_approx_eq::assert_approx_eq!(r.y, 11.5016, 0.0001);
        assert_approx_eq::assert_approx_eq!(r.y, 11.5016, 0.0001);
        assert_approx_eq::assert_approx_eq!(r.z, 249.107738, 0.00001);
    }

    #[test]
    fn test_norm() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let r = v1.norm();
        let v: f64 = 1. + 2. * 2. + 3. * 3.;
        assert_eq!(r, v.sqrt());
    }

    #[test]
    fn test_normalize() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let r = v1.get_normalize();
        assert_approx_eq::assert_approx_eq!(r.x, 0.26726124, 0.000001);
        assert_approx_eq::assert_approx_eq!(r.y, 0.53452248, 0.000001);
        assert_approx_eq::assert_approx_eq!(r.z, 0.8017837, 0.000001);
    }

    #[test]
    fn test_normalize_eq_get_normalize() {
        let mut v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = v1.copy();
        v1.normalize();
        assert_eq!(v1, v2.get_normalize());
    }

}