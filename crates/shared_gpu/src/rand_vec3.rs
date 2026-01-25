use core::f32;

use crate::rand::Rng;
use spirv_std::glam::Vec3;

pub fn random_vec() -> Vec3 {
    let mut rng = Rng::new();

    Vec3 {
        x: rng.f32(),
        y: rng.f32(),
        z: rng.f32(),
    }
}

pub fn random_vec_range(min: f32, max: f32) -> Vec3 {
    let mut rng = Rng::new();

    Vec3 {
        x: min + (max - min) * rng.f32(),
        y: min + (max - min) * rng.f32(),
        z: min + (max - min) * rng.f32(),
    }
}

pub fn random_vec_in_unit_sphere() -> Vec3 {
    loop {
        let p = random_vec_range(-1.0, 1.0);
        if p.length_squared() < 1.0 {
            return p;
        }
    }
}

pub fn random_unit_vector() -> Vec3 {
    random_vec_in_unit_sphere().normalize()
}

pub fn random_vec_on_hemisphere(normal: &Vec3) -> Vec3 {
    let on_unit_sphere = random_unit_vector();
    return if on_unit_sphere.dot(*normal) > 0.0 {
        on_unit_sphere
    } else {
        -on_unit_sphere
    };
}

pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    return *v - 2.0 * v.dot(*n) * *n;
}

pub fn refract(uv: &Vec3, n: &Vec3, etai_over_etat: f32) -> Vec3 {
    let cos_theta = libm::fminf(Vec3::dot(-*uv, *n), 1.0);
    let r_out_perp = etai_over_etat * (*uv + cos_theta * *n);
    let r_out_parallel = -libm::sqrtf(libm::fabsf(1.0 - r_out_perp.length_squared())) * *n;
    return r_out_perp + r_out_parallel;
}

pub fn random_vec_unit_disk() -> Vec3 {
    let mut rng = Rng::new();

    Vec3 {
        x: rng.f32(),
        y: rng.f32(),
        z: 0.0,
    }
}

pub fn random_cosine_direction() -> Vec3 {
    let mut rng = Rng::new();

    let r1 = rng.f32();
    let r2 = rng.f32();

    let phi = 2.0 * f32::consts::PI * r1;
    let x = libm::cosf(phi) * libm::sqrtf(r2);
    let y = libm::sinf(phi) * libm::sqrtf(r2);
    let z = libm::sqrtf(1.0 - r2);
    Vec3::new(x, y, z)
}
