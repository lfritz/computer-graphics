use crate::vec3::Vec3;

/// A defined by `origin + t*direction` for `t >= 0`.
#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3, // must not be (0, 0, 0)
}

impl Ray {
    pub fn at(&self, t: f64) -> Vec3 {
        self.origin + t * self.direction
    }
}
