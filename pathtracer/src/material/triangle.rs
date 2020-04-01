use super::Material;
use crate::core::{LightProperties, LinearColor, ReflTransEnum};
use crate::Point2D;
use serde::Deserialize;

/// Represent a material which interpolates between three points.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct TriangleMaterial {
    /// The diffuse components.
    diffuse: [LinearColor; 3],
    /// The specular components.
    specular: [LinearColor; 3],
    /// The transparency or reflectivity properties, this is not interpolated.
    #[serde(flatten)]
    refl_trans: Option<ReflTransEnum>,
    /// The amount of light emitted by the material, only used during path-tracing rendering.
    emitted: [LinearColor; 3],
}

impl Material for TriangleMaterial {
    fn properties(&self, point: Point2D) -> LightProperties {
        let (u, v) = (point.x, point.y);
        let sample = |param: &[LinearColor; 3]| -> LinearColor {
            param[0].clone() * (1. - u - v) + param[1].clone() * u + param[2].clone() * v
        };
        let diffuse = sample(&self.diffuse);
        let specular = sample(&self.specular);
        let emitted = sample(&self.emitted);
        LightProperties::new(diffuse, specular, self.refl_trans.clone(), emitted)
    }
}

// FIXME: tests
