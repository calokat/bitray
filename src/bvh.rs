use std::{borrow::Borrow, cmp::Ordering, fmt::Debug};

use rand::Rng;

use crate::{aabb::AABB, hittable::Hittable};

enum BVHValue<'a> {
    SubBVH(Box<BVH<'a>>),
    Leaf(&'a dyn Hittable),
}

impl<'a> Debug for BVHValue<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            &Self::SubBVH(bvh) => {
                f.write_fmt(format_args!("{:?}", bvh))?;
            }
            &Self::Leaf(hittable) => {
                f.write_fmt(format_args!("{:?}", hittable))?;
            }
        };
        Ok(())
    }
}

pub struct BVH<'a> {
    left: BVHValue<'a>,
    right: BVHValue<'a>,
    aabb: AABB,
    name: String,
}

impl<'a> BVH<'a> {
    fn box_x_compare(a: &&dyn Hittable, b: &&dyn Hittable) -> Ordering {
        match a.bounding_box().z.min < b.bounding_box().z.min {
            true => Ordering::Less,
            false => Ordering::Greater,
        }
    }

    fn box_y_compare(a: &&dyn Hittable, b: &&dyn Hittable) -> Ordering {
        match a.bounding_box().z.min < b.bounding_box().z.min {
            true => Ordering::Less,
            false => Ordering::Greater,
        }
    }

    fn box_z_compare(a: &&dyn Hittable, b: &&dyn Hittable) -> Ordering {
        match a.bounding_box().z.min < b.bounding_box().z.min {
            true => Ordering::Less,
            false => Ordering::Greater,
        }
    }
    pub fn new(hit_list: Vec<&'a dyn Hittable>) -> Self {
        Self::build(hit_list)
    }
    fn build(hit_list: Vec<&'a dyn Hittable>) -> Self {
        let i: i32 = rand::thread_rng().gen_range(0..3);

        let span = hit_list.len();
        let mut sorted_objects = hit_list;
        sorted_objects.sort_by(|a, b| match i {
            0 => Self::box_x_compare(a, b),
            1 => Self::box_y_compare(a, b),
            2 => Self::box_z_compare(a, b),
            _ => unreachable!(),
        });
        let name: String = "BVH".into();
        if span == 1 {
            let left = sorted_objects[0];
            Self {
                aabb: left.bounding_box(),
                left: BVHValue::Leaf(left),
                right: BVHValue::Leaf(left),
                name,
            }
        } else if span == 2 {
            let left = sorted_objects[0];
            let right = sorted_objects[1];
            Self {
                aabb: AABB::to_contain(&left.bounding_box(), &right.bounding_box()),
                left: BVHValue::Leaf(left),
                right: BVHValue::Leaf(right),
                name,
            }
        } else {
            let mid = span / 2;
            let mut left_list = vec![];
            let mut right_list = vec![];

            for (i, h) in sorted_objects.into_iter().enumerate() {
                if i < mid {
                    left_list.push(h);
                } else {
                    right_list.push(h);
                }
            }

            let left = Self::build(left_list);
            let right = Self::build(right_list);
            let aabb = AABB::to_contain(&left.bounding_box(), &right.bounding_box());

            Self {
                left: BVHValue::SubBVH(Box::new(left)),
                right: BVHValue::SubBVH(Box::new(right)),
                aabb,
                name,
            }
        }
    }
}

impl<'a> Hittable for BVH<'a> {
    fn hit(
        &self,
        r: &crate::ray::Ray,
        ray_t: crate::interval::Interval,
    ) -> Option<crate::hittable::HitRecord> {
        if !self.aabb.hit(r, ray_t) {
            return None;
        }
        let left_hit = match self.left.borrow() {
            BVHValue::Leaf(leaf) => leaf.hit(r, ray_t),
            BVHValue::SubBVH(bvh) => bvh.hit(r, ray_t),
        };

        let right_hit = match self.right.borrow() {
            BVHValue::Leaf(leaf) => leaf.hit(r, ray_t),
            BVHValue::SubBVH(bvh) => bvh.hit(r, ray_t),
        };
        if right_hit.is_some() && left_hit.is_some() {
            if left_hit.as_ref().unwrap().t <= right_hit.as_ref().unwrap().t {
                return left_hit;
            } else {
                return right_hit;
            }
        } else {
            return left_hit.or(right_hit);
        }
    }

    fn bounding_box(&self) -> AABB {
        self.aabb
    }

    fn get_name(&self) -> &String {
        return &self.name;
    }
}

impl<'a> Debug for BVH<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.name)?;
        f.write_str(" Left ")?;
        f.write_fmt(format_args!("{:?}\n", &self.left))?;
        f.write_str(" Right ")?;
        f.write_fmt(format_args!("{:?}\n", &self.right))?;
        Ok(())
    }
}
