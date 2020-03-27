use std::convert::TryFrom;
use std::path::PathBuf;

use nalgebra::{Similarity3, Unit, VectorSlice3};

use serde::Deserialize;

use tobj::{self, load_obj};

use super::Object;
use crate::{
    core::{LightProperties, LinearColor},
    material::{MaterialEnum, UniformMaterial},
    shape::{InterpolatedTriangle, ShapeEnum, Triangle},
    texture::{TextureEnum, UniformTexture},
    Point, Vector,
};

/// Represent a mesh of objects.
#[serde(try_from = "Wavefront")]
#[derive(Debug, PartialEq, Deserialize)]
pub struct Mesh {
    /// The shapes composing the mesh
    pub(crate) shapes: Vec<Object>,
}

#[derive(Debug, PartialEq, Deserialize)]
pub(crate) struct Wavefront {
    pub obj_file: PathBuf,
    #[serde(default = "nalgebra::zero")]
    translation: Vector,
    #[serde(default = "nalgebra::zero")]
    rotation: Vector,
    #[serde(default = "crate::serialize::coefficient::default_identity")]
    scale: f32,
}

impl TryFrom<Wavefront> for Mesh {
    type Error = tobj::LoadError;

    fn try_from(wavefront: Wavefront) -> Result<Mesh, Self::Error> {
        let mut shapes = Vec::new();

        let (models, materials) = load_obj(&wavefront.obj_file)?;

        // The object to world transformation matrix
        let transform =
            Similarity3::new(wavefront.translation, wavefront.rotation, wavefront.scale);

        for model in models {
            let mesh = &model.mesh;

            // mesh.indices contains all vertices. Each group of 3 vertices
            // is a triangle, so we iterate over indices 3 by 3.
            for i in 0..(mesh.indices.len() / 3) {
                let (a, b, c) = (
                    mesh.indices[i * 3] as usize,
                    mesh.indices[i * 3 + 1] as usize,
                    mesh.indices[i * 3 + 2] as usize,
                );

                let pos_a = transform * Point::from_slice(&mesh.positions[(a * 3)..(a * 3 + 3)]);
                let pos_b = transform * Point::from_slice(&mesh.positions[(b * 3)..(b * 3 + 3)]);
                let pos_c = transform * Point::from_slice(&mesh.positions[(c * 3)..(c * 3 + 3)]);

                let triangle: ShapeEnum = if mesh.normals.is_empty() {
                    Triangle::new(pos_a, pos_b, pos_c).into()
                } else {
                    // We apply the (arguably useless) scaling to the vectors in case it is
                    // negative, which would invert their direction
                    let norm_a = {
                        let vec: Vector =
                            VectorSlice3::from_slice(&mesh.normals[(a * 3)..(a * 3 + 3)]).into();
                        Unit::new_normalize(transform * vec)
                    };
                    let norm_b = {
                        let vec: Vector =
                            VectorSlice3::from_slice(&mesh.normals[(b * 3)..(b * 3 + 3)]).into();
                        Unit::new_normalize(transform * vec)
                    };
                    let norm_c = {
                        let vec: Vector =
                            VectorSlice3::from_slice(&mesh.normals[(c * 3)..(c * 3 + 3)]).into();
                        Unit::new_normalize(transform * vec)
                    };

                    InterpolatedTriangle::new(pos_a, pos_b, pos_c, norm_a, norm_b, norm_c).into()
                };

                // FIXME: handle material
                let (material, texture): (MaterialEnum, TextureEnum) =
                    if let Some(mat_id) = mesh.material_id {
                        let mesh_mat = &materials[mat_id];

                        let diffuse = LinearColor::from_slice(&mesh_mat.ambient[..]);
                        let specular = LinearColor::from_slice(&mesh_mat.ambient[..]);

                        let material = UniformMaterial::new(LightProperties::new(
                            diffuse.clone(),
                            specular,
                            // FIXME: material.dissolve is supposed to be "the alpha term"
                            // Needs translation to our ReflTransEnum
                            None,
                        ));

                        // we only handle uniform textures
                        let texture = UniformTexture::new(diffuse);

                        (material.into(), texture.into())
                    } else {
                        // FIXME: should we accept this, and use a default
                        // Material, or throw a LoadError
                        (
                            UniformMaterial::new(LightProperties::new(
                                LinearColor::new(0.5, 0.5, 0.5),
                                LinearColor::new(0.1, 0.1, 0.1),
                                None,
                            ))
                            .into(),
                            UniformTexture::new(LinearColor::new(0.5, 0.5, 0.5)).into(),
                        )
                    };

                shapes.push(Object::new(triangle, material, texture));
            }
        }

        Ok(Mesh { shapes })
    }
}
