//! Define the different kinds of renderers for use on a given scene.
use image::RgbImage;

/// Each renderer implements this trait, to be called after being built.
pub trait Renderer {
    /// Render the [`Scene`] using the chosen rendering technique.
    ///
    /// [`Scene`]: ../scene/scene/struct.Scene.html
    fn render(&self) -> RgbImage;
}

mod bidirectional;
pub use bidirectional::*;

mod pathtrace;
pub use pathtrace::*;

mod raytrace;
pub use raytrace::*;

pub(crate) mod utils;
