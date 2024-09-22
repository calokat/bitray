use crate::{Float, PI, Vec2, Vec3};
use std::fmt::{Debug, Write};

use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::materials::material::Material;
use crate::ray::Ray;

pub struct Sphere<'a> {
    center: Vec3,
    radius: Float,
    material: &'a dyn Material,
    name: String,
}

impl<'a> Sphere<'a> {
    pub fn new(c: Vec3, r: Float, material: &'a dyn Material, name: String) -> Self {
        Self {
            center: c,
            radius: r,
            material,
            name,
        }
    }

    fn get_uv(p: Vec3) -> Vec2 {
        let theta = Float::acos(-p.y);
        let phi = Float::atan2(-p.z, p.x) + PI;

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

        let sqrtd = Float::sqrt(discriminant);
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

    fn pdf_value(&self, origin: &Vec3, direction: &Vec3) -> Float {
        if let Some(_) = self.hit(&Ray::new(*origin, *direction), Interval::new(0.0001, Float::MAX)) {
            let dist_squared = (self.center - *origin).length_squared();
            let cos_theta_max = Float::sqrt(1.0 - self.radius * self.radius / dist_squared);
            let solid_angle = 2.0 * PI * (1.0 - cos_theta_max);

            return 1.0 / solid_angle;
        }

        0.0
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
