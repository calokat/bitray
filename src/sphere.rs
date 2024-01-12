use crate::hittable::{Hittable, HitRecord};
use crate::interval::Interval;
use glam::Vec3;

#[derive(Default)]
pub struct Sphere {
    center: Vec3,
    radius: f32,
}

impl Sphere {
    pub fn new(c: Vec3, r: f32) -> Self {
        Self {
            center: c,
            radius: r,
        }
    }
}

impl Hittable for Sphere {
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
        let mut rec: HitRecord = Default::default();
        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal  = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, &outward_normal);
        return Some(rec);

    }
}