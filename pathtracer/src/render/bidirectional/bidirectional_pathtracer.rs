use super::super::Renderer;
use super::path::*;
use crate::core::LinearColor;
use crate::material::Material;
use crate::render::utils::{buffer_to_image, sample_hemisphere};
use crate::scene::{Object, Scene};
use crate::shape::Shape;
use crate::{Point, Vector};
use beevee::ray::Ray;
use image::RgbImage;
use indicatif::ProgressIterator;
use nalgebra::Unit;
use rayon::prelude::*;

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
        let (width, height) = (
            self.scene.camera.film().width(),
            self.scene.camera.film().height(),
        );
        let total = width * height;

        let p = super::super::progress::get_passes_progressbar(self.scene.shot_rays);

        let (img_buf, _) = (0..self.scene.shot_rays.max(1))
            .progress_with(p)
            .map(|_| {
                let mut buffer: Vec<LinearColor> = Vec::new();
                buffer.resize_with(total as usize, LinearColor::black);

                buffer
                    .par_chunks_mut(width as usize)
                    .enumerate()
                    .for_each(|(y, row)| {
                        for x in 0..width {
                            row[x as usize] = self.pixel_ray(x as f32, y as f32);
                        }
                    });

                buffer
            })
            .fold(
                {
                    let mut vec = Vec::new();
                    vec.resize_with(total as usize, LinearColor::black);
                    let count = 0usize;
                    (vec, count)
                },
                |(mut acc, count), buf| {
                    for (i, pixel) in buf.into_iter().enumerate() {
                        acc[i] += pixel;
                    }

                    let count = count + 1; // Because count is 0-indexed
                    if self.scene.steps.contains(&count) {
                        let image = buffer_to_image(&acc, count as u32, width, height);
                        image
                            .save(format!("{}_passes.png", count))
                            .expect("writing image failed!");
                    }

                    (acc, count) // Count has been updated previously
                },
            );

        buffer_to_image(&img_buf, self.scene.shot_rays, width, height)
    }

    fn pixel_ray(&self, x: f32, y: f32) -> LinearColor {
        let light_paths = self
            .scene
            .lights
            .sample_lights_iter()
            .map(|l| {
                let light_ray = l.sample_ray();
                self.construct_light_path(light_ray.origin, light_ray.direction, l.luminance())
            })
            .collect::<Vec<_>>();

        let (x, y) = self.scene.camera.film().pixel_ratio(x, y);
        let ray = self.scene.camera.ray_with_ratio(x, y);

        self.cast_ray(ray).map_or_else(
            || self.scene.background.clone(),
            |(t, obj)| self.radiance(ray, t, obj, &light_paths, self.scene.reflection_limit),
        )
    }

    fn radiance(
        &self,
        ray: Ray,
        t: f32,
        obj: &Object,
        light_paths: &[Path],
        limit: u32,
    ) -> LinearColor {
        let hit_pos = ray.origin + ray.direction.as_ref() * t;
        let texel = obj.shape.project_texel(&hit_pos);
        let properties = obj.material.properties(texel);

        let mut light_samples = LinearColor::black();
        for path in light_paths {
            for point in &path.points {
                light_samples += point.luminance.clone() / (hit_pos - point.point).norm();
            }
        }

        if limit == 0 {
            return properties.emitted;
        }

        let brdf = properties.diffuse;

        let normal = obj.shape.normal(&hit_pos);
        let new_direction = sample_hemisphere(normal);

        let new_ray = Ray::new(hit_pos + new_direction.as_ref() * 0.001, new_direction);
        let incoming = self
            .cast_ray(new_ray)
            .map_or_else(LinearColor::black, |(t, obj)| {
                self.radiance(new_ray, t, obj, light_paths, limit - 1)
            });

        light_samples + properties.emitted + (brdf * incoming)
    }

    #[allow(unused)]
    fn construct_light_path(
        &self,
        mut origin: Point,
        mut direction: Unit<Vector>,
        luminance: LinearColor,
    ) -> Path {
        let mut res = Path::new(origin);
        let mut previous_luminance = luminance.clone();

        let light_point = PathPoint::new(origin, luminance);
        res.push_point(light_point);

        for _ in 0..self.scene.reflection_limit {
            let ray = Ray::new(origin, direction);
            match self.cast_ray(ray) {
                Some((distance, obj)) => {
                    let hit_pos = origin + direction.as_ref() * distance;
                    let texel = obj.shape.project_texel(&hit_pos);
                    let properties = obj.material.properties(texel);
                    let emitted = properties.emitted;
                    let diffuse = properties.diffuse;
                    let normal = obj.shape.normal(&hit_pos);

                    let luminance = emitted + (diffuse * (previous_luminance / distance));

                    let p = PathPoint::new(hit_pos, luminance.clone());
                    res.push_point(p);

                    let new_direction = sample_hemisphere(normal);
                    // Calculate the incoming light along the new ray
                    origin = hit_pos + new_direction.as_ref() * 0.001;
                    direction = new_direction;
                    previous_luminance = luminance;
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
