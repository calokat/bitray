use spirv_std::glam::Vec2;

use crate::color::Color;
#[derive(Clone, Copy)]
pub struct ColorTexture2D {
    pub color: Color,
}

#[derive(Clone, Copy)]
pub enum Sampler2D {
    ColorTexture2D(ColorTexture2D),
}

impl Sampler2D {
    pub fn sample(&self, _v: Vec2) -> Color {
        match &self {
            Sampler2D::ColorTexture2D(color) => color.color.clone(),
        }
    }
}
