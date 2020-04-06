use super::super::utils::{buffer_to_image, prepare_buffer};
use super::super::Renderer;
use crate::core::LinearColor;
use crate::scene::{Object, Scene};
use beevee::ray::Ray;
use image::RgbImage;

/// Render the [`Scene`] using Pathtracing
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

    /// Render the [`Scene`] using Pathtracing.
    ///
    /// [`Scene`]: ../scene/scene/struct.Scene.html
    pub fn render(&self) -> RgbImage {
        let (width, height) = (
            self.scene.camera.film().width(),
            self.scene.camera.film().height(),
        );
        let total = width * height;
        let mut buffer = prepare_buffer(total);

        // (total passes, film)
        // FIXME: use MultiProgress because of rendering issues
        let (pa, pb) = super::super::progress::get_multiple_progress(total, self.scene.shot_rays);

        // Ensure at least one round of shots
        for _ in 0..self.scene.shot_rays.max(1) {
            pb.reset(); // We're rendering the whole film again, reset the pixel counter
            for y in 0..self.scene.camera.film().height() {
                for x in 0..self.scene.camera.film().width() {
                    let i = x + y * self.scene.camera.film().width();
                    buffer[i as usize] += self.pixel_ray(x as f32, y as f32);
                    pb.inc(1);
                }
            }
            pa.inc(1); // Increment the number of passes
        }

        pa.finish();
        pb.finish_and_clear();

        buffer_to_image(buffer, self.scene.shot_rays, width, height)
    }

    fn pixel_ray(&self, x: f32, y: f32) -> LinearColor {
        let (x, y) = self.scene.camera.film().pixel_ratio(x, y);
        let ray = self.scene.camera.ray_with_ratio(x, y);
        self.cast_ray(ray).map_or_else(
            || self.scene.background.clone(),
            |(t, obj)| {
                LinearColor::new(1., 1., 1.) // FIXME: calculate real color
            },
        )
    }

    fn cast_ray(&self, ray: Ray) -> Option<(f32, &Object)> {
        self.scene.bvh.walk(&ray, &self.scene.objects)
    }
}

impl Renderer for Pathtracer {
    fn render(&self) -> RgbImage {
        self.render()
    }
}
