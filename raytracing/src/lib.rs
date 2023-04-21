use color::Color;
use raytracer::Raytracer;
use scene::{Light, LightSource, Material, Scene, Sphere};
use std::io;
use vec3::Vec3;

pub mod approx;
pub mod canvas;
pub mod color;
pub mod ray;
pub mod raytracer;
pub mod scene;
pub mod vec3;

/// Run the raytracer.
pub fn run() {
    if let Err(e) = raytracing() {
        eprintln!("error: {}", e)
    }
}

/// Render a pre-defined scene and save it to a file called image.ppm.
fn raytracing() -> io::Result<()> {
    let scene = Scene {
        background_color: Color::BLACK,
        lights: vec![
            Light {
                intensity: 0.2,
                source: LightSource::Ambient,
            },
            Light {
                intensity: 0.6,
                source: LightSource::Point {
                    position: Vec3::new(2.0, 1.0, 0.0),
                },
            },
            Light {
                intensity: 0.2,
                source: LightSource::Directional {
                    direction: Vec3::new(1.0, 4.0, 4.0),
                },
            },
        ],
        spheres: vec![
            Sphere {
                center: Vec3::new(0.0, -1.0, 3.0),
                radius: 1.0,
                material: Material {
                    color: Color(1.0, 0.0, 0.0),
                    specular: Some(500),
                    reflective: 0.2,
                },
            },
            Sphere {
                center: Vec3::new(2.0, 0.0, 4.0),
                radius: 1.0,
                material: Material {
                    color: Color(0.0, 0.0, 1.0),
                    specular: Some(500),
                    reflective: 0.3,
                },
            },
            Sphere {
                center: Vec3::new(-2.0, 0.0, 4.0),
                radius: 1.0,
                material: Material {
                    color: Color(0.0, 1.0, 0.0),
                    specular: Some(10),
                    reflective: 0.4,
                },
            },
            Sphere {
                center: Vec3::new(0.0, -5001.0, 0.0),
                radius: 5000.0,
                material: Material {
                    color: Color(1.0, 1.0, 0.0),
                    specular: Some(1000),
                    reflective: 0.5,
                },
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
    canvas.save_to_ppm("image.ppm")
}
