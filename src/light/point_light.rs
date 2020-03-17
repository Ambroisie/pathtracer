use super::{Light, SpatialLight};
use crate::core::LinearColor;
use crate::{Point, Vector};

/// Represent a light emanating from a point in space, following the square distance law.
#[derive(Debug, PartialEq)]
pub struct PointLight {
    position: Point,
    color: LinearColor,
}

impl PointLight {
    pub fn new(position: Point, color: LinearColor) -> Self {
        PointLight { position, color }
    }
}

impl Light for PointLight {
    fn illumination(&self, point: &Point) -> LinearColor {
        let dist = (self.position - point).norm();
        self.color.clone() / dist
    }
}

impl SpatialLight for PointLight {
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
        let expected = (Vector::new(-1., 0., 0.), 1.);
        assert_eq!(ans, expected);
    }
}
