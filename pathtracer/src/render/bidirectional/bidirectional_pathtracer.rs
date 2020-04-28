use super::super::Renderer;
use super::path::*;
use crate::material::Material;
use crate::render::utils::sample_hemisphere;
use crate::scene::{Object, Scene};
use crate::shape::Shape;
use crate::{Point, Vector};
use beevee::ray::Ray;
use image::RgbImage;
use nalgebra::Unit;

/// Render the [`Scene`] using Bidirectional-Pathtracing
///
/// [`Scene`]: ../scene/scene/struct.Scene.html
pub struct BidirectionalPathtracer {
    #[allow(unused)]
    scene: Scene,
}

impl BidirectionalPathtracer {
    /// Create a [`BidirectionalPathtracer`] renderer with the given [`Scene`]
    ///
    /// [`BidirectionalPathtracer`]: struct.BidirectionalPathtracer.html
    /// [`Scene`]: ../scene/scene/struct.Scene.html
    pub fn new(scene: Scene) -> Self {
        BidirectionalPathtracer { scene }
    }

    /// Render the [`Scene`] using Bidirectional-Pathtracing.
    ///
    /// [`Scene`]: ../scene/scene/struct.Scene.html
    pub fn render(&self) -> RgbImage {
        todo!()
    }

    #[allow(unused)]
    fn construct_path(&self, mut origin: Point, mut direction: Unit<Vector>) -> Path {
        let mut res = Path::new(origin);
        for _ in 0..self.scene.reflection_limit {
            let ray = Ray::new(origin, direction);
            match self.cast_ray(ray) {
                Some((distance, obj)) => {
                    let hit_pos = origin + direction.as_ref() * distance;
                    let texel = obj.shape.project_texel(&hit_pos);
                    let properties = obj.material.properties(texel);
                    let normal = obj.shape.normal(&hit_pos);
                    let p = PathPoint::new(origin, direction, normal, properties);

                    res.push_point(p);

                    let (new_direction, _) = sample_hemisphere(normal);
                    // Calculate the incoming light along the new ray
                    origin = hit_pos + new_direction.as_ref() * 0.001;
                    direction = new_direction;
                }
                None => break,
            }
        }
        res
    }

    #[allow(unused)]
    fn cast_ray(&self, ray: Ray) -> Option<(f32, &Object)> {
        self.scene.bvh.walk(&ray, &self.scene.objects)
    }
}

impl Renderer for BidirectionalPathtracer {
    fn render(&self) -> RgbImage {
        self.render()
    }
}
