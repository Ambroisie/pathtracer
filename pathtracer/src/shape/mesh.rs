use super::{InterpolatedTriangle, Shape, Triangle};
use crate::material::{Material, TriangleMaterial, UniformMaterial};
use crate::texture::{Texture, TriangleTexture, UniformTexture};
use crate::Point;
use beevee::{
    aabb::{Bounded, AABB},
    bvh::Intersected,
    ray::Ray,
};
