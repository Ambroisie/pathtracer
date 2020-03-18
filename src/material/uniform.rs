use super::Material;
use crate::core::color::LinearColor;
use crate::Point2D;
use serde::Deserialize;

/// A material with the same characteristics on all points.
#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct UniformMaterial {
    diffuse: LinearColor,
    specular: LinearColor,
    reflectivity: f32,
}

impl UniformMaterial {
    pub fn new(diffuse: LinearColor, specular: LinearColor, reflectivity: f32) -> Self {
        UniformMaterial {
            diffuse,
            specular,
            reflectivity,
        }
    }
}

impl Material for UniformMaterial {
    fn diffuse(&self, _: Point2D) -> LinearColor {
        self.diffuse.clone()
    }

    fn specular(&self, _: Point2D) -> LinearColor {
        self.specular.clone()
    }

    fn reflectivity(&self, _: Point2D) -> f32 {
        self.reflectivity
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new_works() {
        let diffuse = LinearColor::new(0., 0.5, 0.);
        let specular = LinearColor::new(1., 1., 1.);
        let reflectivity = 0.5;
        let mat = UniformMaterial::new(diffuse.clone(), specular.clone(), reflectivity);
        assert_eq!(
            mat,
            UniformMaterial {
                diffuse,
                specular,
                reflectivity
            }
        )
    }

    fn simple_material() -> impl Material {
        UniformMaterial::new(
            LinearColor::new(0.5, 0.5, 0.5),
            LinearColor::new(1., 1., 1.),
            0.5,
        )
    }

    #[test]
    fn diffuse_works() {
        let mat = simple_material();
        assert_eq!(
            mat.diffuse(Point2D::origin()),
            LinearColor::new(0.5, 0.5, 0.5)
        )
    }

    #[test]
    fn specular_works() {
        let mat = simple_material();
        assert_eq!(
            mat.specular(Point2D::origin()),
            LinearColor::new(1., 1., 1.)
        )
    }

    #[test]
    fn reflectivity_works() {
        let mat = simple_material();
        assert!(mat.reflectivity(Point2D::origin()) - 0.5 < std::f32::EPSILON)
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
            UniformMaterial::new(
                LinearColor::new(1., 0.5, 0.25),
                LinearColor::new(0.25, 0.125, 0.75),
                0.25
            )
        )
    }
}
