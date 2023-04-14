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
        for x in (-cw / 2)..(cw / 2) {
            for y in (-ch / 2)..(ch / 2) {
                let d = self.canvas_to_viewport(x, y);
                let color = self.trace_ray(o, d, 1.0, f64::INFINITY);
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

    fn trace_ray(&self, o: Vec3, d: Vec3, t_min: f64, t_max: f64) -> Color {
        let mut closest: Option<(&Sphere, f64)> = None;
        for sphere in &self.scene.spheres {
            let ts = sphere.intersect_ray(o, d);
            for t in ts {
                if t < t_min || t > t_max {
                    continue;
                }
                if closest.map_or(true, |(_, closest_t)| t < closest_t) {
                    closest = Some((sphere, t));
                }
            }
        }
        closest.map_or(self.scene.background_color, |(sphere, t)| {
            let p = o + t * d; // point where the ray intersects the sphere
            let n = (p - sphere.center).normalized(); // normal
            sphere.color * self.compute_lighting(p, n, -d, sphere.specular)
        })
    }

    fn compute_lighting(&self, p: Vec3, n: Vec3, v: Vec3, specular: i32) -> f64 {
        let mut i = 0.0;
        for light in &self.scene.lights {
            i += match *light {
                Light::Ambient { intensity } => intensity,
                Light::Point {
                    intensity,
                    position,
                } => lighting(n, position - p, v, specular, intensity),
                Light::Directional {
                    intensity,
                    direction,
                } => lighting(n, direction, v, specular, intensity),
            }
        }
        i
    }
}

fn lighting(n: Vec3, l: Vec3, v: Vec3, specular: i32, intensity: f64) -> f64 {
    // diffuse
    let n_dot_l = n.dot(l);
    let diffuse = if n_dot_l > 0.0 {
        intensity * n_dot_l / (n.len() * l.len())
    } else {
        0.0
    };

    // specular
    let specular = if specular != -1 {
        let r = 2.0 * n * n.dot(l) - l;
        let r_dot_v = r.dot(v);
        if r_dot_v > 0.0 {
            intensity * (r_dot_v / (r.len() * v.len())).powi(specular)
        } else {
            0.0
        }
    } else {
        0.0
    };

    diffuse + specular
}
