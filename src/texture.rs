use core::f32;

use glam::{Vec2, Vec3};
use image::io;

use crate::color::Color;
pub trait Sampler2D: Send + Sync {
    fn sample(&self, v: Vec2) -> Color;
}

pub trait Sampler3D: Send + Sync {
    fn sample(&self, v: Vec3) -> Color;
}

#[derive(Default)]
pub struct ColorTexture2D {
    pub color: Color,
}

impl Sampler2D for ColorTexture2D {
    fn sample(&self, _v: Vec2) -> Color {
        return self.color;
    }
}

pub struct ImageTexture2D {
    img: image::RgbImage,
}

impl ImageTexture2D {
    pub fn new(path: String) -> Self {
        let img = io::Reader::open(path)
            .unwrap()
            .decode()
            .unwrap()
            .as_rgb8()
            .unwrap()
            .to_owned();
        Self { img }
    }
}

impl Sampler2D for ImageTexture2D {
    fn sample(&self, v: Vec2) -> Color {
        let rgba = self
            .img
            .get_pixel(
                ((self.img.width() as f32 * v.x) as u32).clamp(0, self.img.width() - 1),
                ((self.img.height() as f32 * v.y) as u32).clamp(0, self.img.height() - 1),
            )
            .0
            .map(|f| f as f32 / 255.0);
        Color::new(rgba[0], rgba[1], rgba[2])
    }
}
