use super::super::Renderer;
use super::path::*;
use crate::scene::Scene;
use crate::{Point, Vector};
use image::RgbImage;
use nalgebra::Unit;

/// Render the [`Scene`] using Bidirectional-Pathtracing
///
/// [`Scene`]: ../scene/scene/struct.Scene.html
pub struct Pathtracer {
    #[allow(unused)]
    scene: Scene,
}

impl Pathtracer {
    /// Create a [`Pathtracer`] renderer with the given [`Scene`]
    ///
    /// [`Pathtracer`]: struct.Pathtracer.html
    /// [`Scene`]: ../scene/scene/struct.Scene.html
    pub fn new(scene: Scene) -> Self {
        Pathtracer { scene }
    }

    /// Render the [`Scene`] using Bidirectional-Pathtracing.
    ///
    /// [`Scene`]: ../scene/scene/struct.Scene.html
    pub fn render(&self) -> RgbImage {
        todo!()
    }

    fn construct_path(&self, point: Point, direction: Unit<Vector>) -> Path {
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

impl Renderer for Pathtracer {
    fn render(&self) -> RgbImage {
        self.render()
    }
}
