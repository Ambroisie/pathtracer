use super::{Light, SpatialLight};
use crate::core::LinearColor;
use crate::{Point, Vector};
use beevee::ray::Ray;
use nalgebra::Unit;
use rand::{distributions::Uniform, Rng};
use serde::Deserialize;

/// Represent a light emanating from a point in space, following the square distance law.
#[derive(Debug, PartialEq, Deserialize)]
pub struct PointLight {
    position: Point,
    color: LinearColor,
}

impl PointLight {
    /// Creates a new `PointLight`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pathtracer::light::PointLight;
    /// # use pathtracer::core::color::LinearColor;
    /// # use pathtracer::Point;
    /// #
    /// let dir_light = PointLight::new(
    ///     Point::origin(),
    ///     LinearColor::new(1.0, 0.0, 1.0),
    /// );
    /// ```
    pub fn new(position: Point, color: LinearColor) -> Self {
        PointLight { position, color }
    }

    /// Uniformly sample a ray from the point-light in a random direction.
    ///
    /// # Examles
    ///
    ///```
    /// # use pathtracer::light::PointLight;
    /// # use pathtracer::core::color::LinearColor;
    /// # use pathtracer::Point;
    /// #
    /// let dir_light = PointLight::new(
    ///     Point::origin(),
    ///     LinearColor::new(1.0, 0.0, 1.0),
    /// );
    /// let sampled = dir_light.sample_ray();
    /// ```
    pub fn sample_ray(&self) -> Ray {
        let mut rng = rand::thread_rng();
        // Sample sphere uniformly
        // See <https://mathworld.wolfram.com/SpherePointPicking.html>
        let theta = rng.gen_range(0., std::f32::consts::PI * 2.);
        let y = rng.sample(Uniform::new(-1., 1.)); // Inclusive for the poles
        let dir = Unit::new_unchecked(Vector::new(
            // this vector is already of unit length
            f32::sqrt(1. - y * y) * f32::cos(theta),
            y,
            f32::sqrt(1. - y * y) * f32::sin(theta),
        ));
        Ray::new(self.position, dir)
    }
}

impl Light for PointLight {
    fn illumination(&self, point: &Point) -> LinearColor {
        let dist = (self.position - point).norm();
        self.color.clone() / dist
    }
}

impl SpatialLight for PointLight {
    fn to_source(&self, point: &Point) -> (Unit<Vector>, f32) {
        let delt = self.position - point;
        let dist = delt.norm();
        (Unit::new_normalize(delt), dist)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new_works() {
        let position = Point::origin();
        let color = LinearColor::black();
        let light = PointLight::new(position, color.clone());
        let res = PointLight { position, color };
        assert_eq!(light, res)
    }

    fn simple_light() -> impl SpatialLight {
        let position = Point::origin();
        let color = LinearColor::new(1., 1., 1.);
        PointLight::new(position, color)
    }

    #[test]
    fn illumination_is_correct() {
        let light = simple_light();
        let lum = light.illumination(&Point::new(1., 0., 0.));
        assert_eq!(lum, LinearColor::new(1., 1., 1.))
    }

    #[test]
    fn to_source_is_correct() {
        let light = simple_light();
        let ans = light.to_source(&Point::new(1., 0., 0.));
        let expected = (Unit::new_normalize(Vector::new(-1., 0., 0.)), 1.);
        assert_eq!(ans, expected);
    }

    #[test]
    fn deserialization_works() {
        let yaml = "{position: [1.0, 1.0, 1.0], color: {r: 1.0, g: 0.5, b: 0.2}}";
        let light: PointLight = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(
            light,
            PointLight::new(Point::new(1., 1., 1.), LinearColor::new(1., 0.5, 0.2))
        )
    }
}
