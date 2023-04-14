//! Defines a type for a vector in 3D space.

use crate::approx::ApproxEq;
use std::ops::{Add, Div, Mul, Neg, Sub};

/// A vector in 3D space.
#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    /// Returns a Vec3 with the given values.
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y, z }
    }

    /// Returns the dot product (inner product) of the two vectors.
    pub fn dot(&self, other: Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    /// Returns the length (Euclidian norm) of the vector.
    pub fn len(&self) -> f64 {
        self.dot(*self).sqrt()
    }

    /// Returns a vector with the same direction and length 1.
    pub fn normalized(&self) -> Vec3 {
        *self / self.len()
    }
}

impl ApproxEq for Vec3 {
    fn approx_eq(self, other: Vec3) -> bool {
        self.x.approx_eq(other.x) && self.y.approx_eq(other.y) && self.z.approx_eq(other.z)
    }
}

impl Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Vec3 {
        Vec3::new(-self.x, -self.y, -self.z)
    }
}

impl Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl Sub<Vec3> for Vec3 {
    type Output = Vec3;
    fn sub(self, other: Vec3) -> Vec3 {
        Vec3::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, v: Vec3) -> Vec3 {
        Vec3::new(v.x * self, v.y * self, v.z * self)
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, factor: f64) -> Vec3 {
        Vec3::new(self.x * factor, self.y * factor, self.z * factor)
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, factor: f64) -> Vec3 {
        Vec3::new(self.x / factor, self.y / factor, self.z / factor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::approx;

    #[test]
    fn vec3_approx_eq() {
        const SMALL: f64 = approx::EPS / 2.0;
        const BIG: f64 = approx::EPS * 2.0;
        let c = Vec3::new(0.1, 0.2, 0.3);
        assert!(c.approx_eq(Vec3::new(0.1 + SMALL, 0.2 - SMALL, 0.3 + SMALL)));
        assert!(!c.approx_eq(Vec3::new(0.1 + BIG, 0.2, 0.3)));
        assert!(!c.approx_eq(Vec3::new(0.1, 0.2 - BIG, 0.3)));
        assert!(!c.approx_eq(Vec3::new(0.1, 0.2, 0.3 + BIG)));
    }

    #[test]
    fn vec3_dot() {
        let a = Vec3::new(0.5, 2.0, -0.5);
        let b = Vec3::new(2.0, -0.5, 3.0);
        assert!(a.dot(b).approx_eq(-1.5));
    }

    #[test]
    fn vec3_ops() {
        let a = Vec3::new(0.1, 0.2, 0.3);
        let b = Vec3::new(-0.2, 0.4, -0.1);
        assert!((-a).approx_eq(Vec3::new(-0.1, -0.2, -0.3)));
        assert!((2.0 * a).approx_eq(Vec3::new(0.2, 0.4, 0.6)));
        assert!((a * 2.0).approx_eq(Vec3::new(0.2, 0.4, 0.6)));
        assert!((a / 2.0).approx_eq(Vec3::new(0.05, 0.10, 0.15)));
        assert!((a + b).approx_eq(Vec3::new(-0.1, 0.6, 0.2)));
        assert!((a - b).approx_eq(Vec3::new(0.3, -0.2, 0.4)));
    }

    #[test]
    fn vec3_len() {
        assert!(Vec3::new(0.0, 3.0, -4.0).len().approx_eq(5.0));
    }

    #[test]
    fn vec3_normalized() {
        assert!(Vec3::new(0.0, 3.0, -4.0)
            .normalized()
            .approx_eq(Vec3::new(0.0, 0.6, -0.8)));
    }
}
