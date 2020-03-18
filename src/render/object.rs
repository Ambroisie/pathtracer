use crate::material::MaterialEnum;
use crate::shape::ShapeEnum;
use crate::texture::TextureEnum;

/// An object being rendered in the scene.
#[derive(Debug, PartialEq)]
pub struct Object {
    pub shape: ShapeEnum,
    pub material: MaterialEnum,
    pub texture: TextureEnum,
}

impl Object {
    pub fn new(shape: ShapeEnum, material: MaterialEnum, texture: TextureEnum) -> Self {
        Object {
            shape,
            material,
            texture,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::core::color::LinearColor;
    use crate::material::UniformMaterial;
    use crate::shape::Sphere;
    use crate::texture::UniformTexture;
    use crate::Point;

    #[test]
    fn new_works() {
        let shape = Sphere::new(Point::origin(), 1.);
        let material = UniformMaterial::new(
            LinearColor::new(0.5, 0.5, 0.5),
            LinearColor::new(1., 1., 1.),
            0.5,
        );
        let texture = UniformTexture::new(LinearColor::new(0.25, 0.5, 1.));
        let object = Object::new(
            shape.clone().into(),
            material.clone().into(),
            texture.clone().into(),
        );
        assert_eq!(
            object,
            Object {
                shape: shape.into(),
                material: material.into(),
                texture: texture.into(),
            }
        )
    }
}
