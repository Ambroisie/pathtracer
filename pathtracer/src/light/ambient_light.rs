use super::Light;
use crate::core::LinearColor;
use crate::Point;
use serde::Deserialize;

/// Represent an ambient lighting which is equal in all points of the scene.
#[derive(Debug, PartialEq, Deserialize)]
pub struct AmbientLight {
    color: LinearColor,
}

impl AmbientLight {
    /// Creates a new `AmbientLight`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pathtracer::light::AmbientLight;
    /// # use pathtracer::core::color::LinearColor;
    /// #
    /// let amb_light = AmbientLight::new(LinearColor::new(1.0, 0.0, 1.0));
    /// ```
    pub fn new(color: LinearColor) -> Self {
        AmbientLight { color }
    }
}

impl Light for AmbientLight {
    fn illumination(&self, _: &Point) -> LinearColor {
        self.luminance()
    }

    fn luminance(&self) -> LinearColor {
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

    #[test]
    fn deserialization_works() {
        let yaml = "color: {r: 1.0, g: 0.5, b: 0.2}";
        let light: AmbientLight = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(light, AmbientLight::new(LinearColor::new(1., 0.5, 0.2)))
    }
}
