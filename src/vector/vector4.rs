// Most of the code is considered as a copy of vector3.rs
// For safety purposes with the w component, it is not modified by operator
// Add two Vec4 is not implemented.
// But it is possible to add a Vec4 to a Vec3.
use std::ops::{
    Add, AddAssign,
    Sub, SubAssign,
    Mul, MulAssign,
    Div, DivAssign,
    Neg
};
use std::cmp::{PartialEq};
use super::vector3::Vec3;
use super::quaternions::Quaternion;

#[derive(Clone, Copy, Debug)]
pub struct Vec4 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64
}

impl std::fmt::Display for Vec4 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Vec4(x: {:.2}, y: {:.2}, z: {:.2}, w: {:.2})",
            self.x,
            self.y,
            self.z,
            self.w
        )
    }
}

impl PartialEq<Vec4> for Vec4 {
    fn eq(&self, other: &Vec4) -> bool {
        self.x == other.x &&
        self.y == other.y &&
        self.z == other.z &&
        self.w == other.w
    }
}

impl Vec4 {
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Vec4 {
        Vec4 {
            x: x,
            y: y,
            z: z,
            w: w
        }
    }

    pub fn to_vec3(&self) -> Vec3 {
        Vec3 {
            x: self.x / self.w,
            y: self.y / self.w,
            z: self.z / self.w
        }
    }

    pub fn to_pure_vec3(&self) -> Vec3 {
        Vec3 {
            x: self.x,
            y: self.y,
            z: self.z
        }
    }

    pub fn norm(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn normalize(&mut self) {
        let norm = self.norm();
        if norm > 0.0 {
            let mag = 1.0 / norm;
            self.x *= mag;
            self.y *= mag;
            self.z *= mag;
            self.w *= mag;
        }
    }

    pub fn get_normliaze(&self) -> Vec4 {
        let norm = self.norm();
        return if norm > 0.0 {
            let mag = 1.0 / norm;
            Vec4 {
                x: self.x * mag,
                y: self.y * mag,
                z: self.z * mag,
                w: self.w * mag
            }
        } else {
            self.copy()
        }
    }

    pub fn copy(&self) -> Vec4 {
        Vec4 {
            x: self.x,
            y: self.y,
            z: self.z,
            w: self.w
        }
    }

    pub fn convert_to_unit_norm(&mut self) {
        let angle = self.w * std::f64::consts::PI / 180.0;
        self.normalize();
        self.w = (angle * 0.5).cos();
        let v = self.to_pure_vec3() * (angle * 0.5).sin();
        self.x = v.x;
        self.y = v.y;
        self.z = v.z;
    }

    pub fn conjugate(&self) -> Vec4 {
        Vec4 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: self.w
        }
    }

    pub fn inverse(&self) -> Vec4 {
        let norm = self.norm();
        let conj = self.conjugate();
        let v3 = Vec3::new(conj.x, conj.y, conj.z) * (1.0 / (norm * norm));
        Vec4 {
            x: v3.x,
            y: v3.y,
            z: v3.z,
            w: conj.w * norm
        }
    }

    pub fn rotate(&self, rhs: &Vec4) -> Vec3 {
        Quaternion {
            v: Vec3::new(self.x, self.y, self.z),
            s: self.w
        }.rotate(&(Quaternion {
            v: Vec3::new(rhs.x, rhs.y, rhs.z),
            s: rhs.w}))
    }
}

impl Add<Vec4> for Vec4 {
    type Output = Vec4;

    fn add(self, other: Vec4) -> Vec4 {
        Vec4::new(
            self.x + other.x,
            self.y + other.y,
            self.z + other.z,
            self.w + other.w
        )
    }
}

impl AddAssign<Vec4> for Vec4 {
    fn add_assign(&mut self, other: Vec4) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
        self.w += other.w;
    }
}

impl Sub<Vec4> for Vec4 {
    type Output = Vec4;

