use std::{borrow::Borrow, default, fmt::Debug, process::exit};

use crate::{
    aabb::AABB,
    hittable::{Hittable, HittableList, NeverHittable},
    Float, Vec3,
};

enum BVHValue<'a> {
    SubBVH(Box<BVH<'a>>),
    Leaf(Box<dyn Hittable + 'a>),
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
    pub fn new(hit_list: Vec<Box<dyn Hittable + 'a>>) -> Self {
        Self::build_sah(hit_list)
    }

    fn aabb_from_hittable_slice(slice: &[Box<dyn Hittable + 'a>]) -> AABB {
        let mut bb = AABB::default();
        slice.iter().for_each(|h| {
            bb = bb.to_contain_aabb(&h.bounding_box());
        });
        bb
    }

    fn build_sah(hit_list: Vec<Box<dyn Hittable + 'a>>) -> Self {
        #[repr(usize)]
        #[derive(Clone, Copy)]
        enum Axis {
            X = 0usize,
            Y = 1usize,
            Z = 2usize,
        }

        struct SahSortOutcome {
            cost: Float,
            axis: Axis,
            separating_index: usize,
        }

        #[derive(Clone, Copy)]
        struct SahInput {
            centroid: Vec3,
            aabb: AABB,
        }

        fn aabb_from_sah_inputs(inputs: Vec<&SahInput>) -> AABB {
            let mut bb = AABB::default();
            inputs.iter().for_each(|i| bb = bb.to_contain_aabb(&i.aabb));
            bb
        }

        let list_bb = Self::aabb_from_hittable_slice(&hit_list);
        let area_root = list_bb.surface_area();

        let mut hittable_inputs: Vec<SahInput> = hit_list
            .iter()
            .map(|h| SahInput {
                centroid: h.centroid(),
                aabb: h.bounding_box(),
            })
            .collect();
        let sort_outcome = [Axis::X, Axis::Y, Axis::Z]
            .iter()
            .map(|axis| {
                hittable_inputs.sort_by(|a, b| {
                    a.centroid[*axis as usize].total_cmp(&b.centroid[*axis as usize])
                });
                return (1..hittable_inputs.len())
                    .map(|i| {
                        let (left_objects, right_objects): (Vec<_>, Vec<_>) = hittable_inputs
                            .iter()
                            .enumerate()
                            .partition(|(a, _)| *a < i);
                        let left_bb =
                            aabb_from_sah_inputs(left_objects.iter().map(|i| i.1).collect());
                        let left_sa = left_bb.surface_area();

                        let right_bb =
                            aabb_from_sah_inputs(right_objects.iter().map(|i| i.1).collect());
                        let right_sa = right_bb.surface_area();

                        let sah_cost = 1.0
                            + (left_sa / area_root) * left_objects.len() as Float
                            + (right_sa / area_root) * right_objects.len() as Float;
                        return SahSortOutcome {
                            cost: sah_cost,
                            separating_index: i,
                            axis: *axis,
                        };
                    })
                    .min_by(|a, b| a.cost.total_cmp(&b.cost));
            })
            .filter_map(|o| o)
            .min_by(|a, b| a.cost.total_cmp(&b.cost));
        if let Some(o) = sort_outcome {
            if o.cost < hit_list.len() as f32 {
                let separating_value = hit_list[o.separating_index].centroid()[o.axis as usize];
                let (left_hittables, right_hittables): (Vec<_>, Vec<_>) = hit_list
                    .into_iter()
                    .enumerate()
                    .partition(|(_, h)| h.centroid()[o.axis as usize] < separating_value);
                if left_hittables.is_empty() || right_hittables.is_empty() {
                    let hittable_iter = if left_hittables.is_empty() {
                        right_hittables.into_iter()
                    } else {
                        left_hittables.into_iter()
                    };
                    let hittable_list: Vec<_> = hittable_iter.map(|(_, h)| h).collect();
                    let aabb = Self::aabb_from_hittable_slice(&hittable_list);
                    return Self {
                        left: BVHValue::Leaf(Box::new(HittableList::new(hittable_list))),
                        right: BVHValue::Leaf(Box::new(NeverHittable::default())),
                        aabb,
                        name: String::new(),
                    };
                }
                return Self {
                    left: BVHValue::SubBVH(Box::new(Self::build_sah(
                        left_hittables.into_iter().map(|(_, h)| h).collect(),
                    ))),
                    right: BVHValue::SubBVH(Box::new(Self::build_sah(
                        right_hittables.into_iter().map(|(_, h)| h).collect(),
                    ))),
                    aabb: list_bb,
                    name: String::new(),
                };
            }
        }
        let hittable_list = HittableList::new(hit_list);
        return Self {
            left: BVHValue::Leaf(Box::new(hittable_list)),
            right: BVHValue::Leaf(Box::new(NeverHittable::default())),
            aabb: list_bb,
            name: String::new(),
        };
    }
}

impl<'a> Hittable for BVH<'a> {
    fn hit(
        &self,
        r: &crate::ray::Ray,
        ray_t: crate::interval::Interval,
    ) -> Option<crate::hittable::HitRecord<'_>> {
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

    fn get_name(&self) -> String {
        return self.name.clone();
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
