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
    pub refl_trans: Option<ReflTransEnum>,
}

impl Material for TriangleMaterial {
    fn properties(&self, point: Point2D) -> LightProperties {
        let (u, v) = (point.x, point.y);
        let diffuse = self.diffuse[0].clone() * (1. - u - v)
            + self.diffuse[1].clone() * u
            + self.diffuse[2].clone() * v;
        let specular = self.specular[0].clone() * (1. - u - v)
            + self.specular[1].clone() * u
            + self.specular[2].clone() * v;
        LightProperties::new(diffuse, specular, self.refl_trans.clone())
    }
}

// FIXME: tests
