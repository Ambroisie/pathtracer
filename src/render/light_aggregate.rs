use crate::light::*;
use std::iter::Iterator;

#[derive(Debug, PartialEq)]
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
}
