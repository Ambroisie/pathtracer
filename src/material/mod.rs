use super::core::color::LinearColor;

/// Represent the physical light properties of an object in the scene.
#[derive(Debug, PartialEq)]
pub struct Material {
    /// The diffuse component.
    diffuse: LinearColor,
    /// The specular component.
    specular: LinearColor,
}

impl Material {
    pub fn new(diffuse: LinearColor, specular: LinearColor) -> Self {
        Material { diffuse, specular }
    }

    pub fn diffuse(&self) -> &LinearColor {
        &self.diffuse
    }

    pub fn specular(&self) -> &LinearColor {
        &self.specular
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new_works() {
        let diffuse = LinearColor::new(0., 0.5, 0.);
        let specular = LinearColor::new(1., 1., 1.);
        let mat = Material::new(diffuse.clone(), specular.clone());
        assert_eq!(mat, Material { diffuse, specular })
    }

    fn simple_material() -> Material {
        Material::new(
            LinearColor::new(0.5, 0.5, 0.5),
            LinearColor::new(1., 1., 1.),
        )
    }

    #[test]
    fn diffuse_works() {
        let mat = simple_material();
        assert_eq!(mat.diffuse(), &LinearColor::new(0.5, 0.5, 0.5))
    }

    #[test]
    fn specular_works() {
        let mat = simple_material();
        assert_eq!(mat.specular(), &LinearColor::new(1., 1., 1.))
    }
}
