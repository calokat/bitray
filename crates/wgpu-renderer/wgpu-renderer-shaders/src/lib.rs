#![no_std]

use bytemuck::{Pod, Zeroable};
use glam::{UVec3, Vec3, Vec3Swizzles, Vec4};
use shared_gpu::{
    camera::Camera,
    color::Color,
    hittable::Hittable,
    interval::Interval,
    materials::{lambert::Lambert, material::Material},
    render_parameters::RenderParameters,
    sphere::Sphere,
    texture::{ColorTexture2D, Sampler2D},
};
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
    let red_texture = Sampler2D::ColorTexture2D(ColorTexture2D {
        color: Color::new(1.0, 0.0, 0.0),
    });
    let material = Material::LambertMaterial(Lambert::new(red_texture.clone()));
    let sphere: Sphere = Sphere::new(Vec3::ZERO, 15.0, material.clone());
    let camera = Camera::new(
        Vec3::NEG_Z * 30.0,
        Vec3::ZERO,
        Vec3::Y,
        RenderParameters {
            image_width: 1024,
            image_height: 512,
            num_samples: 1,
            max_depth: 1,
            background_color: Color::black(),
        },
    );
    let ray = camera.generate_ray(id.truncate().into());
    let color_vec: Vec4 = if sphere.hit(&ray, Interval::universe()) {
        Vec4::new(1.0, 0.0, 0.0, 1.0)
    } else {
        Vec4::ZERO
    };
    unsafe {
        color_out.write(id.xy(), color_vec);
    }
}
