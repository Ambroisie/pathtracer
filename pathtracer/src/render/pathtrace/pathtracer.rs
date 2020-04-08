use indicatif::ProgressIterator;
use rayon::prelude::*;

use super::super::utils::{buffer_to_image, sample_hemisphere};
use super::super::Renderer;
use crate::{
    core::LinearColor,
    material::Material,
    scene::{Object, Scene},
    shape::Shape,
};
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

        let p = super::super::progress::get_passes_progressbar(self.scene.shot_rays);

        // Ensure at least one round of shots
        let img_buf = (0..self.scene.shot_rays.max(1))
            .progress_with(p)
            .map(|_| {
                let mut buffer: Vec<LinearColor> = Vec::new();
                buffer.resize_with(total as usize, LinearColor::black);

                (0..height)
                    .into_par_iter()
                    .map(|y| {
                        let mut row: Vec<LinearColor> = Vec::new();
                        row.resize_with(width as usize, LinearColor::black);

                        for x in 0..width {
                            row[x as usize] += self.pixel_ray(x as f32, y as f32);
                        }

                        row
                    })
                    .reduce(
                        || Vec::new(),
                        |mut buf, row| {
                            buf.extend(row);
                            buf
                        },
                    )
            })
            .fold(
                {
                    let mut vec = Vec::new();
                    vec.resize_with(total as usize, LinearColor::black);
                    vec
                },
                |mut acc, buf| {
                    for (i, pixel) in buf.into_iter().enumerate() {
                        acc[i] += pixel;
                    }

                    acc
                },
            );

        buffer_to_image(img_buf, self.scene.shot_rays, width, height)
    }

    fn pixel_ray(&self, x: f32, y: f32) -> LinearColor {
        let (x, y) = self.scene.camera.film().pixel_ratio(x, y);
        let ray = self.scene.camera.ray_with_ratio(x, y);
        self.cast_ray(ray).map_or_else(
            || self.scene.background.clone(),
            |(t, obj)| self.radiance(ray, t, obj, self.scene.reflection_limit),
        )
    }

    fn radiance(&self, ray: Ray, t: f32, obj: &Object, limit: u32) -> LinearColor {
        // This doesn't look great, but it works ¯\_(ツ)_/¯

        let hit_pos = ray.origin + ray.direction.as_ref() * t;
        let texel = obj.shape.project_texel(&hit_pos);
        let properties = obj.material.properties(texel);
        // If we are the at recursion limit, return the light emitted by the object
        if limit == 0 {
            return properties.emitted;
        };
        // Get BRDF
        // FIXME: what about the material's albedo ?
        let brdf = properties.diffuse;
        // Pick a new direction
        let normal = obj.shape.normal(&hit_pos);
        let (new_direction, weight) = sample_hemisphere(normal);
        let cos_new_ray = new_direction.dot(&normal);
        // Calculate the incoming light along the new ray
        let new_ray = Ray::new(hit_pos + new_direction.as_ref() * 0.001, new_direction);
        let incoming = self
            .cast_ray(new_ray)
            .map_or_else(LinearColor::black, |(t, obj)| {
                self.radiance(new_ray, t, obj, limit - 1)
            });
        // Put it all together
        properties.emitted + (brdf * incoming * cos_new_ray * weight)
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
