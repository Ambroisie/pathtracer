use super::core::color::LinearColor;
use super::Point2D;
use serde::Deserialize;

/// A structure holding all the physical proprerties relating to light at a point.
#[derive(Debug, PartialEq, Clone)]
pub struct LightProperties {
    /// The diffuse component.
    pub diffuse: LinearColor,
    /// The specular component,
    pub specular: LinearColor,
    /// The reflectivity coefficient,
    pub reflectivity: f32,
}

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
