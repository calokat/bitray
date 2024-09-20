use glam::{Vec2, Vec3};
use std::fmt::{Debug, Write};
use crate::{aabb::AABB, hittable::{HitRecord, Hittable}, interval::Interval, materials::material::Material};

pub struct Quad<'a> {
    q: Vec3,
    u: Vec3,
    v: Vec3,
    normal: Vec3,
    w: Vec3,
    aabb: AABB,
    d: f32,
    area: f32,
    material: &'a dyn Material,
    name: String,
}

impl<'a> Quad<'a> {
    pub fn new(q: Vec3, u: Vec3, v: Vec3, material: &'a dyn Material) -> Self {
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
            name: "Quad".into()
        }
    }

    fn build_aabb(q: Vec3, u: Vec3, v: Vec3) -> AABB {
        AABB::from_extrema(q, q + u + v).to_contain(&AABB::from_extrema(q + u, q + v))
    }

    fn is_interior(&self, alpha: f32, beta: f32) -> Option<Vec2> {
        let unit_interval = Interval::new(0.0, 1.0);

        if !unit_interval.contains(alpha) || !unit_interval.contains(beta) {
            return None;
        }

        return Some(Vec2::new(alpha, beta));
    }
}

impl<'a> Hittable for Quad<'a> {
    fn hit(&self, r: &crate::ray::Ray, ray_t: crate::interval::Interval) -> Option<crate::hittable::HitRecord> {
        let denom = self.normal.dot(r.direction);
        if denom < 1.0e-8 {
            return None;
        }

        let t = (self.d - self.normal.dot(r.origin)) / denom;

        if !ray_t.contains(t) {
            return None;
        }

        let intersection = r.at(t);

        let planar_hit = intersection - self.q;

        let alpha = self.w.dot(planar_hit.cross(self.v));
        let beta = self.w.dot(self.u.cross(planar_hit));

        if let Some(uv) = self.is_interior(alpha, beta) {
            return Some(HitRecord::new(intersection, t, self.normal, r, self.material, uv));
        }
        None
    }

    fn bounding_box(&self) -> AABB {
        self.aabb
    }

    fn get_name(&self) -> &String {
        &self.name
    }
}

impl<'a> Debug for Quad<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(" Quad ")?;
        f.write_char('\n')?;
        Ok(())
    }
}
