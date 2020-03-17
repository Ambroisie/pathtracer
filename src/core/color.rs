use derive_more::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign, Sum};
use std::ops::{Div, DivAssign, Mul, MulAssign};

#[derive(
    Debug, Clone, PartialEq, Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign, Sum,
)]
/// A structure to represent operations in the linear RGB colorspace.
pub struct LinearColor {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl LinearColor {
    pub fn black() -> Self {
        LinearColor {
            r: 0.,
            g: 0.,
            b: 0.,
        }
    }

    pub fn new(r: f32, g: f32, b: f32) -> Self {
        LinearColor { r, g, b }
    }
}

impl Default for LinearColor {
    fn default() -> Self {
        Self::black()
    }
}

impl Mul for LinearColor {
    type Output = LinearColor;

    fn mul(self, other: Self) -> Self::Output {
        LinearColor {
            r: self.r * other.r,
            g: self.g * other.g,
            b: self.b * other.b,
        }
    }
}

impl MulAssign for LinearColor {
    fn mul_assign(&mut self, other: Self) {
        *self = self.clone() * other
    }
}

impl Div for LinearColor {
    type Output = LinearColor;

    fn div(self, other: Self) -> Self::Output {
        LinearColor {
            r: self.r / other.r,
            g: self.g / other.g,
            b: self.b / other.b,
        }
    }
}

impl DivAssign for LinearColor {
    fn div_assign(&mut self, other: Self) {
        *self = self.clone() / other
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn black_is_black() {
        let black = LinearColor::black();
        assert_eq!(
            black,
            LinearColor {
                r: 0.,
                g: 0.,
                b: 0.
            }
        )
    }

    #[test]
    fn default_is_black() {
        assert_eq!(<LinearColor as Default>::default(), LinearColor::black())
    }

    #[test]
    fn red_is_red() {
        let red = LinearColor::new(1., 0., 0.);
        assert_eq!(
            red,
            LinearColor {
                r: 1.,
                g: 0.,
                b: 0.
            }
        )
    }

    #[test]
    fn green_is_green() {
        let green = LinearColor::new(0., 1., 0.);
        assert_eq!(
            green,
            LinearColor {
                r: 0.,
                g: 1.,
                b: 0.
            }
        )
    }

    #[test]
    fn blue_is_blue() {
        let blue = LinearColor::new(0., 0., 1.);
        assert_eq!(
            blue,
            LinearColor {
                r: 0.,
                g: 0.,
                b: 1.
            }
        )
    }

    #[test]
    fn mul_by_float_works() {
        let color = LinearColor::new(0.125, 0.25, 0.0625);
        assert_eq!(
            color * 4.,
            LinearColor {
                r: 0.5,
                g: 1.,
                b: 0.25,
            }
        )
    }

    #[test]
    fn div_by_float_works() {
        let color = LinearColor::new(0.2, 0.4, 0.6);
        assert_eq!(
            color / 2.,
            LinearColor {
                r: 0.1,
                g: 0.2,
                b: 0.3,
            }
        )
    }

    #[test]
    fn mulassign_by_float_works() {
        let mut color = LinearColor::new(0.125, 0.25, 0.0625);
        color *= 4.;
        assert_eq!(
            color,
            LinearColor {
                r: 0.5,
                g: 1.,
                b: 0.25,
            }
        )
    }

    #[test]
    fn divassign_by_float_works() {
        let mut color = LinearColor::new(0.2, 0.4, 0.6);
        color /= 2.;
        assert_eq!(
            color,
            LinearColor {
                r: 0.1,
                g: 0.2,
                b: 0.3,
            }
        )
    }

    #[test]
    fn mul_by_color_works() {
        let lhs = LinearColor::new(0.125, 0.25, 0.0625);
        let rhs = LinearColor::new(1.0, 0.5, 2.0);
        assert_eq!(lhs * rhs, LinearColor::new(0.125, 0.125, 0.125))
    }

    #[test]
    fn div_by_color_works() {
        let lhs = LinearColor::new(1.0, 0.5, 0.25);
        let rhs = LinearColor::new(4.0, 2.0, 1.0);
        assert_eq!(lhs / rhs, LinearColor::new(0.25, 0.25, 0.25))
    }

    #[test]
    fn mulassign_by_color_works() {
        let mut lhs = LinearColor::new(0.125, 0.25, 0.0625);
        lhs *= LinearColor::new(1.0, 0.5, 2.0);
        assert_eq!(lhs, LinearColor::new(0.125, 0.125, 0.125))
    }

    #[test]
    fn divassign_by_color_works() {
        let mut lhs = LinearColor::new(1.0, 0.5, 0.25);
        lhs /= LinearColor::new(4.0, 2.0, 1.0);
        assert_eq!(lhs, LinearColor::new(0.25, 0.25, 0.25))
    }

    #[test]
    fn add_works() {
        let lhs = LinearColor::new(1., 0., 0.125);
        let rhs = LinearColor::new(0., 0.5, 0.25);
        assert_eq!(
            lhs + rhs,
            LinearColor {
                r: 1.,
                g: 0.5,
                b: 0.375,
            }
        );
    }

    #[test]
    fn sub_works() {
        let lhs = LinearColor::new(1., 0.5, 0.25);
        let rhs = LinearColor::new(0.5, 0.125, 0.25);
        assert_eq!(
            lhs - rhs,
            LinearColor {
                r: 0.5,
                g: 0.375,
                b: 0.,
            }
        );
    }

    #[test]
    fn addassign_works() {
        let mut lhs = LinearColor::new(1., 0., 0.125);
        lhs += LinearColor::new(0., 0.5, 0.25);
        assert_eq!(
            lhs,
            LinearColor {
                r: 1.,
                g: 0.5,
                b: 0.375,
            }
        );
    }

    #[test]
    fn subassign_works() {
        let mut lhs = LinearColor::new(1., 0.5, 0.25);
        lhs -= LinearColor::new(0.5, 0.125, 0.25);
        assert_eq!(
            lhs,
            LinearColor {
                r: 0.5,
                g: 0.375,
                b: 0.,
            }
        );
    }
}
