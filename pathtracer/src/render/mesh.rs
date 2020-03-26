use std::convert::TryFrom;
use std::path::PathBuf;

use nalgebra::Unit;

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
}

impl TryFrom<Wavefront> for Mesh {
    type Error = tobj::LoadError;

    fn try_from(wavefront: Wavefront) -> Result<Mesh, Self::Error> {
        let mut shapes = Vec::new();

        let (models, materials) = load_obj(&wavefront.obj_file)?;

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

                // FIXME: world-to-object transformations needed
                let pos_a = Point::from_slice(&mesh.positions[(a * 3)..(a * 3 + 2)]);
                let pos_b = Point::from_slice(&mesh.positions[(b * 3)..(b * 3 + 2)]);
                let pos_c = Point::from_slice(&mesh.positions[(c * 3)..(c * 3 + 2)]);

                let triangle: ShapeEnum = if mesh.normals.is_empty() {
                    Triangle::new(pos_a, pos_b, pos_c).into()
                } else {
                    let norm_a = Unit::new_normalize(Vector::new(
                        mesh.normals[a * 3],
                        mesh.normals[a * 3 + 1],
                        mesh.normals[a * 3 + 2],
                    ));
                    let norm_b = Unit::new_normalize(Vector::new(
                        mesh.normals[b * 3],
                        mesh.normals[b * 3 + 1],
                        mesh.normals[b * 3 + 2],
                    ));
                    let norm_c = Unit::new_normalize(Vector::new(
                        mesh.normals[c * 3],
                        mesh.normals[c * 3 + 1],
                        mesh.normals[c * 3 + 2],
                    ));

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
