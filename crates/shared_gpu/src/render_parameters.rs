use crate::color::Color;

#[derive(Clone, Copy)]
pub struct RenderParameters {
    pub image_width: i32,
    pub image_height: i32,
    pub num_samples: i32,
    pub max_depth: i32,
    pub background_color: Color,
}
