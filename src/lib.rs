pub mod aabb;
pub mod bvh;
pub mod camera;
pub mod color;
pub mod hittable;
pub mod interval;
pub mod materials;
pub mod mesh;
pub mod onb;
pub mod pdf;
pub mod quad;
pub mod rand_vec3;
pub mod ray;
pub mod sphere;
pub mod texture;
pub mod triangle;
pub mod vertex;

#[cfg(not(feature = "f64"))]
pub type Float = f32;
#[cfg(not(feature = "f64"))]
const PI: f32 = std::f32::consts::PI;
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
const PI: f64 = std::f64::consts::PI;
#[cfg(feature = "f64")]
pub type Vec3 = glam::DVec3;
#[cfg(feature = "f64")]
pub type Vec2 = glam::DVec2;
#[cfg(feature = "f64")]
pub type Mat4 = glam::DMat4;
#[cfg(feature = "f64")]
pub type Mat3 = glam::DMat3;
