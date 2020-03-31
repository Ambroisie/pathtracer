//! Scene representation.

use super::{LightAggregate, Mesh, Object};
use crate::core::{Camera, LinearColor};
use beevee::bvh::BVH;
use serde::Deserialize;

/// Represent the scene being rendered.
#[serde(from = "SerializedScene")]
#[derive(Debug, PartialEq, Deserialize)]
pub struct Scene {
    pub(crate) camera: Camera,
    pub(crate) lights: LightAggregate,
    pub(crate) objects: Vec<Object>,
    pub(crate) bvh: BVH,
    pub(crate) background: LinearColor,
    pub(crate) shot_rays: u32,
    pub(crate) reflection_limit: u32,
    pub(crate) diffraction_index: f32,
}

impl Scene {
    /// Creates a new `Scene`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pathtracer::core::{Camera, LightProperties, LinearColor};
    /// # use pathtracer::material::UniformMaterial;
    /// # use pathtracer::scene::{LightAggregate, Object, Scene};
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
    ///     LinearColor::black(), // Background color
    ///     5,   // amount of rays shot per pixel
    ///     3,   // reflection recursion limit
    ///     0.0, // diffraction index
    /// );
    /// ```
    pub fn new(
        camera: Camera,
        lights: LightAggregate,
        mut objects: Vec<Object>,
        background: LinearColor,
        shot_rays: u32,
        reflection_limit: u32,
        diffraction_index: f32,
    ) -> Self {
        let bvh = BVH::build(&mut objects);
        Scene {
            camera,
            lights,
            objects,
            bvh,
            background,
            shot_rays,
            reflection_limit,
            diffraction_index,
        }
    }
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
struct SerializedScene {
    camera: Camera,
    #[serde(default)]
    lights: LightAggregate,
    #[serde(default)]
    objects: Vec<Object>,
    #[serde(default)]
    meshes: Vec<Mesh>,
    #[serde(default)]
    background: LinearColor,
    #[serde(default)]
    shot_rays: u32,
    #[serde(default)]
    reflection_limit: u32,
    #[serde(default = "crate::serialize::default_identity")]
    starting_diffraction: f32,
}

impl From<SerializedScene> for Scene {
    fn from(mut scene: SerializedScene) -> Self {
        let mut flattened_meshes: Vec<Object> = scene
            .meshes
            .into_iter()
            .map(|m| m.shapes)
            .flatten()
            .collect();
        scene.objects.append(&mut flattened_meshes);

        Scene::new(
            scene.camera,
            scene.lights,
            scene.objects,
            scene.background,
            scene.shot_rays,
            scene.reflection_limit,
            scene.starting_diffraction,
        )
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
    fn empty_scene() {
        use crate::core::Camera;
        use crate::scene::{LightAggregate, Scene};

        let _scene = Scene::new(
            Camera::default(),
            LightAggregate::empty(),
            Vec::new(),           // Objects list
            LinearColor::black(), // Background color
            5,                    // aliasing limit
            3,                    // reflection recursion limit
            0.0,                  // diffraction index
        );
    }
}
