use crate::canvas::Canvas;
use crate::color::Color;
use crate::ray::Ray;
use crate::scene::{LightSource, Scene, Sphere};
use crate::vec3::Vec3;
use std::ops::Range;

/// Renders a static image with raytracing.
pub struct Raytracer {
    pub canvas_width: usize,
    pub canvas_height: usize,
    pub viewport_width: f64,
    pub viewport_height: f64,
    pub distance_to_projection_plane: f64,
    pub scene: Scene,
}

impl Raytracer {
    /// Run the raytracer.
    pub fn go(&self) -> Canvas {
        let mut canvas = Canvas::new(self.canvas_width, self.canvas_height);
        let origin = Vec3::new(0.0, 0.0, 0.0);
        let cw = self.canvas_width as i32;
        let ch = self.canvas_height as i32;
        let recursion_depth = 3;
        let offset = vec![-0.4, -0.2, 0.0, 0.2, 0.4];
        for x in (-cw / 2)..(cw / 2) {
            for y in (-ch / 2)..(ch / 2) {
                let mut average_color = Color::BLACK;
                for x_offset in offset.iter() {
                    for y_offset in offset.iter() {
                        let direction =
                            self.canvas_to_viewport(x as f64 + x_offset, y as f64 + y_offset);
                        let color = trace_ray(
                            &self.scene,
                            Ray { origin, direction },
                            1.0..f64::INFINITY,
                            recursion_depth,
                        );
                        average_color += 0.04 * color;
                    }
                }
                canvas.put_pixel(x, y, average_color);
            }
        }
        canvas
    }

    fn canvas_to_viewport(&self, x: f64, y: f64) -> Vec3 {
        Vec3 {
            x: x * self.viewport_width / (self.canvas_width as f64),
            y: y * self.viewport_height / (self.canvas_height as f64),
            z: self.distance_to_projection_plane,
        }
    }
}

/// Finds the first intersection between the ray and an object in the scene.
///
/// More precisely, for a ray defined by `origin + t*direction`, it looks for intersections with
/// objects in the scene for which `t` is in the given range, and if it finds any, selects the one
/// with the smallest `t` and returns the object and `t`.
fn closest_intersection(scene: &Scene, ray: Ray, t_range: Range<f64>) -> Option<(Sphere, f64)> {
    let mut closest: Option<(Sphere, f64)> = None;
    for sphere in &scene.spheres {
        let ts = sphere.intersect_ray(ray);
        for t in ts {
            if !t_range.contains(&t) {
                continue;
            }
            if closest.map_or(true, |(_, closest_t)| t < closest_t) {
                closest = Some((*sphere, t));
            }
        }
    }
    closest
}

/// Runs the raytracing algorithm for one pixel in the image.
fn trace_ray(scene: &Scene, ray: Ray, t_range: Range<f64>, recursion_depth: i32) -> Color {
    let closest = closest_intersection(scene, ray, t_range);
    closest.map_or(scene.background_color, |(sphere, t)| {
        // compute local color
        let p = ray.at(t); // point where the ray intersects the sphere
        let n = (p - sphere.center).normalized(); // normal
        let material = sphere.material;
        let local_color =
            material.color * compute_lighting(scene, p, n, -ray.direction, material.specular);

        // check if we need the reflective color
        let r = material.reflective;
        if recursion_depth <= 0 || r <= 0.0 {
            return local_color;
        }

        // compute reflected color
        let reflected_color = trace_ray(
            scene,
            Ray {
                origin: p,
                direction: reflect_ray(ray.direction, n),
            },
            0.001..f64::INFINITY,
            recursion_depth - 1,
        );
        local_color * (1.0 - r) + reflected_color * r
    })
}

/// Compute the light intensity for a point on a surface in the scene, taking into account shadows
/// but not reflections.
///
/// Arguments `p` and `n` are the point and its surface normal. Taking into account specular
/// reflection (for shiny objects) requires two more arguments: `v` for the direction to the
/// camera, and the `specular` parameters of the object's material.
fn compute_lighting(scene: &Scene, p: Vec3, n: Vec3, v: Vec3, specular: Option<i32>) -> f64 {
    let mut i = 0.0;
    for light in &scene.lights {
        let intensity = light.intensity;
        if let LightSource::Ambient = light.source {
            i += intensity;
        } else {
            let (intensity, l, t_max) = match light.source {
                LightSource::Point { position } => (intensity, position - p, 1.0),
                LightSource::Directional { direction } => (intensity, direction, f64::INFINITY),
                _ => unreachable!(),
            };

            // shadow check
            let ray = Ray {
                origin: p,
                direction: l,
            };
            if closest_intersection(scene, ray, 0.001..t_max).is_some() {
                continue;
            }

            // diffuse
            let n_dot_l = n.dot(l);
            let diffuse = if n_dot_l > 0.0 {
                intensity * n_dot_l / (n.len() * l.len())
            } else {
                0.0
            };

            // specular
            let specular = specular.map_or(0.0, |s| {
                let r = reflect_ray(-l, n);
                let r_dot_v = r.dot(v);
                if r_dot_v > 0.0 {
                    intensity * (r_dot_v / (r.len() * v.len())).powi(s)
                } else {
                    0.0
                }
            });

            i += diffuse + specular;
        }
    }
    i
}

