use super::core::LinearColor;
use super::Point2D;

/// All the existing `Texture` implementation.
#[enum_dispatch::enum_dispatch]
#[derive(Debug, PartialEq)]
pub enum TextureEnum {
    UniformTexture,
}

/// Represent an object's texture.
#[enum_dispatch::enum_dispatch(TextureEnum)]
pub trait Texture: std::fmt::Debug {
    /// Get the color at a given texel coordinate
    fn texel_color(&self, point: Point2D) -> LinearColor;
}

pub mod uniform;
pub use uniform::*;
