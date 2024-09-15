use std::f32::consts::PI;
use std::fmt::{Debug, Write};

use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::materials::material::Material;
use glam::{Vec2, Vec3};

pub struct Sphere<'a> {
    center: Vec3,
    radius: f32,
    material: &'a dyn Material,
    name: String,
}

impl<'a> Sphere<'a> {
    pub fn new(c: Vec3, r: f32, material: &'a dyn Material, name: String) -> Self {
        Self {
            center: c,
            radius: r,
            material,
            name,
        }
    }

    fn get_uv(p: Vec3) -> Vec2 {
        let theta = f32::acos(-p.y);
        let phi = f32::atan2(-p.z, p.x) + PI;

        let u = phi / (2.0 * PI);
        let v = theta / PI;
        Vec2::new(u, v)
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
        let mut root = (-half_b - sqrtd) / a;
        if !ray_t.surrounds(root) {
            root = (-half_b + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return None;
            }
        }
        let p = r.at(root);
        let outward_normal = (p - self.center) / self.radius;
        let uv = Self::get_uv(p);
        let rec: HitRecord = HitRecord::new(p, root, outward_normal, r, self.material, uv);
        return Some(rec);
    }

    fn bounding_box(&self) -> crate::aabb::AABB {
        crate::aabb::AABB::from_extrema(
            self.center + Vec3::splat(self.radius),
            self.center - Vec3::splat(self.radius),
        )
    }

    fn get_name(&self) -> &String {
        &self.name
    }
}

impl<'a> Debug for Sphere<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Sphere ")?;
        f.write_str(&self.get_name())?;
        f.write_char('\n')?;
        Ok(())
    }
}
