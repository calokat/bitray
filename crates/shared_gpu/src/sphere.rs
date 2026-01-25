use crate::onb::ONB;
use crate::rand::Rng;
use crate::{Float, PI, Vec2, Vec3};
use core::fmt::{Debug, Write};

use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::materials::material::Material;
use crate::ray::Ray;

pub struct Sphere {
    center: Vec3,
    radius: Float,
    material: Material,
}

impl Sphere {
    pub fn new(c: Vec3, r: Float, material: Material) -> Self {
        Self {
            center: c,
            radius: r,
            material,
        }
    }

    fn get_uv(p: Vec3) -> Vec2 {
        let theta = libm::acosf(-p.y);
        let phi = libm::atan2f(-p.z, p.x) + PI;

        let u = phi / (2.0 * PI);
        let v = theta / PI;
        Vec2::new(u, v)
    }

    fn random_to_sphere(radius: Float, distance_squared: Float) -> Vec3 {
        let mut rng = Rng::new();
        let r1 = rng.f32();
        let r2 = rng.f32();
        let z = 1.0 + r2 * (libm::sqrtf(1.0 - radius * radius / distance_squared) - 1.0);

        let phi = 2.0 * crate::PI * r1;
        let x = libm::cosf(phi) * libm::sqrtf(1.0 - z * z);
        let y = libm::sinf(phi) * libm::sqrtf(1.0 - z * z);

        return Vec3::new(x, y, z);
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &crate::ray::Ray, ray_t: Interval) -> bool {
        let oc = r.origin - self.center;
        let a = r.direction.length_squared();
        let half_b = oc.dot(r.direction);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return false;
        }

        let sqrtd = libm::sqrtf(discriminant);
        let root = (-half_b - sqrtd) / a;
        if !ray_t.surrounds(root) {
            let root = (-half_b + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return false;
            }
        }
        let p = r.at(root);
        let outward_normal = (p - self.center) / self.radius;
        let uv = Self::get_uv((p - self.center) / self.radius);
        let rec: HitRecord = HitRecord::new(p, root, outward_normal, r, self.material, uv);
        return true;
    }

    fn bounding_box(&self) -> crate::aabb::AABB {
        crate::aabb::AABB::from_extrema(
            self.center + Vec3::splat(self.radius),
            self.center - Vec3::splat(self.radius),
        )
    }
}

impl Debug for Sphere {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("Sphere")?;
        f.write_char('\n')?;
        Ok(())
    }
}
