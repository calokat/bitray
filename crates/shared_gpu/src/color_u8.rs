use spirv_std::glam::{U8Vec3, U8Vec4, Vec3, Vec4};

use core::ops::{Add, AddAssign, Div, Mul};
#[derive(Clone, Copy, Debug)]
pub struct ColorU8(pub U8Vec3);

impl ColorU8 {
    pub fn new(r: u8, g: u8, b: u8) -> ColorU8 {
        ColorU8(U8Vec3 { x: r, y: g, z: b })
    }

    pub fn black() -> Self {
        ColorU8(U8Vec3::ZERO)
    }

    pub fn clamp(&self) -> Self {
        ColorU8(self.0.clamp(U8Vec3::ZERO, U8Vec3::ONE))
    }

    pub fn to_vec4(&self) -> U8Vec4 {
        U8Vec4::new(self.0.x, self.0.y, self.0.z, 255)
    }
}

impl Mul<u8> for ColorU8 {
    type Output = ColorU8;
    fn mul(self, rhs: u8) -> Self::Output {
        return ColorU8(self.0 * rhs);
    }
}

impl Mul<ColorU8> for ColorU8 {
    type Output = ColorU8;
    fn mul(self, rhs: ColorU8) -> Self::Output {
        return ColorU8(self.0 * rhs.0);
    }
}

impl Add for ColorU8 {
    type Output = ColorU8;
    fn add(self, rhs: Self) -> Self::Output {
        return ColorU8(self.0 + rhs.0);
    }
}

impl AddAssign for ColorU8 {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self(self.0 + rhs.0);
    }
}

impl Div<u8> for ColorU8 {
    type Output = ColorU8;
    fn div(self, rhs: u8) -> Self::Output {
        Self(self.0 / rhs)
    }
}
