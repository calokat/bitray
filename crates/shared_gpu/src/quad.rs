use crate::{
    aabb::AABB,
    hittable::{HitRecord, Hittable},
    interval::Interval,
    materials::material::{Material, MaterialType},
    rand::Rng,
    ray::Ray,
};
use core::fmt::{Debug, Write};
use spirv_std::glam::{Vec2, Vec3};

pub struct Quad {
    q: Vec3,
    u: Vec3,
    v: Vec3,
    normal: Vec3,
    w: Vec3,
    aabb: AABB,
    d: f32,
    area: f32,
    material: MaterialType,
}

impl Quad {
    pub fn new(q: Vec3, u: Vec3, v: Vec3, material: MaterialType) -> Self {
        let n = u.cross(v);
        let w = n / n.dot(n);
        let normal = n.normalize();
        Self {
            q,
            u,
            v,
            normal,
            w,
            aabb: Self::build_aabb(q, u, v),
            d: normal.dot(q),
            area: n.length(),
            material,
        }
    }

    fn build_aabb(q: Vec3, u: Vec3, v: Vec3) -> AABB {
        AABB::from_extrema(q, q + u + v).to_contain(&AABB::from_extrema(q + u, q + v))
    }

    fn is_interior(&self, alpha: f32, beta: f32) -> bool {
        let unit_interval = Interval::new(0.0, 1.0);

        if !unit_interval.contains(alpha) || !unit_interval.contains(beta) {
            return false;
        }

        return true;
    }
}

impl Hittable for Quad {
    fn hit(&self, r: &crate::ray::Ray, ray_t: crate::interval::Interval) -> (bool, HitRecord) {
        let denom = self.normal.dot(r.direction);
        if denom.abs() < 1.0e-8 {
            return (false, HitRecord::miss());
        }

        let t = (self.d - self.normal.dot(r.origin)) / denom;

        if !ray_t.contains(t) {
            return (false, HitRecord::miss());
        }

        let intersection = r.at(t);

        let planar_hit = intersection - self.q;

        let alpha = self.w.dot(planar_hit.cross(self.v));
        let beta = self.w.dot(self.u.cross(planar_hit));

        if self.is_interior(alpha, beta) {
            return (
                true,
                HitRecord::new(intersection, t, self.normal, r, self.material, Vec2::ZERO),
            );
        }
        (false, HitRecord::miss())
    }

    fn bounding_box(&self) -> AABB {
        self.aabb
    }
}

impl Debug for Quad {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(" Quad ")?;
        f.write_char('\n')?;
        Ok(())
    }
}
