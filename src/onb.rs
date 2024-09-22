use crate::{Mat3, Vec3};

pub struct ONB(Mat3);

impl ONB {
    pub fn new(direction: &Vec3) -> Self {
        let w = direction.normalize();
        let a = if w.x.abs() > 0.9 {
            Vec3::Y
        } else {
            Vec3::X
        };
        Self(Mat3::from_cols(a, a.cross(w), w))
    }

    pub fn transform(&self, v: &Vec3) -> Vec3 {
        self.0 * *v
    }

    pub fn w(&self) -> Vec3 {
        self.0.z_axis
    }
}