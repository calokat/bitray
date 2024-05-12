use std::{cmp::Ordering, fmt::Debug};

use rand::Rng;

use crate::{
    aabb::AABB,
    hittable::{Hittable, HittableList},
};

pub struct BVH {
    left: Box<dyn Hittable>,
    right: Box<dyn Hittable>,
    aabb: AABB,
    name: String,
}

impl BVH {
    fn box_x_compare(a: &Box<dyn Hittable>, b: &Box<dyn Hittable>) -> Ordering {
        match a.bounding_box().z.min < b.bounding_box().z.min {
            true => Ordering::Less,
            false => Ordering::Greater,
        }
    }

    fn box_y_compare(a: &Box<dyn Hittable>, b: &Box<dyn Hittable>) -> Ordering {
        match a.bounding_box().z.min < b.bounding_box().z.min {
            true => Ordering::Less,
            false => Ordering::Greater,
        }
    }

    fn box_z_compare(a: &Box<dyn Hittable>, b: &Box<dyn Hittable>) -> Ordering {
        match a.bounding_box().z.min < b.bounding_box().z.min {
            true => Ordering::Less,
            false => Ordering::Greater,
        }
    }
    pub fn new(hit_list: HittableList) -> Self {
        Self::build(hit_list)
    }
    fn build(hit_list: HittableList) -> Self {
        let i: i32 = rand::thread_rng().gen_range(0..3);

        let span = hit_list.objects.len();
        let mut hit_objects = hit_list.objects;
        hit_objects.sort_by(|a, b| match i {
            0 => Self::box_x_compare(a, b),
            1 => Self::box_y_compare(a, b),
            2 => Self::box_z_compare(a, b),
            _ => unreachable!(),
        });
        let name: String = "BVH".into();
        if span == 1 {
            let left = hit_objects.into_iter().next().unwrap();
            let right = Box::new(HittableList::new(vec![]));
            Self {
                aabb: left.bounding_box(),
                left,
                right,
                name,
            }
        } else if span == 2 {
            let mut it = hit_objects.into_iter();
            let left = it.next().unwrap();
            let right = it.next().unwrap();
            Self {
                aabb: AABB::to_contain(&left.bounding_box(), &right.bounding_box()),
                left,
                right,
                name,
            }
        } else {
            let mid = span / 2;
            // let mut left_list = hit_list.objects.into_boxed_slice()[..mid];
            // left_list.sort_by(|a, b| match i {
            //     0 => Self::box_x_compare(*a, *b),
            //     1 => Self::box_y_compare(*a, *b),
            //     2 => Self::box_z_compare(*a, *b),
            //     _ => unreachable!(),
            // });
            // let left_list = HittableList::new(left_list);
            //
            // let mut right_list = hit_list.objects.clone()[mid..].to_vec();
            // right_list.sort_by(|a, b| match i {
            //     0 => Self::box_x_compare(*a, *b),
            //     1 => Self::box_y_compare(*a, *b),
            //     2 => Self::box_z_compare(*a, *b),
            //     _ => unreachable!(),
            // });
            // let right_list = HittableList::new(right_list);
            let mut left_list: Vec<Box<dyn Hittable>> = vec![];
            let mut right_list: Vec<Box<dyn Hittable>> = vec![];

            for (i, h) in hit_objects.into_iter().enumerate() {
                if i < mid {
                    left_list.push(h);
                } else {
                    right_list.push(h);
                }
            }
            left_list.sort_by(|a, b| match i {
                0 => Self::box_x_compare(a, b),
                1 => Self::box_y_compare(a, b),
                2 => Self::box_z_compare(a, b),
                _ => unreachable!(),
            });
            right_list.sort_by(|a, b| match i {
                0 => Self::box_x_compare(a, b),
                1 => Self::box_y_compare(a, b),
                2 => Self::box_z_compare(a, b),
                _ => unreachable!(),
            });

            let left = Self::build(HittableList::new(left_list));
            let right = Self::build(HittableList::new(right_list));
            let aabb = AABB::to_contain(&left.bounding_box(), &right.bounding_box());

            Self {
                left: Box::new(left),
                right: Box::new(right),
                aabb,
                name,
            }
        }
    }
}

impl Hittable for BVH {
    fn hit(
        &self,
        r: &crate::ray::Ray,
        ray_t: crate::interval::Interval,
    ) -> Option<crate::hittable::HitRecord> {
        if !self.aabb.hit(r, ray_t) {
            return None;
        }

        // if let Some(hr) = self.left.hit(r, ray_t) {
        //     return Some(hr);
        // } else if let Some(hr) = self.right.hit(r, ray_t) {
        //     return Some(hr);
        // } else {
        //     return None;
        // }
        let left_hit = self.left.hit(r, ray_t);

        let right_hit = self.right.hit(r, ray_t);
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

impl Debug for BVH {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.name)?;
        f.write_str(" Left ")?;
        f.write_fmt(format_args!("{:?}\n", &self.left))?;
        f.write_str(" Right ")?;
        f.write_fmt(format_args!("{:?}\n", &self.right))?;
        Ok(())
    }
}