/// Calculates how a ray would be reflected by a surface, given the surface normal.
fn reflect_ray(r: Vec3, n: Vec3) -> Vec3 {
    r - 2.0 * n * n.dot(r)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::approx::ApproxEq;
    use crate::scene::Light;
    use crate::scene::Material;

    #[test]
    fn reflect_ray_works() {
        let n = Vec3::new(0.0, 1.0, 0.0);
        let r = Vec3::new(0.0, -1.0, 0.0);
        assert!(reflect_ray(r, n).approx_eq(Vec3::new(0.0, 1.0, 0.0)));

        let r = Vec3::new(1.0, -1.0, 0.0);
        assert!(reflect_ray(r, n).approx_eq(Vec3::new(1.0, 1.0, 0.0)));

        let n = Vec3::new(-1.0, 0.0, 1.0).normalized();
        let r = Vec3::new(1.0, 0.0, -1.0);
        assert!(reflect_ray(r, n).approx_eq(Vec3::new(-1.0, 0.0, 1.0)));

        let r = Vec3::new(1.0, 0.0, 0.0);
        assert!(reflect_ray(r, n).approx_eq(Vec3::new(0.0, 0.0, 1.0)));
    }

    #[test]
    fn closest_intersection_works() {
        // set up scene with two spheres
        let scene = Scene {
            background_color: Color::BLACK,
            lights: vec![],
            spheres: vec![
                Sphere {
                    center: Vec3::new(0.0, 0.0, 3.0),
                    radius: 1.0,
                    material: Material::BLACK,
                },
                Sphere {
                    center: Vec3::new(0.0, 0.0, 7.0),
                    radius: 2.0,
                    material: Material::BLACK,
                },
            ],
        };

        // ray doesn't hit any sphere
        let origin = Vec3::new(0.0, 0.0, 0.0);
        let direction = Vec3::new(0.0, 1.0, 0.0);
        let ray = Ray { origin, direction };
        assert!(closest_intersection(&scene, ray, 0.0..f64::INFINITY).is_none());

        // ray hits the first sphere
        let origin = Vec3::new(0.0, -2.0, 3.0);
        let direction = Vec3::new(0.0, 1.0, 0.0);
        let ray = Ray { origin, direction };
        let (sphere, t) = closest_intersection(&scene, ray, 0.0..f64::INFINITY).unwrap();
        assert_eq!(sphere.radius, 1.0);
        assert!(t.approx_eq(1.0));

        // ray hits both spheres, closest_intersection should return the first hit
        let origin = Vec3::new(0.0, 0.0, 0.0);
        let direction = Vec3::new(0.0, 0.0, 1.0);
        let ray = Ray { origin, direction };
        let (sphere, t) = closest_intersection(&scene, ray, 0.0..f64::INFINITY).unwrap();
        assert_eq!(sphere.radius, 1.0);
        assert!(t.approx_eq(2.0));

        // ray hits both spheres, but only the hit for sphere 2 is within the range
        let (sphere, t) = closest_intersection(&scene, ray, 5.0..f64::INFINITY).unwrap();
        assert_eq!(sphere.radius, 2.0);
        assert!(t.approx_eq(5.0));

        // ray hits both spheres, but neither is within the range
        assert!(closest_intersection(&scene, ray, 10.0..f64::INFINITY).is_none());
        assert!(closest_intersection(&scene, ray, f64::NEG_INFINITY..1.0).is_none());
    }

    #[test]
    fn compute_lighting_works() {
        // only ambient light
        let background_color = Color(0.0, 0.0, 1.0);
        let scene = Scene {
            background_color,
            lights: vec![Light {
                intensity: 0.8,
                source: LightSource::Ambient,
            }],
            spheres: vec![
                Sphere {
                    center: Vec3::new(0.0, 0.0, -2.0),
                    radius: 1.0,
                    material: Material::BLACK,
                },
                Sphere {
                    center: Vec3::new(0.0, 0.0, 2.0),
                    radius: 1.0,
                    material: Material::BLACK,
                },
            ],
        };
        let p = Vec3::new(0.0, 0.0, 1.0);
        let n = Vec3::new(0.0, 0.0, -1.0);
        let v = n;
        assert!(compute_lighting(&scene, p, n, v, None).approx_eq(0.8));

        // only directional light, point is in shadow
        let scene = Scene {
            lights: vec![Light {
                intensity: 0.7,
                source: LightSource::Directional {
                    direction: Vec3::new(0.0, 0.0, 1.0),
                },
            }],
            ..scene
        };
        let p = Vec3::new(0.0, 0.0, 1.0);
        let n = Vec3::new(0.0, 0.0, 1.0);
        let v = Vec3::new(0.0, 0.0, 1.0);
        assert!(compute_lighting(&scene, p, n, v, None).approx_eq(0.0));

        // only directional light, camera is facing the back of the object
        let n = Vec3::new(0.0, 0.0, -1.0);
        let p = Vec3::new(0.0, 0.0, 1.0);
        assert!(compute_lighting(&scene, p, n, v, None).approx_eq(0.0));

        // only directional light
        let n = Vec3::new(0.0, 0.0, 1.0);
        let p = Vec3::new(0.0, 0.0, 3.0);
        assert!(compute_lighting(&scene, p, n, v, None).approx_eq(0.7));
        assert!(compute_lighting(&scene, p, n, v, Some(2)).approx_eq(0.7 + 0.7));

        // only directional light at a 45 degree angle to the surface
        let scene = Scene {
            lights: vec![Light {
                intensity: 0.7,
                source: LightSource::Directional {
                    direction: Vec3::new(0.0, 1.0, 1.0).normalized(),
                },
            }],
            ..scene
        };
        let diffuse = 0.7 / 2f64.sqrt();
        let specular = 0.7 / 2f64;
        assert!(compute_lighting(&scene, p, n, v, None).approx_eq(diffuse));
        assert!(compute_lighting(&scene, p, n, v, Some(2)).approx_eq(diffuse + specular));

        // only point light, point is in shadow
        let scene = Scene {
            lights: vec![Light {
                intensity: 0.7,
                source: LightSource::Point {
                    position: Vec3::new(0.0, 0.0, 5.0),
                },
            }],
            ..scene
        };
        let p = Vec3::new(0.0, 0.0, 1.0);
        let n = Vec3::new(0.0, 0.0, 1.0);
        let v = Vec3::new(0.0, 0.0, 1.0);
        assert!(compute_lighting(&scene, p, n, v, None).approx_eq(0.0));

        // only point light
        let n = Vec3::new(0.0, 0.0, 1.0);
        let p = Vec3::new(0.0, 0.0, 3.0);
        assert!(compute_lighting(&scene, p, n, v, None).approx_eq(0.7));
        assert!(compute_lighting(&scene, p, n, v, Some(2)).approx_eq(0.7 + 0.7));
    }

    #[test]
    fn trace_ray_works() {
        // scene with two spheres, symetrically to the left and right of the origin
        let red = Color(1.0, 0.0, 0.0);
        let green = Color(0.0, 1.0, 0.0);
        let blue = Color(0.0, 0.0, 1.0);
        let mut scene = Scene {
            background_color: blue,
            lights: vec![Light {
                intensity: 0.8,
                source: LightSource::Ambient,
            }],
            spheres: vec![
                Sphere {
                    center: Vec3::new(0.0, 0.0, -2.0),
                    radius: 1.0,
                    material: Material {
                        color: green,
                        specular: None,
                        reflective: 0.0,
                    },
                },
                Sphere {
                    center: Vec3::new(0.0, 0.0, 2.0),
                    radius: 1.0,
                    material: Material {
                        color: red,
                        specular: None,
                        reflective: 0.0,
                    },
                },
            ],
        };

        // ray doesn't hit anything => background color
        let ray = Ray {
            origin: Vec3::new(0.0, 0.0, 0.0),
            direction: Vec3::new(0.0, 1.0, 0.0),
        };
        assert!(trace_ray(&scene, ray, 0.0..f64::INFINITY, 2).approx_eq(blue));

        // ray hits red sphere, it's not reflective => just red
        let ray = Ray {
            origin: Vec3::new(0.0, 0.0, 0.0),
            direction: Vec3::new(0.0, 0.0, 1.0),
        };
        assert!(trace_ray(&scene, ray, 0.0..f64::INFINITY, 2).approx_eq(0.8 * red));

        // ray hits red sphere, recursion depth 0 => just red
        let ray = Ray {
            origin: Vec3::new(0.0, 0.0, 0.0),
            direction: Vec3::new(0.0, 0.0, 1.0),
        };
        scene.spheres[1].material.reflective = 0.6;
        assert!(trace_ray(&scene, ray, 0.0..f64::INFINITY, 0).approx_eq(0.8 * red));

        // ray hits red sphere, it reflects green sphere => red + green
        let ray = Ray {
            origin: Vec3::new(0.0, 0.0, 0.0),
            direction: Vec3::new(0.0, 0.0, 1.0),
        };
        let want = 0.8 * (0.4 * red + 0.6 * green);
        assert!(trace_ray(&scene, ray, 0.0..f64::INFINITY, 1).approx_eq(want));

        // both spheres are reflective, recursion depth 2 => red + green + some more red
        scene.spheres[0].material.reflective = 0.6;
        let want = 0.8 * (0.4 * red + 0.6 * (0.4 * green + 0.6 * red));
        assert!(dbg!(trace_ray(&scene, ray, 0.0..f64::INFINITY, 2)).approx_eq(dbg!(want)));
    }
}
