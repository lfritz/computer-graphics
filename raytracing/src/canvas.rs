//! Defines a type for a rectangular canvas of RGB color values.

use crate::approx::ApproxEq;
use crate::color::Color;
use std::fs::File;
use std::io;
use std::io::Write;
use std::vec::Vec;

/// A rectangular canvas of RGB color values.
pub struct Canvas {
    width: usize,
    height: usize,
    pixels: Vec<Color>,
}

impl Canvas {
    /// Returns a new canvas with all color values zero (black).
    pub fn new(width: usize, height: usize) -> Canvas {
        let pixels = vec![Color::BLACK; width * height];
        Canvas {
            width,
            height,
            pixels,
        }
    }

    /// Set the color of a pixel.
    ///
    /// This method uses a coordinate system where `y` goes from the bottom of the image at
    /// `- height/2` to the top at `height/2 - 1`, and `x` goes from the left at `- width / 2` to
    /// the right at `width / 2 - 1`.
    pub fn put_pixel(&mut self, x: i32, y: i32, color: Color) {
        let w2 = (self.width / 2) as i32;
        let h2 = (self.height / 2) as i32;
        if x < -w2 || x >= w2 || y < -h2 || y >= h2 {
            return;
        }
        let x = (w2 + x) as usize;
        let y = (h2 - 1 - y) as usize;
        self.pixels[y * self.width + x] = color;
    }

    /// Save the image to a file in binary PPM format.
    pub fn save(&self, path: &str) -> io::Result<()> {
        let mut f = File::create(path)?;
        writeln!(f, "P6")?;
        writeln!(f, "{} {}", self.width, self.height)?;
        writeln!(f, "255")?;
        let buf: Vec<u8> = self
            .pixels
            .iter()
            .flat_map(|v| {
                let (r, g, b) = v.to_u8();
                vec![r, g, b]
            })
            .collect();
        f.write_all(&buf)
    }
}

impl ApproxEq for &Canvas {
    fn approx_eq(self, other: &Canvas) -> bool {
        self.width == other.width
            && self.height == other.height
            && !self
                .pixels
                .iter()
                .zip(other.pixels.iter())
                .any(|(p, q)| !p.approx_eq(*q))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canvas_approx_eq() {
        let a = Canvas::new(3, 2);
        assert!(a.approx_eq(&Canvas::new(3, 2)));
        assert!(!a.approx_eq(&Canvas::new(2, 3)));
        let p = Color::BLACK;
        let mut b = Canvas {
            width: 3,
            height: 2,
            pixels: vec![p, p, p, p, p, p],
        };
        assert!(a.approx_eq(&b));
        b.pixels[4] = Color(1.0, 0.0, 0.0);
        assert!(!a.approx_eq(&b));
    }

    #[test]
    fn canvas_put_pixel() {
        let mut c = Canvas::new(4, 2);

        // set the four corner pixels
        c.put_pixel(-2, -1, Color(1.0, 0.0, 0.0)); // bottom left
        c.put_pixel(1, -1, Color(0.0, 1.0, 0.0)); // bottom right
        c.put_pixel(-2, 0, Color(0.0, 0.0, 1.0)); // top left
        c.put_pixel(1, 0, Color(1.0, 1.0, 1.0)); // top right
        let want = Canvas {
            width: 4,
            height: 2,
            pixels: vec![
                Color(0.0, 0.0, 1.0),
                Color::BLACK,
                Color::BLACK,
                Color(1.0, 1.0, 1.0),
                Color(1.0, 0.0, 0.0),
                Color::BLACK,
                Color::BLACK,
                Color(0.0, 1.0, 0.0),
            ],
        };
        assert!(c.approx_eq(&want));

        // set some pixels that are out of range -- should have no effect
        c.put_pixel(-3, 0, Color(0.5, 0.5, 0.5));
        c.put_pixel(2, 0, Color(0.5, 0.5, 0.5));
        c.put_pixel(0, -2, Color(0.5, 0.5, 0.5));
        c.put_pixel(0, 1, Color(0.5, 0.5, 0.5));
        assert!(c.approx_eq(&want));
    }
}
