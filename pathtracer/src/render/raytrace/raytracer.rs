use super::super::utils::*;
use super::super::Renderer;
use crate::scene::{Object, Scene};
use crate::{
    core::{LightProperties, LinearColor, ReflTransEnum},
    material::Material,
    shape::Shape,
    texture::Texture,
    {Point, Vector},
};
use beevee::ray::Ray;
use image::RgbImage;
use nalgebra::Unit;
use rand::prelude::thread_rng;
use rand::Rng;

/// Render the [`Scene`] using Raytracing.
///
/// [`Scene`]: ../scene/scene/struct.Scene.html
pub struct Raytracer {
    scene: Scene,
}

impl Raytracer {
    /// Create a [`Raytracer`] renderer with the given [`Scene`]
    ///
    /// [`Raytracer`]: struct.Raytracer.html
    /// [`Scene`]: ../scene/scene/struct.Scene.html
    pub fn new(scene: Scene) -> Self {
        Raytracer { scene }
    }

    /// Render the [`Scene`] using Raytracing.
    ///
    /// [`Scene`]: ../scene/scene/struct.Scene.html
    pub fn render(&self) -> RgbImage {
        let mut image = RgbImage::new(
            self.scene.camera.film().width(),
            self.scene.camera.film().height(),
        );

        let total = (image.width() * image.height()) as u64;
        let pb = super::super::progress::get_progressbar(total);

        let pixel_func = if self.scene.shot_rays > 0 {
            Self::anti_alias_pixel
        } else {
            Self::pixel
        };

        rayon::scope(|s| {
            // FIXME(Bruno): it would go even faster to cut the image in blocks of rows, leading to
            // better cache-line behaviour...
            for (_, row) in image.enumerate_rows_mut() {
                s.spawn(|_| {
                    for (x, y, pixel) in row {
                        *pixel = pixel_func(&self, x as f32, y as f32).into();
                        pb.inc(1);
                    }
                })
            }
        });

        pb.finish();
        image
    }

    /// Get pixel color for (x, y) a pixel **coordinate**
    fn pixel(&self, x: f32, y: f32) -> LinearColor {
        let (x, y) = self.scene.camera.film().pixel_ratio(x, y);
        let indices = RefractionInfo::with_index(self.scene.diffraction_index);
        let ray = self.scene.camera.ray_with_ratio(x, y);
        self.cast_ray(ray).map_or_else(
            || self.scene.background.clone(),
            |(t, obj)| {
                self.color_at(
                    ray.origin + ray.direction.as_ref() * t,
                    obj,
                    ray.direction,
                    self.scene.reflection_limit,
                    indices,
                )
            },
        )
    }

    /// Get pixel color with anti-aliasing
    fn anti_alias_pixel(&self, x: f32, y: f32) -> LinearColor {
        let range = 0..self.scene.shot_rays;
        let mut rng = thread_rng();
        let acc: LinearColor = range
            .map(|_| {
                let random_x: f32 = rng.gen();
                let random_y: f32 = rng.gen();
                self.pixel(x + random_x, y + random_y)
            })
            .map(LinearColor::clamp)
            .sum();
        acc / self.scene.shot_rays as f32
    }

    fn cast_ray(&self, ray: Ray) -> Option<(f32, &Object)> {
        self.scene.bvh.walk(&ray, &self.scene.objects)
    }

