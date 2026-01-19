#![no_std]

use bytemuck::{Pod, Zeroable};
use core::f32::consts::PI;
use glam::{UVec3, Vec3, Vec3Swizzles, Vec4, vec2, vec3};
#[cfg(target_arch = "spirv")]
use spirv_std::num_traits::Float;
use spirv_std::{Image, spirv};

#[derive(Copy, Clone, Pod, Zeroable)]
#[repr(C)]
pub struct ShaderConstants {
    pub width: u32,
    pub height: u32,
    pub time: f32,
}

#[spirv(compute(threads(64)))]
pub fn compute_main(
    #[spirv(global_invocation_id)] id: UVec3,
    #[spirv(descriptor_set = 0, binding = 0)] color_out: &Image!(
        2D,
        format = rgba8_snorm,
        sampled = false
    ),
) {
    unsafe {
        color_out.write(id.xy(), Vec4::new(1.0, 1.0, 1.0, 1.0));
    }
}
