#![no_std]

pub mod aabb;
pub mod camera;
pub mod color;
pub mod color_u8;
pub mod hittable;
pub mod interval;
pub mod materials;
pub mod mesh;
pub mod onb;
pub mod quad;
pub mod rand;
pub mod rand_vec3;
pub mod ray;
pub mod render_parameters;
pub mod sphere;
pub mod texture;
// pub mod triangle;
// pub mod vertex;

fn fast_u32_to_f32(u: u32) -> f32 {
    f32::from_bits((u >> 9) | 0x3F800000) - 1.0
}
