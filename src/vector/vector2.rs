use std::ops::{
    Add, AddAssign,
    Sub, SubAssign,
    Mul, MulAssign,
    Div, DivAssign,
    Neg
};
use std::cmp::{PartialEq};

#[derive(Debug, Clone, Copy)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64
}

impl std::fmt::Display for Vec2 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Vec2(x: {:.2}, y: {:.2})", self.x, self.y)
    }
}

impl Vec2 {
    pub fn new(x: f64, y: f64) -> Vec2 {
        Vec2 { x: x, y: y }
    }

    pub fn copy(&self) -> Vec2 {
        Vec2::new(self.x, self.y)
    }

    pub fn scalar(&self, v: &Vec2) -> f64 {
        *self * *v
    }

    pub fn dot(&self, v: &Vec2) -> f64 {
        *self * *v
    }

    pub fn magnitude(&self, v: &Vec2) -> f64 {
        (self.x * v.y) - (self.y * v.x)
    }

    pub fn perpendicular(&self) -> Vec2 {
        Vec2::new(self.y, -self.x)
    }
}

impl Add<Vec2> for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Vec2) -> Vec2 {
        Vec2::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl AddAssign<Vec2> for Vec2 {
    fn add_assign(&mut self, rhs: Vec2) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub<Vec2> for Vec2 {
    type Output = Vec2;

    fn sub(self, rhs: Vec2) -> Vec2 {
        Vec2::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl SubAssign<Vec2> for Vec2 {
    fn sub_assign(&mut self, rhs: Vec2) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

// Scalar multiplication or division
// Multiply a |v| . s (or divide)
// where v is a vector and s a real number (a scalar)
// Relation is |r| = |v| . s or |r| = |v| / s
// Returns a Vector
// Do not mismatch with vector multiplication it returns a scalar
impl Mul<f64> for Vec2 {
    type Output = Vec2;

    fn mul(self, rhs: f64) -> Vec2 {
        Vec2::new(self.x * rhs, self.y * rhs)
    }
}

impl MulAssign<f64> for Vec2 {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl Div<f64> for Vec2 {
    type Output = Vec2;

    fn div(self, rhs: f64) -> Vec2 {
        Vec2::new(self.x / rhs, self.y / rhs)
    }
}

impl DivAssign<f64> for Vec2 {
    fn div_assign(&mut self, rhs: f64) {
        self.x /= rhs;
        self.y /= rhs;
    }
}

// Vector multiplication or division
// Multiply or (divide) a |v| . |v2| where v is a vector
// Returns a scalar
// Relation is s = |v| . |v2| or s = |v| / |v2|
// Do not mismatch with s * |v| multiplication it returns a vector
impl Mul<Vec2> for Vec2 {
    type Output = f64;

    fn mul(self, rhs: Vec2) -> f64 {
        self.x * rhs.x + self.y * rhs.y
    }
}

/*
 * impl MulAssign<Vec2> for Vec2
 * Cannot exists because I can't assign the float to Vec
 * It could be done but it would be pretty error prone
 */

// Divisions bewteen vectors is not really necesary
// But here it is for completeness
// Divisions bewteen vector should nor be done
impl Div<Vec2> for Vec2 {
    type Output = Vec2;

    fn div(self, rhs: Vec2) -> Vec2 {
        Vec2::new(self.x / rhs.x, self.y / rhs.y)
    }
}

impl DivAssign<Vec2> for Vec2 {
    fn div_assign(&mut self, rhs: Vec2) {
        self.x /= rhs.x;
        self.y /= rhs.y;
    }
}

/*
 * There is no real implementation of a cross product with
 * Vector on 2 Dimensions
 */
impl Neg for Vec2 {
    type Output = Vec2;

    fn neg(self) -> Vec2 {
        Vec2::new(-self.x, -self.y)
    }
}

impl PartialEq for Vec2 {
    fn eq(&self, rhs: &Vec2) -> bool {
        self.x == rhs.x && self.y == rhs.y
    }
}

#[cfg(test)]
mod tests {
    use super::Vec2;

    #[test]
    fn basic_new() {
        let v = Vec2::new(1.0, 2.0);
        assert_eq!(v.x, 1.0);
        assert_eq!(v.y, 2.0);
    }

    #[test]
    fn test_eq() {
        let v = Vec2::new(1.0, 2.0);
        let v2 = Vec2::new(1.0, 2.0);
        assert_eq!(v, v2);
    }

    #[test]
    fn test_neq() {
        let v = Vec2::new(1.0, 2.0);
        let v2 = Vec2::new(1.0, 3.0);
        assert_ne!(v, v2);
    }

    #[test]
    fn test_copy() {
        let v = Vec2::new(1.0, 2.0);
        let v2 = v.copy();
        assert_eq!(v, v2);
    }

    #[test]
    fn test_add() {
        let v = Vec2::new(1.0, 2.0);
        let v2 = Vec2::new(3.0, 4.0);
        let v3 = v + v2;
        assert_eq!(v3, Vec2::new(4.0, 6.0));
    }

    #[test]
    fn test_add_assign() {
        let mut v = Vec2::new(1.0, 2.0);
        let v2 = Vec2::new(3.0, 4.0);
        v += v2;
        assert_eq!(v, Vec2::new(4.0, 6.0));
    }

    #[test]
    fn test_sub() {
        let v = Vec2::new(1.0, 2.0);
        let v2 = Vec2::new(3.0, 4.0);
        let v3 = v - v2;
        assert_eq!(v3, Vec2::new(-2.0, -2.0));
    }

    #[test]
    fn test_sub_assign() {
        let mut v = Vec2::new(1.0, 2.0);
        let v2 = Vec2::new(3.0, 4.0);
        v -= v2;
        assert_eq!(v, Vec2::new(-2.0, -2.0));
    }

    // Remember that the multiplication between two vector result in a scalar
    #[test]
    fn test_mul_two_vec() {
        let v = Vec2::new(1.0, 2.0);
        let v2 = Vec2::new(3.0, 4.0);
        let s = v * v2;
        assert_eq!(s, 1.0 * 3.0 + 2.0 * 4.0);
    }

    // Mul assign two vector is not possible because it returns a scalar
    // #[test]
    // fn test_mul_assign_two_vec() {
    //
    // }

    // Multiply a vector with a scalar result in a vector
    #[test]
    fn test_mul_scalar() {
        let v = Vec2::new(1.0, 2.0);
        let s = v * 3.0;
        assert_eq!(s, Vec2::new(3.0, 6.0));
    }

    #[test]
    fn test_mul_assign_scalar() {
        let mut v = Vec2::new(1.0, 2.0);
        v *= 3.0;
        assert_eq!(v, Vec2::new(3.0, 6.0));
    }

    #[test]
    fn test_div_scalar() {
        let v = Vec2::new(1.0, 2.0);
        let s = v / 3.0;
        assert_eq!(s, Vec2::new(1.0 / 3.0, 2.0 / 3.0));
    }

    #[test]
    fn test_div_assign_scalar() {
        let mut v = Vec2::new(1.0, 2.0);
        v /= 3.0;
        assert_eq!(v, Vec2::new(1.0 / 3.0, 2.0 / 3.0));
    }

    #[test]
    fn test_div_two_vec() {
        let v = Vec2::new(1.0, 2.0);
        let v2 = Vec2::new(3.0, 4.0);
        let s = v / v2;
        assert_eq!(s, Vec2::new(1.0 / 3.0, 2.0 / 4.0));
    }

    #[test]
    fn test_div_assign_two_vec() {
        let mut v = Vec2::new(1.0, 2.0);
        let v2 = Vec2::new(3.0, 4.0);
        v /= v2;
        assert_eq!(v, Vec2::new(1.0 / 3.0, 2.0 / 4.0));
    }

    #[test]
    fn test_neg() {
        let v = Vec2::new(1.0, 2.0);
        let v2 = -v;
        assert_eq!(v2, Vec2::new(-1.0, -2.0));
    }

    #[test]
    fn test_fmt() {
        let v = Vec2::new(1.252, 2.2);
        assert_eq!("Vec2(x: 1.25, y: 2.20)", format!("{}", v));
    }
}