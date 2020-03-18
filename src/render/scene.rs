use super::{light_aggregate::LightAggregate, object::Object};
use crate::{
    core::{Camera, LinearColor},
    material::{LightProperties, Material},
    shape::Shape,
    texture::Texture,
    {Point, Vector},
};
use bvh::{bvh::BVH, ray::Ray};
use image::RgbImage;
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
}

impl Scene {
    pub fn new(
        camera: Camera,
        lights: LightAggregate,
        mut objects: Vec<Object>,
        aliasing_limit: u32,
        reflection_limit: u32,
    ) -> Self {
        let bvh = BVH::build(&mut objects);
        Scene {
            camera,
            lights,
            objects,
            bvh,
            aliasing_limit,
            reflection_limit,
        }
    }

    /// Render the scene into an image.
    pub fn render(&self) -> RgbImage {
        let mut image = RgbImage::new(self.camera.film().width(), self.camera.film().height());
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
                        *pixel = pixel_func(&self, x as f32, y as f32).into()
                    }
                })
            }
        });
        image
    }

    /// Get pixel color for (x, y) a pixel **coordinate**
    fn pixel(&self, x: f32, y: f32) -> LinearColor {
        let (x, y) = self.camera.film().pixel_ratio(x, y);
        let pixel = self.camera.film().pixel_at_ratio(x, y);
        let direction = (pixel - self.camera.origin()).normalize();
        self.cast_ray(Ray::new(pixel, direction))
            .map_or_else(LinearColor::black, |(t, obj)| {
                self.color_at(pixel + direction * t, obj, direction, self.reflection_limit)
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
        // NOTE(Bruno): should be written using iterators
        let mut shot_obj: Option<&Object> = None;
        let mut t = std::f32::INFINITY;
        for object in self.bvh.traverse(&ray, &self.objects).iter() {
            match object.shape.intersect(&ray) {
                Some(dist) if dist < t => {
                    t = dist;
                    shot_obj = Some(&object);
                }
                _ => {}
            }
        }
        shot_obj.map(|obj| (t, obj))
    }

    fn color_at(
        &self,
        point: Point,
        object: &Object,
        incident_ray: Vector,
        reflection_limit: u32,
    ) -> LinearColor {
        let normal = object.shape.normal(&point);
        let reflected = reflected(incident_ray, normal);
        let texel = object.shape.project_texel(&point);

        let properties = object.material.properties(texel);
        let object_color = object.texture.texel_color(texel);

        self.illuminate(point, object_color, &properties, normal, reflected)
            + self.reflection(point, &properties, reflected, reflection_limit)
    }

    fn reflection(
        &self,
        point: Point,
        properties: &LightProperties,
        reflected: Vector,
        reflection_limit: u32,
    ) -> LinearColor {
        let reflectivity = properties.reflectivity;
        if reflectivity > 1e-5 && reflection_limit > 0 {
            let reflection_start = point + reflected * 0.001;
            if let Some((t, obj)) = self.cast_ray(Ray::new(reflection_start, reflected)) {
                let resulting_position = reflection_start + reflected * t;
                let color = self.color_at(resulting_position, obj, reflected, reflection_limit - 1);
                return color * reflectivity;
            }
        };
        LinearColor::black()
    }

    fn illuminate(
        &self,
        point: Point,
        object_color: LinearColor,
        properties: &LightProperties,
        normal: Vector,
        reflected: Vector,
    ) -> LinearColor {
        let ambient = self.illuminate_ambient(object_color);
        let spatial = self.illuminate_spatial(point, properties, normal, reflected);
        ambient + spatial
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
        normal: Vector,
        reflected: Vector,
    ) -> LinearColor {
        self.lights
            .spatial_lights_iter()
            .map(|light| {
                let (direction, t) = light.to_source(&point);
                let light_ray = Ray::new(point + 0.001 * direction, direction);
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

fn reflected(incident: Vector, normal: Vector) -> Vector {
    let proj = incident.dot(&normal);
    let delt = normal * (proj * 2.);
    incident - delt
}

#[derive(Debug, PartialEq, Deserialize)]
struct SerializedScene {
    camera: Camera,
    lights: LightAggregate,
    objects: Vec<Object>,
    aliasing_limit: u32,
    reflection_limit: u32,
}

impl From<SerializedScene> for Scene {
    fn from(scene: SerializedScene) -> Self {
        Scene::new(
            scene.camera,
            scene.lights,
            scene.objects,
            scene.aliasing_limit,
            scene.reflection_limit,
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
}
