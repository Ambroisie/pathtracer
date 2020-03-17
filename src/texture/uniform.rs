use super::super::core::LinearColor;
use super::super::Point2D;
use super::Texture;

/// A texture with the same color on all points.
#[derive(Debug, PartialEq)]
pub struct UniformTexture {
    color: LinearColor,
}

impl UniformTexture {
    pub fn new(color: LinearColor) -> Self {
        UniformTexture { color }
    }
}

impl Texture for UniformTexture {
    fn texel_color(&self, _: Point2D) -> LinearColor {
        self.color.clone()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new_works() {
        let color = LinearColor::new(0.2, 0.4, 0.6);
        let texture = UniformTexture::new(color.clone());
        assert_eq!(texture, UniformTexture { color })
    }

    fn simple_texture() -> UniformTexture {
        UniformTexture::new(LinearColor::new(0.25, 0.5, 1.))
    }
    #[test]
    fn texel_color_works() {
        let texture = simple_texture();
        assert_eq!(
            texture.texel_color(Point2D::origin()),
            LinearColor::new(0.25, 0.5, 1.)
        )
    }
}
