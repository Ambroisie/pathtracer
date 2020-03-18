use crate::light::*;
use serde::Deserialize;
use std::iter::Iterator;

#[derive(Debug, PartialEq, Deserialize)]
pub struct LightAggregate {
    ambients: Vec<AmbientLight>,
    directionals: Vec<DirectionalLight>,
    points: Vec<PointLight>,
    spots: Vec<SpotLight>,
}

impl LightAggregate {
    pub fn empty() -> Self {
        LightAggregate::new(vec![], vec![], vec![], vec![])
    }

    pub fn new(
        ambients: Vec<AmbientLight>,
        directionals: Vec<DirectionalLight>,
        points: Vec<PointLight>,
        spots: Vec<SpotLight>,
    ) -> Self {
        LightAggregate {
            ambients,
            directionals,
            points,
            spots,
        }
    }

    pub fn ambient_lights_iter(&self) -> impl Iterator<Item = &'_ dyn Light> {
        self.ambients.iter().map(|l| l as &dyn Light)
    }

    pub fn spatial_lights_iter(&self) -> impl Iterator<Item = &'_ dyn SpatialLight> {
        self.directionals
            .iter()
            .map(|l| l as &dyn SpatialLight)
            .chain(self.points.iter().map(|l| l as &dyn SpatialLight))
            .chain(self.spots.iter().map(|l| l as &dyn SpatialLight))
    }
}

impl Default for LightAggregate {
    fn default() -> Self {
        LightAggregate::empty()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn empty_works() {
        let lights = LightAggregate::empty();
        assert_eq!(
            lights,
            LightAggregate {
                ambients: vec![],
                directionals: vec![],
                points: vec![],
                spots: vec![],
            }
        )
    }

    #[test]
    fn default_is_empty() {
        let lights = <LightAggregate as Default>::default();
        assert_eq!(lights, LightAggregate::empty())
    }

    #[test]
    fn deserialization_works() {
        use crate::{core::LinearColor, Point, Vector};

        let yaml = r#"
            ambients:
              - color: {r: 1.0, g: 0.5, b: 0.2}
            directionals:
              - direction: [1.0, 0.0, 0.0]
                color: {r: 1.0, g: 0.5, b: 0.2}
            points:
              - position: [1.0, 1.0, 1.0]
                color: {r: 1.0, g: 0.5, b: 0.2}
            spots:
              - position: [0.0, 0.0, 0.0]
                direction: [1.0, 0.0, 0.0]
                fov: 90.0
                color: {r: 1.0, g: 0.5, b: 0.2}
        "#;
        let expected = LightAggregate::new(
            vec![AmbientLight::new(LinearColor::new(1., 0.5, 0.2))],
            vec![DirectionalLight::new(
                Vector::new(1., 0., 0.),
                LinearColor::new(1., 0.5, 0.2),
            )],
            vec![PointLight::new(
                Point::new(1., 1., 1.),
                LinearColor::new(1., 0.5, 0.2),
            )],
            vec![SpotLight::degrees_new(
                Point::origin(),
                Vector::new(1., 0., 0.),
                90.,
                LinearColor::new(1., 0.5, 0.2),
            )],
        );
        let lights: LightAggregate = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(lights, expected)
    }
}
