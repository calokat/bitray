use crate::{color::Color, Float};

#[derive(Clone, Copy)]
pub struct RenderParameters {
    pub aspect_ratio: Float,
    pub image_width: i32,
    pub image_height: i32,
    pub num_samples: i32,
    pub max_depth: i32,
    pub background_color: Color,
}
