use crate::material::Material;
use crate::shape::Shape;
use crate::texture::Texture;

/// An object being rendered in the scene.
#[derive(Debug)]
pub struct Object<'a> {
    pub shape: &'a dyn Shape,
    pub material: &'a dyn Material,
    pub texture: &'a dyn Texture,
}

impl<'a> Object<'a> {
    #[allow(dead_code)] // FIXME: remove this when used
    pub fn new(shape: &'a dyn Shape, material: &'a dyn Material, texture: &'a dyn Texture) -> Self {
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
    use crate::*;
    use bvh::{
        aabb::{Bounded, AABB},
        ray::Ray,
    };

    /// NOTE(Bruno): those dummy implementations could be used somewhere else ?

    #[derive(Debug)]
    struct DummyShape {}

    impl Bounded for DummyShape {
        fn aabb(&self) -> AABB {
            todo!()
        }
    }

    impl Shape for DummyShape {
        /// Return the distance at which the object intersects with the ray, or None if it does not.
        fn intersect(&self, _: &Ray) -> Option<f32> {
            todo!()
        }
        /// Return the unit vector corresponding to the normal at this point of the shape.
        fn normal(&self, _: &Point) -> Vector {
            todo!()
        }
        /// Project the point from the shape's surface to its texel coordinates.
        fn project_texel(&self, _: &Point) -> Point2D {
            todo!()
        }
    }

    #[derive(Debug)]
    struct DummyMaterial {}

    impl Material for DummyMaterial {
        fn diffuse(&self, _: Point2D) -> LinearColor {
            todo!()
        }
        fn specular(&self, _: Point2D) -> LinearColor {
            todo!()
        }
    }

    #[derive(Debug)]
    struct DummyTexture {}

    impl Texture for DummyTexture {
        fn texel_color(&self, _: Point2D) -> LinearColor {
            todo!()
        }
    }

    #[test]
    fn new_works() {
        let shape = DummyShape {};
        let material = DummyMaterial {};
        let texture = DummyTexture {};
        let _ = Object::new(&shape, &material, &texture);
        // Can't compare the results... Just make sure the new compiles.
    }
}
