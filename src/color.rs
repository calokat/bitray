use glam::Vec3;
use std::ops::{Add, AddAssign, Div, Mul};

#[derive(Clone, Copy, Debug, Default)]
pub struct Color(Vec3);

impl Color {
    pub fn new(r: f32, g: f32, b: f32) -> Color {
        Color(Vec3 { x: r, y: g, z: b })
    }

    fn to_output_value(f: f32, num_samples: i32) -> u8 {
        let scale = f32::clamp(1.0 / num_samples as f32, 0.0, 0.999);
        let corrected = Self::linear_to_gamma(f * scale);
        return (255.999 * corrected) as u8;
    }

    fn linear_to_gamma(linear: f32) -> f32 {
        linear.sqrt()
    }

    pub fn to_output_array(&self, num_samples: i32) -> [u8; 3] {
        return [
            Self::to_output_value(self.0.x, num_samples),
            Self::to_output_value(self.0.y, num_samples),
            Self::to_output_value(self.0.z, num_samples),
        ];
    }

    pub fn correct_nans(&mut self) {
        if self.0.x.is_nan() {
            self.0.x = 0.0;
        }
        if self.0.y.is_nan() {
            self.0.y = 0.0;
        }
        if self.0.z.is_nan() {
            self.0.z = 0.0;
        }
    }
}

impl Mul<f32> for Color {
    type Output = Color;
    fn mul(self, rhs: f32) -> Self::Output {
        return Color(self.0 * rhs);
    }
}

impl Mul<Color> for Color {
    type Output = Color;
    fn mul(self, rhs: Color) -> Self::Output {
        return Color(self.0 * rhs.0);
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

impl Div<f32> for Color {
    type Output = Color;
    fn div(self, rhs: f32) -> Self::Output {
        Self(self.0 / rhs)
    }
}
