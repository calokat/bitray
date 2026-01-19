use image::{ImageBuffer, ImageResult};

use crate::color::Color;

pub fn write_image(
    image_memory: &Vec<Vec<Color>>,
    dimensions: (i32, i32),
    num_samples: i32,
) -> ImageResult<()> {
    let img = ImageBuffer::from_fn(dimensions.0 as u32, dimensions.1 as u32, |x, y| {
        let c = image_memory[y as usize][x as usize];
        image::Rgb(c.to_output_array(num_samples))
    });
    img.save("image.png")
}
