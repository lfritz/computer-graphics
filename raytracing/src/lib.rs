use color::Color;
use raytracer::Raytracer;
use scene::{Light, Scene, Sphere};
use std::io;
use vec3::Vec3;

pub mod approx;
pub mod canvas;
pub mod color;
pub mod raytracer;
pub mod scene;
pub mod vec3;

pub fn run() {
    if let Err(e) = raytracing() {
        eprintln!("error: {}", e)
    }
}

fn raytracing() -> io::Result<()> {
    let scene = Scene {
        background_color: Color::BLACK,
        //background_color: Color(0.0, 0.71, 0.89),
        lights: vec![
            Light::Ambient { intensity: 0.2 },
            Light::Point {
                intensity: 0.6,
                position: Vec3::new(2.0, 1.0, 0.0),
            },
            Light::Directional {
                intensity: 0.2,
                direction: Vec3::new(1.0, 4.0, 4.0),
            },
        ],
        spheres: vec![
            Sphere {
                center: Vec3::new(0.0, -1.0, 3.0),
                radius: 1.0,
                color: Color(1.0, 0.0, 0.0),
                specular: Some(500),
                reflective: 0.2,
            },
            Sphere {
                center: Vec3::new(2.0, 0.0, 4.0),
                radius: 1.0,
                color: Color(0.0, 0.0, 1.0),
                specular: Some(500),
                reflective: 0.3,
            },
            Sphere {
                center: Vec3::new(-2.0, 0.0, 4.0),
                radius: 1.0,
                color: Color(0.0, 1.0, 0.0),
                specular: Some(10),
                reflective: 0.4,
            },
            Sphere {
                center: Vec3::new(0.0, -5001.0, 0.0),
                radius: 5000.0,
                color: Color(1.0, 1.0, 0.0),
                specular: Some(1000),
                reflective: 0.5,
            },
        ],
    };
    let raytracer = Raytracer {
        canvas_width: 640,
        canvas_height: 640,
        viewport_width: 1.0,
        viewport_height: 1.0,
        distance_to_projection_plane: 1.0,
        scene,
    };
    let canvas = raytracer.go();
    canvas.save("image.ppm")
}
