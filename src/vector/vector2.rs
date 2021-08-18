use std::ops::{
    Add,       Sub,       Mul,       Div,       Neg,
    AddAssign, SubAssign, MulAssign, DivAssign
};

#[derive(Debug, Clone, Copy)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64
}

impl std::fmt::Display for Vec2 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Vec2(x: {}, y: {})", self.x, self.y)
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
// Do not mismatch with vector multiplication it returns a scalar
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
