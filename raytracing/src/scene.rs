use crate::color::Color;
use crate::ray::Ray;
use crate::vec3::Vec3;

/// A scene that can be rendered by a raytracer.
#[derive(Debug)]
pub struct Scene {
    pub background_color: Color,
    pub lights: Vec<Light>,
    pub spheres: Vec<Sphere>,
}

/// A source of (white) light in the scene.
#[derive(Debug, Clone, Copy)]
pub struct Light {
    /// The intensity of the light.
    pub intensity: f64,

    /// Describes where the light comes from.
    pub source: LightSource,
}

/// Describes where a light comes from.
#[derive(Debug, Clone, Copy)]
pub enum LightSource {
    /// Ambient light has the same intensity anywhere in the scene.
    Ambient,

    /// A point light appears to shine from a single point.
    Point { position: Vec3 },

    /// A directional light shines in a fixed direction.
    Directional { direction: Vec3 },
}

/// Defines how an object reflects light.
#[derive(Debug, Clone, Copy)]
pub struct Material {
    /// The object's color.
    pub color: Color,

    /// A parameter to model specular reflection; higher values means a shinier surface.
    pub specular: Option<i32>,

    /// A parameter to model how reflective the surface is; goes from 0.0 (not reflective) to 1.0
    /// (perfect mirror).
    pub reflective: f64,
}

impl Material {
    pub const BLACK: Material = Material {
        color: Color::BLACK,
        specular: None,
        reflective: 0.0,
    };
}

/// A sphere in a scene.
#[derive(Debug, Clone, Copy)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub material: Material,
}

impl Sphere {
    /// Return the values `t` where the ray intersects the sphere.
    ///
    /// Returns 0 to 2 values in a sorted vector.
    pub fn intersect_ray(&self, ray: Ray) -> Vec<f64> {
        let r = self.radius;
        let co = ray.origin - self.center;

        let a = ray.direction.dot(ray.direction);
        let b = 2.0 * co.dot(ray.direction);
        let c = co.dot(co) - r * r;

        let discriminant = b * b - 4.0 * a * c;
        if discriminant < 0.0 {
            return vec![];
        }

        if discriminant == 0.0 {
            return vec![-b / (2.0 * a)];
        }

        let t1 = (-b + discriminant.sqrt()) / (2.0 * a);
        let t2 = (-b - discriminant.sqrt()) / (2.0 * a);
        if t1 < t2 {
            vec![t1, t2]
        } else {
            vec![t2, t1]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::approx::ApproxEq;

    #[test]
    fn sphere_intersect_ray() {
        let ray = Ray {
            origin: Vec3::new(0.0, 0.0, 0.0),
            direction: Vec3::new(0.0, 0.0, 1.0),
        };
        let sphere = Sphere {
            center: Vec3::new(0.0, 0.0, 3.0),
            radius: 1.0,
            material: Material {
                color: Color::BLACK,
                specular: None,
                reflective: 0.0,
            },
        };

        // simple case
        let want = vec![2.0, 4.0];
        assert!(sphere.intersect_ray(ray).approx_eq(&want));

        // smaller radius
        let small_sphere = Sphere {
            radius: 0.5,
            ..sphere
        };
        let want = vec![2.5, 3.5];
        assert!(small_sphere.intersect_ray(ray).approx_eq(&want));

        // larger radius
        let large_sphere = Sphere {
            radius: 2.0,
            ..sphere
        };
        let want = vec![1.0, 5.0];
        assert!(large_sphere.intersect_ray(ray).approx_eq(&want));

        // sphere to the left
        let sphere_left = Sphere {
            center: Vec3::new(-0.5, 0.0, 1.0),
            ..sphere
        };
        let want = vec![0.1339745962155614, 1.8660254037844386];
        assert!(sphere_left.intersect_ray(ray).approx_eq(&want));

        // sphere further to the left, no hits
        let sphere_far_left = Sphere {
            center: Vec3::new(1.5, 0.0, 1.0),
            ..sphere
        };
        assert!(sphere_far_left.intersect_ray(ray).approx_eq(&vec![]));

        // origin is inside sphere
        let sphere_close = Sphere {
            center: Vec3::new(0.0, 0.0, -0.5),
            ..sphere
        };
        let want = vec![-1.5, 0.5];
        assert!(sphere_close.intersect_ray(ray).approx_eq(&want));

        // sphere is behind origin
        let sphere_behind = Sphere {
            center: Vec3::new(0.0, 0.0, -1.5),
            ..sphere
        };
        let want = vec![-2.5, -0.5];
        assert!(sphere_behind.intersect_ray(ray).approx_eq(&want));

        // vary origin
        let origin_moved = Ray {
            origin: Vec3::new(0.1, 0.2, 0.3),
            ..ray
        };
        let want = vec![1.725320565519104, 3.6746794344808964];
        assert!(sphere.intersect_ray(origin_moved).approx_eq(&want));

        // vary direction
        let new_direction = Ray {
            direction: Vec3::new(0.1, 0.2, 2.0),
            ..ray
        };
        let want = vec![1.012995902197277, 1.949967060765686];
        assert!(sphere.intersect_ray(new_direction).approx_eq(&want));
    }
}
