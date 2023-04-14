//! Definitions for comparing floats for approximate equality.

pub const EPS: f64 = 1E-8;

/// Used for types that can be compared for approximate equality.
pub trait ApproxEq {
    /// Returns true if the two values are approximately equal.
    fn approx_eq(self, other: Self) -> bool;
}

impl ApproxEq for f64 {
    fn approx_eq(self, other: f64) -> bool {
        (self - other).abs() < EPS
    }
}

impl ApproxEq for &Vec<f64> {
    fn approx_eq(self, other: &Vec<f64>) -> bool {
        if self.len() != other.len() {
            return false;
        }
        !self.iter().zip(other.iter()).any(|(a, b)| !a.approx_eq(*b))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SMALL: f64 = EPS / 2.0;
    const LARGE: f64 = EPS * 2.0;

    #[test]
    fn approx_eq_for_f64() {
        let x = 0.0;
        let y = -123.45;
        assert!(x.approx_eq(x));
        assert!(y.approx_eq(y));
        assert!(x.approx_eq(x + SMALL));
        assert!(x.approx_eq(x - SMALL));
        assert!(y.approx_eq(y + SMALL));
        assert!(y.approx_eq(y - SMALL));
        assert!(!x.approx_eq(x + LARGE));
        assert!(!x.approx_eq(x - LARGE));
        assert!(!y.approx_eq(y + LARGE));
        assert!(!y.approx_eq(y - LARGE));
    }

    #[test]
    fn approx_eq_for_vec() {
        let v = vec![0.1, 0.2, 0.3];
        assert!(v.approx_eq(&vec![0.1, 0.2, 0.3]));
        assert!(!v.approx_eq(&vec![0.1, 0.2]));
        assert!(!v.approx_eq(&vec![0.1, 0.2, 0.3, 0.4]));
        assert!(v.approx_eq(&vec![0.1 + SMALL, 0.2 - SMALL, 0.3 + SMALL]));
        assert!(!v.approx_eq(&vec![0.1 + LARGE, 0.2, 0.3]));
        assert!(!v.approx_eq(&vec![0.1, 0.2 - LARGE, 0.3]));
        assert!(!v.approx_eq(&vec![0.1, 0.2, 0.3 + LARGE]));
    }
}
