use crate::Vec3;
use rand::prelude::*;
use crate::{Float, PI};
pub fn random_vec() -> Vec3 {
    Vec3 {
        x: random::<Float>(),
        y: random::<Float>(),
        z: random::<Float>(),
    }
}

pub fn random_vec_range(min: Float, max: Float) -> Vec3 {
    Vec3 {
        x: min + (max - min) * random::<Float>(),
        y: min + (max - min) * random::<Float>(),
        z: min + (max - min) * random::<Float>(),
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

pub fn refract(uv: &Vec3, n: &Vec3, etai_over_etat: Float) -> Vec3 {
    let cos_theta = Float::min(Vec3::dot(-*uv, *n), 1.0);
    let r_out_perp = etai_over_etat * (*uv + cos_theta * *n);
    let r_out_parallel = -Float::sqrt(Float::abs(1.0 - r_out_perp.length_squared())) * *n;
    return r_out_perp + r_out_parallel;
}

pub fn random_vec_unit_disk() -> Vec3 {
    Vec3 {
        x: random(),
        y: random(),
        z: 0.0,
    }
}

pub fn random_cosine_direction() -> Vec3 {
    let r1 = random::<Float>();
    let r2 = random::<Float>();

    let phi = 2.0 * PI * r1;
    let x = Float::cos(phi) * Float::sqrt(r2);
    let y = Float::sin(phi) * Float::sqrt(r2);
    let z = Float::sqrt(1.0 - r2);
    Vec3::new(x, y, z)
}
