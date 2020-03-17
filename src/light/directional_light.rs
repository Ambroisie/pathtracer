use super::{Light, SpatialLight};
use crate::core::LinearColor;
use crate::{Point, Vector};

/// Represent a light emanating from a far away source, with parallel rays on all points.
#[derive(Debug, PartialEq)]
pub struct DirectionalLight {
    direction: Vector,
    color: LinearColor,
}

impl DirectionalLight {
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
}
