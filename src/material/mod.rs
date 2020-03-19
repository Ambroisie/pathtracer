use super::core::LightProperties;
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
    /// Get the physical properties at a point.
    fn properties(&self, point: Point2D) -> LightProperties;
}

pub mod uniform;
pub use uniform::*;
