use super::{Light, SpatialLight};
use crate::core::LinearColor;
use crate::{Point, Vector};

/// Represent a light emanating from a directed light-source, outputting rays in a cone.
/// The illumination cone cannot have an FOV over 180°.
#[derive(Debug, PartialEq)]
pub struct SpotLight {
    position: Point,
    direction: Vector,
    cosine_value: f32,
    color: LinearColor,
}

impl SpotLight {
    /// Construct a SpotLight with the given FOV in radian.
    pub fn radians_new(
        position: Point,
        direction: Vector,
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
        direction: Vector,
        fov_deg: f32,
        color: LinearColor,
    ) -> Self {
        SpotLight {
            position,
            direction,
            cosine_value: (std::f32::consts::PI * fov_deg / 360.).cos(),
            color,
        }
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
    fn to_source(&self, point: &Point) -> (Vector, f32) {
        let delt = self.position - point;
        let dist = delt.norm();
        (delt.normalize(), dist)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn radian_new_works() {
        let light = SpotLight::radians_new(
            Point::origin(),
            Vector::new(1., 0., 0.),
            std::f32::consts::PI / 2.,
            LinearColor::new(1., 1., 1.),
        );
        // The FOV is 90°, therefore the angle to the direction is 45° [= PI / 4]
        let calculated_cosine_value = (std::f32::consts::PI / 4.).cos();
        assert_eq!(
            light,
            SpotLight {
                position: Point::origin(),
                direction: Vector::new(1., 0., 0.),
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
            Vector::new(1., 0., 0.),
            60.,
            LinearColor::new(1., 1., 1.),
        );
        let calculated_cosine_value = (std::f32::consts::PI * 60. / 360.).cos();
        assert_eq!(
            light,
            SpotLight {
                position: Point::origin(),
                direction: Vector::new(1., 0., 0.),
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
            Vector::new(1., 0., 0.),
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
        let expected = (Vector::new(-1., 0., 0.), 1.);
        assert_eq!(ans, expected);
    }
}