    fn sub(self, other: Vec4) -> Vec4 {
        Vec4::new(
            self.x - other.x,
            self.y - other.y,
            self.z - other.z,
            self.w - other.w
        )
    }
}

impl SubAssign<Vec4> for Vec4 {
    fn sub_assign(&mut self, other: Vec4) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
        self.w -= other.w;
    }
}

/**
 * Cross product of two vectors with 4 dimensions does not exist.
 * It will be just a simple multiplication of a vector4 with vec3.
 * [x, y, z, w] * [u, v, t] = [xu, yu, zt, w]
 */
impl Mul<Vec4> for Vec4 {
    type Output = Vec4;

    fn mul(self, other: Vec4) -> Vec4 {
        Vec4::new(
            self.x * other.x,
            self.y * other.y,
            self.z * other.z,
            self.w * other.w
        )
    }
}

impl MulAssign<Vec4> for Vec4 {
    fn mul_assign(&mut self, other: Vec4) {
        self.x *= other.x;
        self.y *= other.y;
        self.z *= other.z;
        self.w *= other.w;
    }
}

impl Div<Vec4> for Vec4 {
    type Output = Vec4;

    fn div(self, other: Vec4) -> Vec4 {
        Vec4::new(
            self.x / other.x,
            self.y / other.y,
            self.z / other.z,
            self.w / other.w
        )
    }
}

impl DivAssign<Vec4> for Vec4 {
    fn div_assign(&mut self, other: Vec4) {
        self.x /= other.x;
        self.y /= other.y;
        self.z /= other.z;
        self.w /= other.w;
    }
}

// For safety purpose we do not touch to the w component
// The w component is used for the perspective transformation
// And it often implicte on most game Engine (like Unity)
impl Neg for Vec4 {
    type Output = Vec4;

    fn neg(self) -> Vec4 {
        Vec4::new(
            -self.x,
            -self.y,
            -self.z,
            -self.w
        )
    }
}

#[cfg(test)]
mod test {
    use super::Vec3;
    use super::Vec4;

    #[test]
    fn test_create_vec4() {
        let v = Vec4::new(1.0, 2.0, 3.0, 4.0);

        assert_eq!(v.x, 1.0);
        assert_eq!(v.y, 2.0);
        assert_eq!(v.z, 3.0);
        assert_eq!(v.w, 4.0);
    }

    #[test]
    fn test_eq() {
        let v1 = Vec4::new(1.0, 2.0, 3.0, 4.0);
        let v2 = Vec4::new(1.0, 2.0, 3.0, 4.0);

        assert_eq!(v1, v2);
    }

    #[test]
    fn test_ne() {
        let v1 = Vec4::new(1.0, 2.0, 3.0, 4.0);
        let v2 = Vec4::new(1.0, 2.0, 3.0, 5.0);

        assert_ne!(v1, v2);
    }

    #[test]
    fn test_add() {
        let v1 = Vec4::new(1.0, 2.0, 3.0, 4.0);
        let v2 = Vec4::new(1.0, 2.0, 3.0, 4.0);

        assert_eq!(v1 + v2, Vec4::new(2.0, 4.0, 6.0, 8.0));
    }

    #[test]
    fn test_add_assign() {
        let mut v1 = Vec4::new(1.0, 2.0, 3.0, 4.0);
        let v2 = Vec4::new(1.0, 2.0, 3.0, 4.0);

        v1 += v2;
        assert_eq!(v1, Vec4::new(2.0, 4.0, 6.0, 8.0));
    }

    #[test]
    fn test_sub() {
        let v1 = Vec4::new(1.0, 2.0, 3.0, 4.0);
        let v2 = Vec4::new(1.0, 2.0, 3.0, 4.0);
        assert_eq!(v1 - v2, Vec4::new(0.0, 0.0, 0.0, 0.0));
    }

