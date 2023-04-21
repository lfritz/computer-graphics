//! Defines a type for color values and some operations on it.

use crate::approx::ApproxEq;
use std::ops::{Add, AddAssign, Mul};

/// A color with red, green, and blue values. The values should be between 0 and 1; other values
/// will be clamped to the [0, 1] range.
#[derive(Debug, Clone, Copy)]
pub struct Color(pub f64, pub f64, pub f64);

impl Color {
    pub const BLACK: Color = Color(0.0, 0.0, 0.0);
    pub const WHITE: Color = Color(1.0, 1.0, 1.0);

    /// Converts the red, green, blue values to u8, mapping the range [0, 1] to [0, 0xff].
    pub fn to_u8(&self) -> (u8, u8, u8) {
        let Color(r, g, b) = *self;
        (to_u8(r), to_u8(g), to_u8(b))
    }
}

impl ApproxEq for Color {
    fn approx_eq(self, other: Color) -> bool {
        self.0.approx_eq(other.0) && self.1.approx_eq(other.1) && self.2.approx_eq(other.2)
    }
}

impl Add<Color> for Color {
    type Output = Color;

    fn add(self, other: Color) -> Color {
        Color(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl AddAssign<Color> for Color {
    fn add_assign(&mut self, other: Color) {
        self.0 += other.0;
        self.1 += other.1;
        self.2 += other.2;
    }
}

impl Mul<Color> for f64 {
    type Output = Color;

    fn mul(self, color: Color) -> Color {
        let Color(r, g, b) = color;
        Color(self * r, self * g, self * b)
    }
}

impl Mul<f64> for Color {
    type Output = Color;

    fn mul(self, attenuation: f64) -> Color {
        let Color(r, g, b) = self;
        Color(attenuation * r, attenuation * g, attenuation * b)
    }
}

fn to_u8(f: f64) -> u8 {
    (256.0 * f) as u8
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::approx;

    #[test]
    fn to_u8_works() {
        let cases = vec![
            (-0.5, 0),
            (0.0, 0),
            (0.001, 0),
            (0.5, 0x80),
            (0.999, 0xff),
            (1.0, 0xff),
            (1.5, 0xff),
            (12345678.9, 0xff),
        ];
        for (f, want) in cases {
            let got = to_u8(f);
            assert_eq!(got, want, "to_u8({}) == {}, want {}", f, got, want)
        }
    }

    #[test]
    fn color_u8() {
        assert_eq!(Color(0.25, 0.5, 0.75).to_u8(), (0x40, 0x80, 0xc0));
        assert_eq!(Color(-1.1, 0.0, 99.9).to_u8(), (0, 0, 0xff));
    }

    #[test]
    fn color_approx_eq() {
        const SMALL: f64 = approx::EPS / 2.0;
        const BIG: f64 = approx::EPS * 2.0;
        let c = Color(0.1, 0.2, 0.3);
        assert!(c.approx_eq(Color(0.1 + SMALL, 0.2 - SMALL, 0.3 + SMALL)));
        assert!(!c.approx_eq(Color(0.1 + BIG, 0.2, 0.3)));
        assert!(!c.approx_eq(Color(0.1, 0.2 - BIG, 0.3)));
        assert!(!c.approx_eq(Color(0.1, 0.2, 0.3 + BIG)));
    }

    #[test]
    fn color_ops() {
        let c = Color(0.1, 0.2, 0.3);
        let d = Color(0.2, 0.3, 0.4);
        assert!((2.0 * c).approx_eq(Color(0.2, 0.4, 0.6)));
        assert!((c * 2.0).approx_eq(Color(0.2, 0.4, 0.6)));
        assert!((c + d).approx_eq(Color(0.3, 0.5, 0.7)));
    }
}
