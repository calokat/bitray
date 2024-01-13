use glam::Vec3;
use rand::prelude::*;

pub fn random_vec() -> Vec3 {
    Vec3 { x: random::<f32>(), y: random::<f32>(), z: random::<f32>() }
}

pub fn random_vec_range(min: f32, max: f32) -> Vec3 {
    Vec3 { 
        x: min + (max - min) * random::<f32>(),
        y: min + (max - min) * random::<f32>(),
        z: min + (max - min) * random::<f32>()
    }
}

pub fn random_vec_in_unit_sphere() -> Vec3 {
    loop {
        let p = random_vec_range(-1.0, 1.0);
        if p.length_squared() < 1.0 {
            return p;
        }
    }
    unreachable!()
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
    }
}