#![no_std]

use bytemuck::{Pod, Zeroable};
use core::f32::consts::PI;
use glam::{UVec3, Vec3, Vec4, vec2, vec3};
#[cfg(target_arch = "spirv")]
use spirv_std::num_traits::Float;
use spirv_std::spirv;

#[derive(Copy, Clone, Pod, Zeroable)]
#[repr(C)]
pub struct ShaderConstants {
    pub width: u32,
    pub height: u32,
    pub time: f32,
}

#[spirv(compute(threads(64)))]
pub fn main_cs(
    #[spirv(global_invocation_id)] id: UVec3,
    #[spirv(storage_texture, descriptor_set = 0, binding = 0)] color_out: &mut [u8],
) {
    let index = id.x as usize;
}
