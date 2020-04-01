use super::super::Renderer;
use super::path::*;
use crate::scene::Scene;
use crate::{Point, Vector};
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
    fn construct_path(&self, point: Point, _direction: Unit<Vector>) -> Path {
        let mut res = Path::new(point);
        for _ in 0..self.scene.reflection_limit {
            // FIXME:
            //  * cast_ray: if no intersection, return the empty path
            //  * look-up information at intersection
            //  * append to path
            //  * start again with new origin
        }
        res
    }
}

impl Renderer for BidirectionalPathtracer {
    fn render(&self) -> RgbImage {
        self.render()
    }
}
