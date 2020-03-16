use super::core::LinearColor;
use super::{Point, Vector};

/// Represent a light in the scene being rendered.
pub trait Light: std::fmt::Debug {
    /// Get the illumination of that light on that point.
    fn illumination(&self, point: &Point) -> LinearColor;
}

/// Represent a light which has an abstract position in the scene being rendered.
pub trait SpatialLight: Light {
    /// Get a unit vector from the origin to the position of the light, and its distance
    fn to_source(&self, origin: &Point) -> (Vector, f32);
}
