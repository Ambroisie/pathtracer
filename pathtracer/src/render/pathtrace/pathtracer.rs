use super::super::utils::*;
use super::super::Renderer;
use super::light_path::{LightPath, LightPathPoint};
use crate::core::{LightProperties, LinearColor};
use crate::light::SampleLight;
use crate::material::Material;
use crate::scene::object::Object;
use crate::scene::Scene;
use crate::shape::Shape;
use crate::{Point, Vector};
use beevee::ray::Ray;
use image::RgbImage;
use nalgebra::Unit;
use rand::Rng;

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

    fn cast_ray(&self, ray: Ray) -> Option<(f32, &Object)> {
        self.scene.bvh.walk(&ray, &self.scene.objects)
    }

    fn construct_light_path(&self) -> LightPath {
        let mut rng = rand::thread_rng();
        let num_lights = self.scene.lights.points.len() + self.scene.lights.spots.len();
        let index = rng.gen_range(0, num_lights);

        let sample_light: &dyn SampleLight = if index < self.scene.lights.points.len() {
            &self.scene.lights.points[index]
        } else {
            &self.scene.lights.spots[index - self.scene.lights.points.len()]
        };

        let mut ray = sample_light.sample_ray();
        let mut res = LightPath::new(sample_light);

        if let Some((dist, obj)) = self.cast_ray(ray) {
            let hit_pos = ray.origin + ray.direction.as_ref() * dist;
            let texel = obj.shape.project_texel(&hit_pos);
            let new_point = LightPathPoint::new(
                hit_pos,
                sample_light.illumination(&hit_pos),
                obj.material.properties(texel),
            );
            res.push_point(new_point);
            ray = todo!(); // Sample new direction
        } else {
            return res;
        };

        for _ in 1..self.scene.reflection_limit {
            if let Some((dist, obj)) = self.cast_ray(ray) {
                let new_point = todo!();
                res.push_point(new_point);
            } else {
                break;
            }
        }
        res
    }

    fn illuminate(
        &self,
        point: Point,
        properties: LightProperties,
        path: LightPath,
    ) -> LinearColor {
        path.points.iter().map(|p| p.luminance.clone()).sum()
    }
}

impl Renderer for Pathtracer {
    fn render(&self) -> RgbImage {
        self.render()
    }
}
