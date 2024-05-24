use crate::interval::Interval;
use glam::Vec3;

#[derive(Default, Clone, Copy, Debug)]
pub struct AABB {
    pub x: Interval,
    pub y: Interval,
    pub z: Interval,
}

impl AABB {
    pub fn new(x: Interval, y: Interval, z: Interval) -> Self {
        Self { x, y, z }
    }

    pub fn from_extrema(a: Vec3, b: Vec3) -> Self {
        Self {
            x: if a.x <= b.x {
                Interval::new(a.x, b.x)
            } else {
                Interval::new(b.x, a.x)
            },
            y: if a.y <= b.y {
                Interval::new(a.y, b.y)
            } else {
                Interval::new(b.y, a.y)
            },
            z: if a.z <= b.z {
                Interval::new(a.z, b.z)
            } else {
                Interval::new(b.z, a.z)
            },
        }
    }

    pub fn hit(&self, r: &crate::ray::Ray, ray_t: Interval) -> bool {
        let mut ray_t = ray_t;
        for i in 0..3 {
            let ax = match i {
                0 => self.x,
                1 => self.y,
                2 => self.z,
                _ => unreachable!(),
            };
            let adinv = 1.0 / r.direction[i];

            let t0 = (ax.min - r.origin[i]) * adinv;
            let t1 = (ax.max - r.origin[i]) * adinv;

            if t0 < t1 {
                if t0 > ray_t.min {
                    ray_t.min = t0;
                }
                if t1 < ray_t.max {
                    ray_t.max = t1;
                }
            } else {
                if t1 > ray_t.min {
                    ray_t.min = t1;
                }
                if t0 < ray_t.max {
                    ray_t.max = t0;
                }
            }
            if ray_t.max <= ray_t.min {
                return false;
            }
        }
        return true;
    }

    pub fn to_contain(&self, other: &Self) -> Self {
        Self {
            x: self.x.to_contain(&other.x),
            y: self.y.to_contain(&other.y),
            z: self.z.to_contain(&other.z),
        }
    }

    pub fn min(&self) -> Vec3 {
        Vec3 { x: self.x.min, y: self.y.min, z: self.z.min }
    }

    pub fn max(&self) -> Vec3 {
        Vec3 { x: self.x.max, y: self.y.max, z: self.z.max }
    }
}
