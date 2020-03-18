use crate::material::MaterialEnum;
use crate::shape::{Shape, ShapeEnum};
use crate::texture::TextureEnum;
use bvh::aabb::{Bounded, AABB};
use bvh::bounding_hierarchy::BHShape;
use serde::Deserialize;

/// An object being rendered in the scene.
#[derive(Debug, PartialEq, Deserialize)]
pub struct Object {
    pub shape: ShapeEnum,
    pub material: MaterialEnum,
    pub texture: TextureEnum,
    #[serde(skip_deserializing)]
    index: usize,
}

impl Object {
    pub fn new(shape: ShapeEnum, material: MaterialEnum, texture: TextureEnum) -> Self {
        Object {
            shape,
            material,
            texture,
            index: 0,
        }
    }
}

impl Bounded for Object {
    fn aabb(&self) -> AABB {
        self.shape.aabb()
    }
}
impl BHShape for Object {
    fn set_bh_node_index(&mut self, index: usize) {
        self.index = index
    }

    fn bh_node_index(&self) -> usize {
        self.index
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

    fn simple_object() -> Object {
        let shape = Sphere::new(Point::new(5., 0., 0.), 1.);
        let material = UniformMaterial::new(
            LinearColor::new(0.5, 0.5, 0.5),
            LinearColor::new(1., 1., 1.),
            0.5,
        );
        let texture = UniformTexture::new(LinearColor::new(0.25, 0.5, 1.));
        Object::new(shape.into(), material.into(), texture.into())
    }

    #[test]
    fn new_works() {
        let shape = Sphere::new(Point::new(5., 0., 0.), 1.);
        let material = UniformMaterial::new(
            LinearColor::new(0.5, 0.5, 0.5),
            LinearColor::new(1., 1., 1.),
            0.5,
        );
        let texture = UniformTexture::new(LinearColor::new(0.25, 0.5, 1.));
        assert_eq!(
            simple_object(),
            Object {
                shape: shape.into(),
                material: material.into(),
                texture: texture.into(),
                index: 0,
            }
        )
    }

    #[test]
    fn deserialization_works() {
        let yaml = r#"
            shape:
              type: sphere
              inverted: false
              center: [5., 0.0, 0.0]
              radius: 1.0
            material:
              type: uniform
              diffuse: {r: 0.5, g: 0.5, b: 0.5}
              specular: {r: 1., g: 1., b: 1.}
              reflectivity: 0.5
            texture:
              type: uniform
              color: {r: 0.25, g: 0.5, b: 1.}
        "#;
        let object: Object = serde_yaml::from_str(yaml).unwrap();
        let expected = simple_object();
        assert_eq!(object, expected)
    }
}
