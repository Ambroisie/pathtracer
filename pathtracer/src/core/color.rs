//! Color definition and operations

use derive_more::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign, Sum};
use serde::Deserialize;
use std::ops::{Div, DivAssign, Mul, MulAssign};

#[derive(
    Debug,
    Clone,
    PartialEq,
    Add,
    AddAssign,
    Div,
    DivAssign,
    Mul,
    MulAssign,
    Sub,
    SubAssign,
    Sum,
    Deserialize,
)]
/// A structure to represent operations in the linear RGB colorspace.
pub struct LinearColor {
    /// The color's red component
    pub r: f32,
    /// The color's green component
    pub g: f32,
    /// The color's blue component
    pub b: f32,
}

impl LinearColor {
    /// Creates the color black.
    ///
    /// All 3 components are set to 0.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pathtracer::core::LinearColor;
    /// #
    /// let black = LinearColor::black();
    /// assert_eq!(
    ///     black,
    ///     LinearColor {
    ///         r: 0.,
    ///         g: 0.,
    ///         b: 0.
    ///     }
    /// );
    /// ```
    pub fn black() -> Self {
        LinearColor {
            r: 0.,
            g: 0.,
            b: 0.,
        }
    }

    /// Creates a new `Color`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pathtracer::core::LinearColor;
    /// #
    /// let color = LinearColor::new(1.0, 0.0, 0.0); // bright red!
    /// ```
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        LinearColor { r, g, b }
    }

    #[must_use]
    /// Clamps the color's RGB components between 0.0 and 1.0.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pathtracer::core::LinearColor;
    /// #
    /// let color = LinearColor::new(1.5, -1.0, 0.5);
    /// assert_eq!(color.clamp(), LinearColor::new(1.0, 0.0, 0.5))
    /// ```
    pub fn clamp(self) -> Self {
        fn clamp(v: f32) -> f32 {
            if v > 1. {
                1.
            } else if v < 0. {
                0.
            } else {
                v
            }
        };
        LinearColor::new(clamp(self.r), clamp(self.g), clamp(self.b))
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

impl From<LinearColor> for image::Rgb<u8> {
    fn from(mut color: LinearColor) -> Self {
        color = color.clamp();
        image::Rgb([
            (color.r * 255.) as u8,
            (color.g * 255.) as u8,
            (color.b * 255.) as u8,
        ])
    }
}

#[cfg(test)]
mod test {
    use super::*;

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

    #[test]
    fn deserialization_works() {
        let yaml = "{r: 1.0, g: 0.5, b: 0.2}";
        let ans: LinearColor = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(
            ans,
            LinearColor {
                r: 1.0,
                g: 0.5,
                b: 0.2
            }
        )
    }
}
