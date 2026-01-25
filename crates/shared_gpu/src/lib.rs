#![no_std]
#[cfg(not(feature = "f64"))]
pub type Float = f32;
#[cfg(not(feature = "f64"))]
const PI: f32 = core::f32::consts::PI;
#[cfg(not(feature = "f64"))]
pub type Vec3 = glam::Vec3;
#[cfg(not(feature = "f64"))]
pub type Vec2 = glam::Vec2;
#[cfg(not(feature = "f64"))]
pub type Mat4 = glam::Mat4;
#[cfg(not(feature = "f64"))]
pub type Mat3 = glam::Mat3;

#[cfg(feature = "f64")]
pub type Float = f64;
#[cfg(feature = "f64")]
const PI: f64 = core::f64::consts::PI;
#[cfg(feature = "f64")]
pub type Vec3 = glam::DVec3;
#[cfg(feature = "f64")]
pub type Vec2 = glam::DVec2;
#[cfg(feature = "f64")]
pub type Mat4 = glam::DMat4;
#[cfg(feature = "f64")]
pub type Mat3 = glam::DMat3;

pub mod aabb;
pub mod camera;
pub mod color;
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
