use spirv_std::glam::{Vec3, Vec4};

use core::ops::{Add, AddAssign, Div, Mul};
#[derive(Clone, Copy, Debug)]
pub struct Color(Vec3);

impl Color {
    pub fn new(r: f32, g: f32, b: f32) -> Color {
        Color(Vec3 { x: r, y: g, z: b })
    }

    pub fn black() -> Self {
        Color(Vec3::ZERO)
    }

    fn to_output_value(f: f32, num_samples: i32) -> u8 {
        let scale = f32::clamp(1.0 / num_samples as f32, 0.0, 0.999);
        let corrected = Self::linear_to_gamma(f * scale);
        return (255.999 * corrected) as u8;
    }

    fn linear_to_gamma(linear: f32) -> f32 {
        libm::sqrtf(linear)
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

    pub fn clamp(&self) -> Self {
        Color(self.0.clamp(Vec3::ZERO, Vec3::ONE))
    }

    pub fn to_vec4(&self) -> Vec4 {
        Vec4::new(self.0.x, self.0.y, self.0.z, 1.0)
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