    #[test]
    fn test_sub_assign() {
        let mut v1 = Vec4::new(1.0, 2.0, 3.0, 4.0);
        let v2 = Vec4::new(1.0, 2.0, 3.0, 2.0);
        v1 -= v2;
        assert_eq!(v1, Vec4::new(0.0, 0.0, 0.0, 2.0));
    }

    #[test]
    fn test_mul() {
        let v1 = Vec4::new(1.0, 2.0, 3.0, 4.0);
        let v2 = Vec4::new(1.0, 2.0, 3.0, 2.5);
        assert_eq!(v1 * v2, Vec4::new(1.0, 4.0, 9.0, 10.0));
    }

    #[test]
    fn test_mul_assign() {
        let mut v1 = Vec4::new(1.0, 2.0, 3.0, 4.0);
        let v2 = Vec4::new(1.0, 2.0, 3.0, 2.0);
        v1 *= v2;
        assert_eq!(v1, Vec4::new(1.0, 4.0, 9.0, 8.0));
    }

    #[test]
    fn test_div() {
        let v1 = Vec4::new(1.0, 2.0, 3.0, 4.0);
        let v2 = Vec4::new(12.0, 3.0, 55.0, 3.0);
        let v3 = v1/v2;

        assert_eq!(v3.x, 1.0/12.0);
        assert_eq!(v3.y, 2.0/3.0);
        assert_eq!(v3.z, 3.0/55.0);
        assert_eq!(v3.w, 4.0/3.0);
    }

    #[test]
    fn test_div_assign() {
        let mut v1 = Vec4::new(1.0, 2.0, 3.0, 4.0);
        let v2 = Vec4::new(12.0, 3.0, 55.0, 1.255);
        v1 /= v2;
        assert_eq!(v1.x, 1.0/12.0);
        assert_eq!(v1.y, 2.0/3.0);
        assert_eq!(v1.z, 3.0/55.0);
        assert_eq!(v1.w, 4.0/1.255);
    }

    #[test]
    fn test_neg() {
        let v1 = Vec4::new(1.0, 2.0, 3.0, 4.0);
        assert_eq!(-v1, Vec4::new(-1.0, -2.0, -3.0, -4.0));
    }

    #[test]
    fn test_to_vec3() {
        let v1 = Vec4::new(1.0, 2.0, 3.0, 4.0);
        let v2 = Vec3::new(1.0/4.0, 2.0/4.0, 3.0/4.0);
        assert_eq!(v1.to_vec3(), v2);
    }

    #[test]
    fn test_to_pure_vec3() {
        let v1 = Vec4::new(1.0, 2.0, 3.0, 4.0);
        let v2 = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(v1.to_pure_vec3(), v2);
    }

    #[test]
    fn test_norm() {
        let v1 = Vec4::new(1.0, 2.0, 3.0, 4.0);

        assert_eq!(v1.norm(), (14.0_f64).sqrt());
    }

    #[test]
    fn test_normalize() {
        let mut v1 = Vec4::new(1.0, 2.0, 3.0, 4.0);
        let sq = (14.0_f64).sqrt();
        let v2 = Vec4::new(1.0/sq, 2.0/sq, 3.0/sq, 4.0/sq);
        v1.normalize();
        assert_eq!(v1, v2);
    }

    #[test]
    fn test_normalize_zero() {
        let mut v1 = Vec4::new(0.0, 0.0, 0.0, 0.0);
        let v2 = v1.copy();
        v1.normalize();
        assert_eq!(v1, v2);
    }

    #[test]
    fn test_get_normliaze() {
        let mut v1 = Vec4::new(1.0, 2.0, 3.0, 4.0);
        let v2 = v1.copy();

        v1.normalize();
        assert_eq!(v1, v2.get_normliaze());
    }

    #[test]
    fn test_fmt() {
        let v1 = Vec4::new(1.0519, 2.10, 3.33, 4.01);
        assert_eq!(format!("{}", v1), "Vec4(x: 1.05, y: 2.10, z: 3.33, w: 4.01)");
    }
}
