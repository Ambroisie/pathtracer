use super::{Light, SpatialLight};
use crate::core::LinearColor;
use crate::{Point, Vector};
use nalgebra::Unit;
use serde::{Deserialize, Deserializer};

/// Represent a light emanating from a directed light-source, outputting rays in a cone.
///
/// The illumination cone cannot have an FOV over 180°.
#[derive(Debug, PartialEq)]
pub struct SpotLight {
    position: Point,
    direction: Unit<Vector>,
    cosine_value: f32,
    color: LinearColor,
}

impl SpotLight {
    /// Construct a SpotLight with the given FOV in radian.
    pub fn radians_new(
        position: Point,
        direction: Unit<Vector>,
        fov_rad: f32,
        color: LinearColor,
    ) -> Self {
        SpotLight {
            position,
            direction,
            cosine_value: (fov_rad / 2.).cos(),
            color,
        }
    }

    /// Construct a SpotLight with the given FOV in degrees.
    pub fn degrees_new(
        position: Point,
        direction: Unit<Vector>,
        fov_deg: f32,
        color: LinearColor,
    ) -> Self {
        SpotLight::radians_new(
            position,
            direction,
            std::f32::consts::PI * fov_deg / 180.,
            color,
        )
    }
}

impl Light for SpotLight {
    fn illumination(&self, point: &Point) -> LinearColor {
        let delt = point - self.position;
        let cos = self.direction.dot(&delt.normalize());
        if cos >= self.cosine_value {
            self.color.clone() / delt.norm_squared()
        } else {
            LinearColor::black()
        }
    }
}

impl SpatialLight for SpotLight {
    fn to_source(&self, point: &Point) -> (Unit<Vector>, f32) {
        let delt = self.position - point;
        let dist = delt.norm();
        (Unit::new_normalize(delt), dist)
    }
}

#[derive(Debug, Deserialize)]
struct SerializedSpotLight {
    position: Point,
    #[serde(deserialize_with = "crate::serialize::vector_normalizer")]
    direction: Unit<Vector>,
    fov: f32,
    color: LinearColor,
}

impl From<SerializedSpotLight> for SpotLight {
    fn from(light: SerializedSpotLight) -> Self {
        SpotLight::degrees_new(light.position, light.direction, light.fov, light.color)
    }
}

impl<'de> Deserialize<'de> for SpotLight {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let cam: SerializedSpotLight = Deserialize::deserialize(deserializer)?;
        Ok(cam.into())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn radian_new_works() {
        let light = SpotLight::radians_new(
            Point::origin(),
            Vector::x_axis(),
            std::f32::consts::PI / 2.,
            LinearColor::new(1., 1., 1.),
        );
        // The FOV is 90°, therefore the angle to the direction is 45° [= PI / 4]
        let calculated_cosine_value = (std::f32::consts::PI / 4.).cos();
        assert_eq!(
            light,
            SpotLight {
                position: Point::origin(),
                direction: Vector::x_axis(),
                cosine_value: calculated_cosine_value,
                color: LinearColor::new(1., 1., 1.),
            }
        );
        // Checking this way because of rounding issues...
        assert!((calculated_cosine_value - f32::sqrt(2.) / 2.).abs() < 1e-5)
    }

    #[test]
    fn degrees_new_works() {
        let light = SpotLight::degrees_new(
            Point::origin(),
            Vector::x_axis(),
            60.,
            LinearColor::new(1., 1., 1.),
        );
        let calculated_cosine_value = (std::f32::consts::PI * 60. / 360.).cos();
        assert_eq!(
            light,
            SpotLight {
                position: Point::origin(),
                direction: Vector::x_axis(),
                cosine_value: calculated_cosine_value,
                color: LinearColor::new(1., 1., 1.),
            }
        );
        // Checking this way because of rounding issues...
        assert!((calculated_cosine_value - f32::sqrt(3.) / 2.).abs() < 1e-5)
    }

    fn simple_light() -> impl SpatialLight {
        SpotLight::degrees_new(
            Point::origin(),
            Vector::x_axis(),
            90.,
            LinearColor::new(1., 1., 1.),
        )
    }

    #[test]
    fn illumination_in_axis_works() {
        let light = simple_light();
        let lum = light.illumination(&Point::new(1., 0., 0.));
        assert_eq!(lum, LinearColor::new(1., 1., 1.))
    }

    #[test]
    fn illumination_on_limit_works_1() {
        let light = simple_light();
        let lum = light.illumination(&Point::new(1., 1., 0.));
        assert_eq!(lum, LinearColor::new(0.5, 0.5, 0.5))
    }

    #[test]
    fn illumination_on_limit_works_2() {
        let light = simple_light();
        let lum = light.illumination(&Point::new(1., 0., 1.));
        assert_eq!(lum, LinearColor::new(0.5, 0.5, 0.5))
    }

    #[test]
    fn illumination_out_of_ray_works() {
        let light = simple_light();
        let lum = light.illumination(&Point::new(1., 1., 1.));
        assert_eq!(lum, LinearColor::new(0., 0., 0.))
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
        let yaml = r#"
            position: [0.0, 0.0, 0.0]
            direction: [1.0, 0.0, 0.0]
            fov: 90.0
            color: {r: 1.0, g: 0.5, b: 0.2}
        "#;
        let light: SpotLight = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(
            light,
            SpotLight::degrees_new(
                Point::origin(),
                Vector::x_axis(),
                90.,
                LinearColor::new(1., 0.5, 0.2)
            )
        )
    }
}
