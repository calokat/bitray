use crate::hittable::{Hittable, HitRecord};
use crate::interval::Interval;
use crate::materials::material::Material;
use glam::Vec3;

pub struct Sphere<'a> {
    center: Vec3,
    radius: f32,
    material: &'a dyn Material,
}

impl<'a> Sphere<'a> {
    pub fn new(c: Vec3, r: f32, material: &'a dyn Material) -> Self {
        Self {
            center: c,
            radius: r,
            material
        }
    }
}

impl<'a> Hittable for Sphere<'a> {
    fn hit(&self, r: &crate::ray::Ray, ray_t: Interval) -> Option<HitRecord> {
        let oc = r.origin - self.center;
        let a = r.direction.length_squared();
        let half_b = oc.dot(r.direction);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = f32::sqrt(discriminant);
        let root = (-half_b - sqrtd) / a;
        if !ray_t.surrounds(root) {
            let root = (-half_b + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return None;
            }
        }
        let p = r.at(root);
        let t = root;
        let outward_normal  = (p - self.center) / self.radius;
        let mut rec: HitRecord = HitRecord::new(p, root, &outward_normal, r, self.material);
        return Some(rec);

    }
}