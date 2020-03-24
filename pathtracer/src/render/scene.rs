//! Scene rendering logic

use super::{light_aggregate::LightAggregate, object::Object, utils::*};
use crate::{
    core::{Camera, LightProperties, LinearColor, ReflTransEnum},
    material::Material,
    shape::Shape,
    texture::Texture,
    {Point, Vector},
};
use beevee::{bvh::BVH, ray::Ray};
use image::RgbImage;
use nalgebra::Unit;
use rand::prelude::thread_rng;
use rand::Rng;
use serde::{Deserialize, Deserializer};

/// Represent the scene being rendered.
pub struct Scene {
    camera: Camera,
    lights: LightAggregate,
    objects: Vec<Object>,
    bvh: BVH,
    aliasing_limit: u32,
    reflection_limit: u32,
    diffraction_index: f32,
}

impl Scene {
    /// Creates a new `Scene`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pathtracer::core::{Camera, LightProperties, LinearColor};
    /// # use pathtracer::material::UniformMaterial;
    /// # use pathtracer::render::{LightAggregate, Object, Scene};
    /// # use pathtracer::shape::Sphere;
    /// # use pathtracer::texture::UniformTexture;
    /// # use pathtracer::Point;
    /// #
    /// let scene = Scene::new(
    ///     Camera::default(),
    ///     LightAggregate::empty(),
    ///     vec![
    ///         Object::new(
    ///             Sphere::new(Point::origin(), 1.0).into(),
    ///             UniformMaterial::new(
    ///                 LightProperties::new(
    ///                     LinearColor::new(1.0, 0.0, 0.0), // diffuse component
    ///                     LinearColor::new(0.0, 0.0, 0.0), // specular component
    ///                     None,
    ///                 ),
    ///             ).into(),
    ///             UniformTexture::new(LinearColor::new(0.5, 0.5, 0.5)).into(),
    ///         ),
    ///     ],
    ///     5,   // aliasing limit
    ///     3,   // reflection recursion limit
    ///     0.0, // diffraction index
    /// );
    /// ```
    pub fn new(
        camera: Camera,
        lights: LightAggregate,
        mut objects: Vec<Object>,
        aliasing_limit: u32,
        reflection_limit: u32,
        diffraction_index: f32,
    ) -> Self {
        // NOTE(Antoine): fun fact: BVH::build stack overflows when given an empty slice :)
        let bvh = BVH::build(&mut objects);
        Scene {
            camera,
            lights,
            objects,
            bvh,
            aliasing_limit,
            reflection_limit,
            diffraction_index,
        }
    }

    /// Render the scene into an image.
    pub fn render(&self) -> RgbImage {
        let mut image = RgbImage::new(self.camera.film().width(), self.camera.film().height());

        let total = (image.width() * image.height()) as u64;
        let pb = indicatif::ProgressBar::new(total);
        pb.set_draw_delta(total / 10000);
        pb.set_style(indicatif::ProgressStyle::default_bar().template(
            "{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {percent:>3}%: {pos}/{len} pixels (ETA: {eta})",
        ));

        let pixel_func = if self.aliasing_limit > 0 {
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
        let (x, y) = self.camera.film().pixel_ratio(x, y);
        let pixel = self.camera.film().pixel_at_ratio(x, y);
        let direction = Unit::new_normalize(pixel - self.camera.origin());
        let indices = RefractionInfo::with_index(self.diffraction_index);
        self.cast_ray(Ray::new(pixel, direction))
            .map_or_else(LinearColor::black, |(t, obj)| {
                self.color_at(
                    pixel + direction.as_ref() * t,
                    obj,
                    direction,
                    self.reflection_limit,
                    indices,
                )
            })
    }

    /// Get pixel color with anti-aliasing
    fn anti_alias_pixel(&self, x: f32, y: f32) -> LinearColor {
        let range = 0..self.aliasing_limit;
        let mut rng = thread_rng();
        let acc: LinearColor = range
            .map(|_| {
                let random_x: f32 = rng.gen();
                let random_y: f32 = rng.gen();
                self.pixel(x + random_x, y + random_y)
            })
            .map(LinearColor::clamp)
            .sum();
        acc / self.aliasing_limit as f32
    }

    fn cast_ray(&self, ray: Ray) -> Option<(f32, &Object)> {
        self.bvh
            .walk(&ray, &self.objects)
            .and_then(|o| o.shape.intersect(&ray).map(|t| (t, o)))
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
        self.lights
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
        self.lights
            .spatial_lights_iter()
            .map(|light| {
                let (direction, t) = light.to_source(&point);
                let light_ray = Ray::new(point + 0.001 * direction.as_ref(), direction);
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

#[derive(Debug, PartialEq, Deserialize)]
struct SerializedScene {
    camera: Camera,
    #[serde(default)]
    lights: LightAggregate,
    #[serde(default)]
    objects: Vec<Object>,
    #[serde(default)]
    aliasing_limit: u32,
    #[serde(default)]
    reflection_limit: u32,
    #[serde(default = "crate::serialize::default_identity")]
    starting_diffraction: f32,
}

impl From<SerializedScene> for Scene {
    fn from(scene: SerializedScene) -> Self {
        Scene::new(
            scene.camera,
            scene.lights,
            scene.objects,
            scene.aliasing_limit,
            scene.reflection_limit,
            scene.starting_diffraction,
        )
    }
}

impl<'de> Deserialize<'de> for Scene {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let cam: SerializedScene = Deserialize::deserialize(deserializer)?;
        Ok(cam.into())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn deserialization_works() {
        let yaml = std::include_str!("../../examples/scene.yaml");
        let _: Scene = serde_yaml::from_str(yaml).unwrap();
        // FIXME: actually test the equality ?
    }

    #[test]
    #[ignore] // stack overflow because of BVH :(
    fn bvh_fails() {
        use crate::core::Camera;
        use crate::render::{LightAggregate, Scene};

        let _scene = Scene::new(
            Camera::default(),
            LightAggregate::empty(),
            Vec::new(), // Objects list
            5,          // aliasing limit
            3,          // reflection recursion limit
            0.0,        // diffraction index
        );
    }
}
