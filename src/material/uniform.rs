use super::Material;
use crate::core::LightProperties;
use crate::Point2D;
use serde::Deserialize;

/// A material with the same characteristics on all points.
#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct UniformMaterial {
    #[serde(flatten)]
    properties: LightProperties,
}

impl UniformMaterial {
    pub fn new(properties: LightProperties) -> Self {
        UniformMaterial { properties }
    }
}

impl Material for UniformMaterial {
    fn properties(&self, _: Point2D) -> LightProperties {
        self.properties.clone()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::core::color::LinearColor;
    use crate::core::ReflTransEnum;

    #[test]
    fn new_works() {
        let properties = LightProperties {
            diffuse: LinearColor::new(0., 0.5, 0.),
            specular: LinearColor::new(1., 1., 1.),
            refl_trans: None,
        };
        let mat = UniformMaterial::new(properties.clone());
        assert_eq!(mat, UniformMaterial { properties })
    }

    #[test]
    fn properties_works() {
        let properties = LightProperties::new(
            LinearColor::new(0., 0.5, 0.),
            LinearColor::new(1., 1., 1.),
            None,
        );
        let mat = UniformMaterial::new(properties.clone());
        assert_eq!(mat.properties(Point2D::origin()), properties)
    }

    #[test]
    fn deserialization_works() {
        let yaml = r#"
            diffuse: {r: 1.0, g: 0.5, b: 0.25}
            specular: {r: 0.25, g: 0.125, b: 0.75}
            reflectivity: 0.25
        "#;
        let material: UniformMaterial = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(
            material,
            UniformMaterial::new(LightProperties::new(
                LinearColor::new(1., 0.5, 0.25),
                LinearColor::new(0.25, 0.125, 0.75),
                Some(ReflTransEnum::Reflectivity { coef: 0.25 })
            ))
        )
    }
}
