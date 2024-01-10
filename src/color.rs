use glam::Vec3;
use std::{
    ops::{Add, Mul},
    string::String,
};

pub struct Color(Vec3);

fn to_ppm_value(f: f32) -> i32 {
    return (255.999 * f) as i32;
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32) -> Color {
        Color(Vec3 { x: r, y: g, z: b })
    }
    pub fn to_ppm_str(&self) -> String {
        return format!(
            "{} {} {}\n",
            to_ppm_value(self.0.x),
            to_ppm_value(self.0.y),
            to_ppm_value(self.0.z)
        );
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