    fn color_at(
        &self,
        point: Point,
        object: &Object,
        incident_ray: Unit<Vector>,
        reflection_limit: u32,
        mut indices: RefractionInfo,
    ) -> LinearColor {
        let texel = object.shape.project_texel(&point);
        let properties = object.material.properties(texel);
        let object_color = object.texture.texel_color(texel);

        let normal = object.shape.normal(&point);
        let reflected_ray = reflected(incident_ray, normal);

        // FIXME: change this to averaged sampled rays instead of visiting every light ?
        // Indeed the path-tracing algorithm is good for calculating the radiance at a point
        // But it should be used for reflection and refraction too...
        let lighting = self.illuminate(point, object_color, &properties, normal, reflected_ray);
        if properties.refl_trans.is_none() {
            // Avoid calculating reflection when not needed
            return lighting;
        }
        let reflected = self.reflection(point, reflected_ray, reflection_limit, indices.clone());
        // We can unwrap safely thanks to the check for None before
        match properties.refl_trans.unwrap() {
            ReflTransEnum::Transparency { coef, index } => {
                // Calculate the refracted ray, if it was refracted, and mutate indices accordingly
                refracted(incident_ray, normal, &mut indices, index).map_or_else(
                    // Total reflection
                    || reflected.clone(),
                    // Refraction (refracted ray, amount of *reflection*)
                    |(r, refl_t)| {
                        let refracted = self.refraction(point, coef, r, reflection_limit, indices);
                        let refr_light = refracted * (1. - refl_t) + reflected.clone() * refl_t;
                        refr_light * coef + lighting * (1. - coef)
                    },
                )
            }
            ReflTransEnum::Reflectivity { coef } => reflected * coef + lighting * (1. - coef),
        }
    }

    fn refraction(
        &self,
        point: Point,
        transparency: f32,
        refracted: Unit<Vector>,
        reflection_limit: u32,
        indices: RefractionInfo,
    ) -> LinearColor {
        if transparency > 1e-5 && reflection_limit > 0 {
            let refraction_start = point + refracted.as_ref() * 0.001;
            if let Some((t, obj)) = self.cast_ray(Ray::new(refraction_start, refracted)) {
                let resulting_position = refraction_start + refracted.as_ref() * t;
                let refracted = self.color_at(
                    resulting_position,
                    obj,
                    refracted,
                    reflection_limit - 1,
                    indices,
                );
                return refracted * transparency;
            }
        }
        LinearColor::black()
    }

    fn reflection(
        &self,
        point: Point,
        reflected: Unit<Vector>,
        reflection_limit: u32,
        indices: RefractionInfo,
    ) -> LinearColor {
        if reflection_limit > 0 {
            let reflection_start = point + reflected.as_ref() * 0.001;
            if let Some((t, obj)) = self.cast_ray(Ray::new(reflection_start, reflected)) {
                let resulting_position = reflection_start + reflected.as_ref() * t;
                let color = self.color_at(
                    resulting_position,
                    obj,
                    reflected,
                    reflection_limit - 1,
                    indices,
                );
                return color;
            }
        };
        LinearColor::black()
    }

    fn illuminate(
        &self,
        point: Point,
        object_color: LinearColor,
        properties: &LightProperties,
        normal: Unit<Vector>,
        reflected: Unit<Vector>,
    ) -> LinearColor {
        let ambient = self.illuminate_ambient(object_color.clone());
        let spatial = self.illuminate_spatial(point, properties, normal, reflected);
        ambient + object_color * spatial
    }

    fn illuminate_ambient(&self, color: LinearColor) -> LinearColor {
        self.scene
            .lights
            .ambient_lights_iter()
            .map(|light| color.clone() * light.illumination(&Point::origin()))
            .map(LinearColor::clamp)
            .sum()
    }

    fn illuminate_spatial(
        &self,
        point: Point,
        properties: &LightProperties,
        normal: Unit<Vector>,
        reflected: Unit<Vector>,
    ) -> LinearColor {
        self.scene
            .lights
            .spatial_lights_iter()
            .map(|light| {
                let (direction, t) = light.to_source(&point);
                let light_ray = Ray::new(point + direction.as_ref() * 0.001, direction);
                match self.cast_ray(light_ray) {
                    // Take shadows into account
                    Some((obstacle_t, _)) if obstacle_t < t => return LinearColor::black(),
                    _ => {}
                }
                let lum = light.illumination(&point);
                let diffused = properties.diffuse.clone() * normal.dot(&direction);
                let specular = properties.specular.clone() * reflected.dot(&direction);
                lum * (diffused + specular)
            })
            .map(LinearColor::clamp)
            .sum()
    }
}

impl Renderer for Raytracer {
    fn render(&self) -> RgbImage {
        self.render()
    }
}
