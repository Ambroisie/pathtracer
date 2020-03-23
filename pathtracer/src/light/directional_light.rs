use super::{Light, SpatialLight};
use crate::core::LinearColor;
use crate::{Point, Vector};
use serde::Deserialize;

/// Represent a light emanating from a far away source, with parallel rays on all points.
#[derive(Debug, PartialEq, Deserialize)]
pub struct DirectionalLight {
    #[serde(deserialize_with = "crate::serialize::vector_normalizer")]
    direction: Vector,
    color: LinearColor,
}

impl DirectionalLight {
    /// Creates a new `DirectionalLight`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pathtracer::light::DirectionalLight;
    /// # use pathtracer::core::color::LinearColor;
    /// # use pathtracer::Vector;
    /// #
    /// let dir_light = DirectionalLight::new(
    ///     Vector::new(1.0, 0.0, 0.0),
    ///     LinearColor::new(1.0, 0.0, 1.0),
    /// );
    /// ```
    pub fn new(direction: Vector, color: LinearColor) -> Self {
        DirectionalLight {
            direction: direction.normalize(),
            color,
        }
    }
}

impl Light for DirectionalLight {
    fn illumination(&self, _: &Point) -> LinearColor {
        self.color.clone()
    }
}

impl SpatialLight for DirectionalLight {
    fn to_source(&self, _: &Point) -> (Vector, f32) {
        (self.direction * -1., std::f32::INFINITY)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new_works() {
        let direction = Vector::new(1., 0., 0.);
        let color = LinearColor::new(1., 1., 1.);
        let light = DirectionalLight::new(direction, color.clone());
        let res = DirectionalLight { direction, color };
        assert_eq!(light, res)
    }

    fn simple_light() -> impl SpatialLight {
        let direction = Vector::new(1., 0., 0.);
        let color = LinearColor::new(1., 1., 1.);
        DirectionalLight::new(direction, color)
    }

    #[test]
    fn illumination_is_correct() {
        let light = simple_light();
        let lum = light.illumination(&Point::new(1., 1., 1.));
        assert_eq!(lum, LinearColor::new(1., 1., 1.))
    }

    #[test]
    fn to_source_is_correct() {
        let light = simple_light();
        let ans = light.to_source(&Point::new(1., 0., 0.));
        let expected = (Vector::new(-1., 0., 0.), std::f32::INFINITY);
        assert_eq!(ans, expected)
    }

    #[test]
    fn deserialization_works() {
        let yaml = "{direction: [1.0, 0.0, 0.0], color: {r: 1.0, g: 0.5, b: 0.2}}";
        let light: DirectionalLight = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(
            light,
            DirectionalLight::new(Vector::new(1., 0., 0.), LinearColor::new(1., 0.5, 0.2))
        )
    }
}
