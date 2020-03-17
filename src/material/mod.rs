use super::core::color::LinearColor;
use super::Point2D;

/// Represent the physical light properties of an object in the scene;
pub trait Material: std::fmt::Debug {
    /// The diffuse component on a texel point.
    fn diffuse(&self, point: Point2D) -> LinearColor;
    /// The specular component on a texel point.
    fn specular(&self, point: Point2D) -> LinearColor;
}

pub mod uniform;
pub use uniform::*;
