//! Light property coefficients (diffuse, specular, transparency, reflectivity...)

use super::color::LinearColor;
use serde::Deserialize;

#[derive(Debug, PartialEq, Clone, Deserialize)]
#[serde(untagged)]
/// This enum stores the reflectivity or transparency information.
pub enum ReflTransEnum {
    /// Transparence properties.
    Transparency {
        /// The transparency coefficient.
        #[serde(rename = "transparency")]
        coef: f32,
        /// The diffraction index.
        index: f32,
    },
    /// Reflectivity properties.
    Reflectivity {
        /// The reflectivity coefficient.
        #[serde(rename = "reflectivity")]
        coef: f32,
    },
}

/// A structure holding all the physical proprerties relating to light at a point.
#[derive(Debug, PartialEq, Clone, Deserialize)]
pub struct LightProperties {
    /// The diffuse component.
    pub diffuse: LinearColor,
    /// The specular component.
    pub specular: LinearColor,
    /// The transparency or reflectivity properties.
    #[serde(flatten)]
    pub refl_trans: Option<ReflTransEnum>,
}

impl LightProperties {
    /// Creates a new `LightProperties` struct.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pathtracer::core::light_properties::{LightProperties, ReflTransEnum};
    /// # use pathtracer::core::color::LinearColor;
    /// #
    /// let lp = LightProperties::new(
    ///     LinearColor::new(0.25, 0.5, 1.),
    ///     LinearColor::new(0.75, 0.375, 0.125),
    ///     Some(ReflTransEnum::Reflectivity { coef: 0.5 }),
    /// );
    /// ```
    pub fn new(
        diffuse: LinearColor,
        specular: LinearColor,
        refl_trans: Option<ReflTransEnum>,
    ) -> Self {
        LightProperties {
            diffuse,
            specular,
            refl_trans,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new_works() {
        let diffuse = LinearColor::new(0.25, 0.5, 1.);
        let specular = LinearColor::new(0.75, 0.375, 0.125);
        let refl_trans = Some(ReflTransEnum::Reflectivity { coef: 0.5 });
        let properties =
            LightProperties::new(diffuse.clone(), specular.clone(), refl_trans.clone());
        assert_eq!(
            properties,
            LightProperties {
                diffuse,
                specular,
                refl_trans,
            }
        )
    }

    #[test]
    fn deserialization_without_refl_trans_works() {
        let yaml = r#"
            diffuse: {r: 1.0, g: 0.5, b: 0.25}
            specular: {r: 0.25, g: 0.125, b: 0.75}
        "#;
        let properties: LightProperties = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(
            properties,
            LightProperties::new(
                LinearColor::new(1., 0.5, 0.25),
                LinearColor::new(0.25, 0.125, 0.75),
                None
            )
        )
    }

    #[test]
    fn deserialization_with_reflection_works() {
        let yaml = r#"
            diffuse: {r: 1.0, g: 0.5, b: 0.25}
            specular: {r: 0.25, g: 0.125, b: 0.75}
            transparency: 0.5
            index: 1.5
        "#;
        let properties: LightProperties = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(
            properties,
            LightProperties::new(
                LinearColor::new(1., 0.5, 0.25),
                LinearColor::new(0.25, 0.125, 0.75),
                Some(ReflTransEnum::Transparency {
                    coef: 0.5,
                    index: 1.5
                })
            )
        )
    }

    #[test]
    fn deserialization_with_transparency_works() {
        let yaml = r#"
            diffuse: {r: 1.0, g: 0.5, b: 0.25}
            specular: {r: 0.25, g: 0.125, b: 0.75}
            reflectivity: 0.25
        "#;
        let properties: LightProperties = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(
            properties,
            LightProperties::new(
                LinearColor::new(1., 0.5, 0.25),
                LinearColor::new(0.25, 0.125, 0.75),
                Some(ReflTransEnum::Reflectivity { coef: 0.25 })
            )
        )
    }
}
