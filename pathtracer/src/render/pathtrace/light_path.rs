use crate::core::{LightProperties, LinearColor};
use crate::light::SampleLight;
use crate::Point;

pub struct LightPathPoint {
    pub point: Point,
    pub luminance: LinearColor,
    pub properties: LightProperties,
}

impl LightPathPoint {
    pub fn new(point: Point, luminance: LinearColor, properties: LightProperties) -> Self {
        LightPathPoint {
            point,
            luminance,
            properties,
        }
    }
}

pub struct LightPath<'a> {
    pub origin: &'a dyn SampleLight,
    pub points: Vec<LightPathPoint>,
}

impl<'a> LightPath<'a> {
    pub fn new(origin: &'a dyn SampleLight) -> Self {
        LightPath {
            origin,
            points: Vec::new(),
        }
    }

    pub fn push_point(&mut self, new_point: LightPathPoint) {
        self.points.push(new_point)
    }
}
