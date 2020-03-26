use super::{uniform::UniformTexture, Texture};
use crate::core::LinearColor;
use crate::Point2D;
use serde::Deserialize;

/// Represent a texture which interpolates between three points.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct TriangleTexture {
    /// The texture at each point
    textures: [UniformTexture; 3],
}

impl Texture for TriangleTexture {
    fn texel_color(&self, point: Point2D) -> LinearColor {
        let (u, v) = (point.x, point.y);
        let sum = self.textures[0].texel_color(point) * (1. - u - v)
            + self.textures[1].texel_color(point) * u
            + self.textures[2].texel_color(point) * v;
        sum / 3.
    }
}

// FIXME: tests
