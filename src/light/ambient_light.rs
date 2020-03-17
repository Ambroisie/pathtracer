use super::Light;
use crate::core::LinearColor;
use crate::Point;

/// Represent an ambient lighting which is equal in all points of the scene.
#[derive(Debug, PartialEq)]
pub struct AmbientLight {
    color: LinearColor,
}

impl AmbientLight {
    pub fn new(color: LinearColor) -> Self {
        AmbientLight { color }
    }
}

impl Light for AmbientLight {
    fn illumination(&self, _: &Point) -> LinearColor {
        self.color.clone()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new_works() {
        let color = LinearColor::new(1., 1., 1.);
        let light = AmbientLight::new(color.clone());
        let res = AmbientLight { color };
        assert_eq!(light, res)
    }

    #[test]
    fn illumination_is_correct() {
        let light = AmbientLight::new(LinearColor::new(1., 1., 1.));
        let lum = light.illumination(&Point::new(1., 1., 1.));
        assert_eq!(lum, LinearColor::new(1., 1., 1.))
    }
}
