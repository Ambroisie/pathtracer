use super::core::LinearColor;
use super::Point2D;

/// Represent an object's texture.
pub trait Texture: std::fmt::Debug {
    /// Get the color at a given texel coordinate
    fn texel_color(&self, point: Point2D) -> LinearColor;
}

pub mod uniform;
pub use uniform::*;
