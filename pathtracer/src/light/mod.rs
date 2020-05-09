//! Various light implementations

use super::core::LinearColor;
use super::{Point, Vector};
use beevee::ray::Ray;
use nalgebra::Unit;

/// Represent a light in the scene being rendered.
pub trait Light: std::fmt::Debug {
    /// Get the illumination of that light on that point.
    fn illumination(&self, point: &Point) -> LinearColor;
}

/// Represent a light which has an abstract position in the scene being rendered.
pub trait SpatialLight: Light {
    /// Get a unit vector from the origin to the position of the light, and its distance
    fn to_source(&self, origin: &Point) -> (Unit<Vector>, f32);
}

/// Represent a light from which we can sample a random `Ray`.
pub trait SampleLight: Light {
    /// Uniformly sample a ray from the point-light in a random direction.
    ///
    /// # Examles
    ///
    ///```
    /// # use pathtracer::light::{PointLight, SampleLight};
    /// # use pathtracer::core::color::LinearColor;
    /// # use pathtracer::Point;
    /// #
    /// let dir_light = PointLight::new(
    ///     Point::origin(),
    ///     LinearColor::new(1.0, 0.0, 1.0),
    /// );
    /// let sampled = dir_light.sample_ray();
    /// ```
    fn sample_ray(&self) -> Ray;
}

mod ambient_light;
pub use ambient_light::*;

mod directional_light;
pub use directional_light::*;

mod point_light;
pub use point_light::*;

mod spot_light;
pub use spot_light::*;
