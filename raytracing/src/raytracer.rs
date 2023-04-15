use crate::canvas::Canvas;
use crate::color::Color;
use crate::scene::{Light, Scene, Sphere};
use crate::vec3::Vec3;

pub struct Raytracer {
    pub canvas_width: usize,
    pub canvas_height: usize,
    pub viewport_width: f64,
    pub viewport_height: f64,
    pub distance_to_projection_plane: f64,
    pub scene: Scene,
}

impl Raytracer {
    pub fn go(&self) -> Canvas {
        let mut canvas = Canvas::new(self.canvas_width, self.canvas_height);
        let o = Vec3::new(0.0, 0.0, 0.0);
        let cw = self.canvas_width as i32;
        let ch = self.canvas_height as i32;
        let recursion_depth = 3;
        for x in (-cw / 2)..(cw / 2) {
            for y in (-ch / 2)..(ch / 2) {
                let d = self.canvas_to_viewport(x, y);
                let color = self.trace_ray(o, d, 1.0, f64::INFINITY, recursion_depth);
                canvas.put_pixel(x, y, color);
            }
        }
        canvas
    }

    fn canvas_to_viewport(&self, x: i32, y: i32) -> Vec3 {
        Vec3 {
            x: (x as f64) * self.viewport_width / (self.canvas_width as f64),
            y: (y as f64) * self.viewport_height / (self.canvas_height as f64),
            z: self.distance_to_projection_plane,
        }
    }

    fn closest_intersection(
        &self,
        o: Vec3,
        d: Vec3,
        t_min: f64,
        t_max: f64,
    ) -> Option<(Sphere, f64)> {
        let mut closest: Option<(Sphere, f64)> = None;
        for sphere in &self.scene.spheres {
            let ts = sphere.intersect_ray(o, d);
            for t in ts {
                if t < t_min || t > t_max {
                    continue;
                }
                if closest.map_or(true, |(_, closest_t)| t < closest_t) {
                    closest = Some((*sphere, t));
                }
            }
        }
        closest
    }

    fn trace_ray(&self, o: Vec3, d: Vec3, t_min: f64, t_max: f64, recursion_depth: i32) -> Color {
        let closest = self.closest_intersection(o, d, t_min, t_max);
        closest.map_or(self.scene.background_color, |(sphere, t)| {
            // compute local color
            let p = o + t * d; // point where the ray intersects the sphere
            let n = (p - sphere.center).normalized(); // normal
            let local_color = sphere.color * self.compute_lighting(p, n, -d, sphere.specular);

            // check if we need the reflective color
            let r = sphere.reflective;
            if recursion_depth <= 0 || r <= 0.0 {
                return local_color;
            }

            // compute reflected color
            let reflected_color = self.trace_ray(
                p,
                reflect_ray(-d, n),
                0.001,
                f64::INFINITY,
                recursion_depth - 1,
            );
            local_color * (1.0 - r) + reflected_color * r
        })
    }

    fn compute_lighting(&self, p: Vec3, n: Vec3, v: Vec3, specular: Option<i32>) -> f64 {
        let mut i = 0.0;
        for light in &self.scene.lights {
            if let Light::Ambient { intensity } = *light {
                i += intensity;
            } else {
                let (intensity, l, t_max) = match *light {
                    Light::Point {
                        intensity,
                        position,
                    } => (intensity, position - p, 1.0),
                    Light::Directional {
                        intensity,
                        direction,
                    } => (intensity, direction, f64::INFINITY),
                    _ => unreachable!(),
                };

                // shadow check
                if self.closest_intersection(p, l, 0.001, t_max).is_some() {
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
                    let r = reflect_ray(l, n);
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
}

fn reflect_ray(r: Vec3, n: Vec3) -> Vec3 {
    2.0 * n * n.dot(r) - r
}
