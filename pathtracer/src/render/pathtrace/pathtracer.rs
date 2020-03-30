use super::super::Renderer;
use crate::scene::Scene;
use image::RgbImage;

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
}

impl Renderer for Pathtracer {
    fn render(&self) -> RgbImage {
        self.render()
    }
}
