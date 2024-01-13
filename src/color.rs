use glam::Vec3;
use std::{
    ops::{Add, Mul, AddAssign},
    string::String,
};

pub struct Color(Vec3);

impl Color {
    pub fn new(r: f32, g: f32, b: f32) -> Color {
        Color(Vec3 { x: r, y: g, z: b })
    }
    pub fn to_ppm_str(&self, num_samples: i32) -> String {
        return format!(
            "{} {} {}\n",
            Self::to_ppm_value(self.0.x, num_samples),
            Self::to_ppm_value(self.0.y, num_samples),
            Self::to_ppm_value(self.0.z, num_samples)
        );
    }
    fn to_ppm_value(f: f32, num_samples: i32) -> i32 {
        let scale = f32::clamp(1.0 / num_samples as f32, 0.0, 0.999);
        let corrected = Self::linear_to_gamma(f * scale);
        return (255.999 * corrected as f32) as i32;
    }

    fn linear_to_gamma(linear: f32) -> f32 {
        linear.sqrt()
    }
}

impl Mul<f32> for Color {
    type Output = Color;
    fn mul(self, rhs: f32) -> Self::Output {
        return Color(self.0 * rhs);
    }
}

impl Add for Color {
    type Output = Color;
    fn add(self, rhs: Self) -> Self::Output {
        return Color(self.0 + rhs.0);
    }
}

impl AddAssign for Color {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self(self.0 + rhs.0);
    }
}
