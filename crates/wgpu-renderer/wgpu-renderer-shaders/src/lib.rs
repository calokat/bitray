#![no_std]

use bytemuck::{Pod, Zeroable};
use shared_gpu::{
    camera::Camera,
    color::Color,
    hittable::{HitRecord, Hittable},
    interval::Interval,
    materials::{
        lambert::Lambert,
        material::{Material, MaterialHitResult, MaterialType},
    },
    onb::ONB,
    quad::Quad,
    rand_vec3::random_cosine_direction,
    ray::Ray,
    render_parameters::RenderParameters,
    sphere::Sphere,
    texture::{ColorTexture2D, Sampler2D},
};
#[cfg(target_arch = "spirv")]
use spirv_std::num_traits::Float;
use spirv_std::{Image, spirv};
use spirv_std::{
    VectorTruncateInto,
    glam::{UVec3, Vec3, Vec3Swizzles, Vec4},
};

#[derive(Copy, Clone, Pod, Zeroable)]
#[repr(C)]
pub struct ShaderConstants {
    pub width: u32,
    pub height: u32,
    pub time: f32,
}

fn lambert_scatter(_r_in: &Ray, rec: &shared_gpu::hittable::HitRecord) -> Ray {
    return Ray::new(
        rec.p,
        ONB::new(&rec.normal).transform(&random_cosine_direction()),
    );
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
    let sphere: Sphere = Sphere::new(Vec3::ZERO, 15.0, MaterialType::Lambert);
    let light_sphere = Sphere::new(Vec3::new(10.0, 40.0, 0.0), 10.0, MaterialType::Lambert);
    let camera = Camera::new(
        Vec3::NEG_Z * 60.0,
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
    let num_samples = 2;
    let max_depth = 2;

    let mut color_vec: Vec4 = Vec4::ZERO;
    let mut ray: Ray = camera.generate_ray(id.truncate().into());
    for i in 0..num_samples {
        let mut color_sample = Vec4::ONE;
        for j in 0..max_depth {
            let hit_result: (bool, HitRecord) = sphere.hit(&ray, Interval::new(0.0001, f32::MAX));
            if hit_result.0 {
                color_sample *= Vec4::new(0.4, 0.4, 0.0, 1.0);
                ray = lambert_scatter(&ray, &hit_result.1);
            } else {
                let hit_result = light_sphere.hit(&ray, Interval::new(0.0001, f32::MAX));
                if hit_result.0 {
                    color_sample *= Vec4::new(3.0, 2.0, 0.2, 1.0);
                    break;
                } else {
                    color_sample = Vec4::new(0.0, 0.0, 0.0, 1.0);
                    break;
                }
            }
        }
        color_vec += color_sample / num_samples as f32;
    }
    unsafe {
        color_out.write(id.xy(), color_vec);
    }
}
