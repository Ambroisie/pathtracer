//! Logic for the scene objects

use crate::material::MaterialEnum;
use crate::shape::ShapeEnum;
use crate::texture::TextureEnum;
use crate::Point;
use beevee::{
    aabb::{Bounded, AABB},
    bvh::Intersected,
    ray::Ray,
};
use serde::Deserialize;

/// An object being rendered in the scene.
#[derive(Debug, PartialEq, Deserialize)]
pub struct Object {
    /// The `Object`'s physical shape
    pub shape: ShapeEnum,
    /// The `Object`'s material
    pub material: MaterialEnum,
    /// The `Object`'s texture
    pub texture: TextureEnum,
}

impl Object {
    /// Creates a new `Object`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pathtracer::core::{LightProperties, LinearColor};
    /// # use pathtracer::material::UniformMaterial;
    /// # use pathtracer::render::Object;
    /// # use pathtracer::shape::Sphere;
    /// # use pathtracer::texture::UniformTexture;
    /// # use pathtracer::Point;
    /// #
    /// let obj = Object::new(
    ///     Sphere::new(Point::origin(), 1.0).into(),
    ///     UniformMaterial::new(
    ///         LightProperties::new(
    ///             LinearColor::new(1.0, 0.0, 0.0), // diffuse component
    ///             LinearColor::new(0.0, 0.0, 0.0), // specular component
    ///             None,
    ///         ),
    ///     ).into(),
    ///     UniformTexture::new(LinearColor::new(0.5, 0.5, 0.5)).into(),
    /// );
    /// ```
    pub fn new(shape: ShapeEnum, material: MaterialEnum, texture: TextureEnum) -> Self {
        Object {
            shape,
            material,
            texture,
        }
    }
}

impl Bounded for Object {
    fn aabb(&self) -> AABB {
        self.shape.aabb()
    }

    fn centroid(&self) -> Point {
        self.shape.centroid()
    }
}

impl Intersected for Object {
    fn intersect(&self, ray: &Ray) -> Option<f32> {
        self.shape.intersect(ray)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::core::color::LinearColor;
    use crate::core::LightProperties;
    use crate::material::UniformMaterial;
    use crate::shape::Sphere;
    use crate::texture::UniformTexture;

    fn simple_object() -> Object {
        let shape = Sphere::new(Point::new(5., 0., 0.), 1.);
        let material = UniformMaterial::new(LightProperties::new(
            LinearColor::new(0.5, 0.5, 0.5),
            LinearColor::new(1., 1., 1.),
            None,
        ));
        let texture = UniformTexture::new(LinearColor::new(0.25, 0.5, 1.));
        Object::new(shape.into(), material.into(), texture.into())
    }

    #[test]
    fn new_works() {
        let shape = Sphere::new(Point::new(5., 0., 0.), 1.);
        let material = UniformMaterial::new(LightProperties::new(
            LinearColor::new(0.5, 0.5, 0.5),
            LinearColor::new(1., 1., 1.),
            None,
        ));
        let texture = UniformTexture::new(LinearColor::new(0.25, 0.5, 1.));
        assert_eq!(
            simple_object(),
            Object {
                shape: shape.into(),
                material: material.into(),
                texture: texture.into(),
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
            texture:
              type: uniform
              color: {r: 0.25, g: 0.5, b: 1.}
        "#;
        let object: Object = serde_yaml::from_str(yaml).unwrap();
        let expected = simple_object();
        assert_eq!(object, expected)
    }
}
