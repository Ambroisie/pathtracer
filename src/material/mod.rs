use super::core::color::LinearColor;
use super::Point2D;
use serde::Deserialize;

/// All the existing `Material` implementation.
#[serde(tag = "type")]
#[serde(rename_all = "lowercase")]
#[enum_dispatch::enum_dispatch]
#[derive(Debug, PartialEq, Deserialize)]
pub enum MaterialEnum {
    #[serde(rename = "uniform")]
    UniformMaterial,
}

/// Represent the physical light properties of an object in the scene;
#[enum_dispatch::enum_dispatch(MaterialEnum)]
pub trait Material: std::fmt::Debug {
    /// The diffuse component on a texel point.
    fn diffuse(&self, point: Point2D) -> LinearColor;
    /// The specular component on a texel point.
    fn specular(&self, point: Point2D) -> LinearColor;
    /// The reflectivity coefficient on a texel point.
    fn reflectivity(&self, point: Point2D) -> f32;
}

pub mod uniform;
pub use uniform::*;
