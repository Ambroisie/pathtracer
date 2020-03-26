//! Various texture implementations

use super::core::LinearColor;
use super::Point2D;
use serde::Deserialize;

/// All the existing `Texture` implementation.
#[serde(tag = "type")]
#[serde(rename_all = "lowercase")]
#[allow(missing_docs)]
#[enum_dispatch::enum_dispatch]
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub enum TextureEnum {
    #[serde(rename = "uniform")]
    UniformTexture,
    TriangleTexture,
}

/// Represent an object's texture.
#[enum_dispatch::enum_dispatch(TextureEnum)]
pub trait Texture: std::fmt::Debug {
    /// Get the color at a given texel coordinate
    fn texel_color(&self, point: Point2D) -> LinearColor;
}

mod triangle;
pub use triangle::*;

mod uniform;
pub use uniform::*;
