use crate::{
    Float,
    aabb::AABB,
    hittable::{HitRecord, Hittable},
    interval::Interval,
    materials::material::Material,
    rand::Rng,
    ray::Ray,
};
use crate::{Vec2, Vec3};
use core::fmt::{Debug, Write};

pub struct Quad {
    q: Vec3,
    u: Vec3,
    v: Vec3,
    normal: Vec3,
    w: Vec3,
    aabb: AABB,
    d: Float,
    area: Float,
    material: Material,
}

impl Quad {
    pub fn new(q: Vec3, u: Vec3, v: Vec3, material: Material) -> Self {
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

    fn is_interior(&self, alpha: Float, beta: Float) -> Option<Vec2> {
        let unit_interval = Interval::new(0.0, 1.0);

        if !unit_interval.contains(alpha) || !unit_interval.contains(beta) {
            return None;
        }

        return Some(Vec2::new(alpha, beta));
    }
}

impl Hittable for Quad {
    fn hit(&self, r: &crate::ray::Ray, ray_t: crate::interval::Interval) -> bool {
        let denom = self.normal.dot(r.direction);
        if denom.abs() < 1.0e-8 {
            return false;
        }

        let t = (self.d - self.normal.dot(r.origin)) / denom;

        if !ray_t.contains(t) {
            return false;
        }

        let intersection = r.at(t);

        let planar_hit = intersection - self.q;

        let alpha = self.w.dot(planar_hit.cross(self.v));
        let beta = self.w.dot(self.u.cross(planar_hit));

        if let Some(uv) = self.is_interior(alpha, beta) {
            return true;
        }
        false
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
