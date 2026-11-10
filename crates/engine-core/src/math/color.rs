use super::math_utils::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Color {
    pub packed_value: u32,
}

impl Color {
    pub const TRANSPARENT: Color = Color { packed_value: 0 };
    pub const BLACK: Color = Color { packed_value: 0xFF000000 };
    pub const DARK_GRAY: Color = Color { packed_value: 0xFF404040 };
    pub const GRAY: Color = Color { packed_value: 0xFF808080 };
    pub const LIGHT_GRAY: Color = Color { packed_value: 0xFFC0C0C0 };
    pub const WHITE: Color = Color { packed_value: 0xFFFFFFFF };
    pub const RED: Color = Color { packed_value: 0xFF0000FF };
    pub const GREEN: Color = Color { packed_value: 0xFF00FF00 };
    pub const YELLOW: Color = Color { packed_value: 0xFF00FFFF };
    pub const BLUE: Color = Color { packed_value: 0xFFFF0000 };
    pub const MAGENTA: Color = Color { packed_value: 0xFFFF00FF };
    pub const CYAN: Color = Color { packed_value: 0xFFFFFF00 };
    pub const DARK_RED: Color = Color { packed_value: 0xFF000080 };
    pub const DARK_GREEN: Color = Color { packed_value: 0xFF008000 };
    pub const DARK_BLUE: Color = Color { packed_value: 0xFF800000 };
    pub const ORANGE: Color = Color { packed_value: 0xFF0080FF };
    pub const PINK: Color = Color { packed_value: 0xFF8000FF };

    #[inline]
    pub fn new(packed_value: u32) -> Self {
        Color { packed_value }
    }

    #[inline]
    pub fn from_rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Color { packed_value: ((a as u32) << 24) | ((b as u32) << 16) | ((g as u32) << 8) | (r as u32) }
    }

    #[inline]
    pub fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        Color::from_rgba(r, g, b, 255)
    }

    #[inline]
    pub fn from_rgba_i(r: i32, g: i32, b: i32, a: i32) -> Self {
        Color::from_rgba(
            clamp_i32(r, 0, 255) as u8,
            clamp_i32(g, 0, 255) as u8,
            clamp_i32(b, 0, 255) as u8,
            clamp_i32(a, 0, 255) as u8,
        )
    }

    #[inline]
    pub fn from_rgb_f(r: f32, g: f32, b: f32) -> Self {
        Color::from_rgba(
            (saturate(r) * 255.0) as u8,
            (saturate(g) * 255.0) as u8,
            (saturate(b) * 255.0) as u8,
            255,
        )
    }

    #[inline]
    pub fn from_rgba_f(r: f32, g: f32, b: f32, a: f32) -> Self {
        Color::from_rgba(
            (saturate(r) * 255.0) as u8,
            (saturate(g) * 255.0) as u8,
            (saturate(b) * 255.0) as u8,
            (saturate(a) * 255.0) as u8,
        )
    }

    #[inline]
    pub fn r(&self) -> u8 {
        self.packed_value as u8
    }

    #[inline]
    pub fn g(&self) -> u8 {
        (self.packed_value >> 8) as u8
    }

    #[inline]
    pub fn b(&self) -> u8 {
        (self.packed_value >> 16) as u8
    }

    #[inline]
    pub fn a(&self) -> u8 {
        (self.packed_value >> 24) as u8
    }

    #[inline]
    pub fn set_r(&mut self, v: u8) {
        self.packed_value = (self.packed_value & 0xFFFFFF00) | (v as u32);
    }

    #[inline]
    pub fn set_g(&mut self, v: u8) {
        self.packed_value = (self.packed_value & 0xFFFF00FF) | ((v as u32) << 8);
    }

    #[inline]
    pub fn set_b(&mut self, v: u8) {
        self.packed_value = (self.packed_value & 0xFF00FFFF) | ((v as u32) << 16);
    }

    #[inline]
    pub fn set_a(&mut self, v: u8) {
        self.packed_value = (self.packed_value & 0x00FFFFFF) | ((v as u32) << 24);
    }

    #[inline]
    pub fn rgb(&self) -> Color {
        Color { packed_value: self.packed_value | 0xFF000000 }
    }

    #[inline]
    pub fn with_alpha(rgb: Color, a: u8) -> Color {
        Color { packed_value: (rgb.packed_value & 0x00FFFFFF) | ((a as u32) << 24) }
    }

    pub fn lerp(c1: Color, c2: Color, f: f32) -> Color {
        Color::from_rgba_i(
            lerp(c1.r() as f32, c2.r() as f32, f) as i32,
            lerp(c1.g() as f32, c2.g() as f32, f) as i32,
            lerp(c1.b() as f32, c2.b() as f32, f) as i32,
            lerp(c1.a() as f32, c2.a() as f32, f) as i32,
        )
    }

    pub fn premultiply_alpha(c: Color) -> Color {
        let a = c.a() as f32;
        Color::from_rgba(
            (c.r() as f32 * a / 255.0) as u8,
            (c.g() as f32 * a / 255.0) as u8,
            (c.b() as f32 * a / 255.0) as u8,
            c.a(),
        )
    }

    pub fn rgb_to_hsv(rgb: super::Vector3) -> super::Vector3 {
        let min_v = min_f32_3(rgb.x, rgb.y, rgb.z);
        let max_v = max_f32_3(rgb.x, rgb.y, rgb.z);
        let z = max_v;
        let num3 = max_v - min_v;
        let (y, num4) = if max_v != 0.0 {
            let y = num3 / max_v;
            let h = if num3 == 0.0 {
                0.0
            } else if rgb.x == max_v {
                (rgb.y - rgb.z) / num3
            } else if rgb.y != max_v {
                4.0 + (rgb.x - rgb.y) / num3
            } else {
                2.0 + (rgb.z - rgb.x) / num3
            };
            (y, h * 60.0)
        } else {
            (0.0, -1.0)
        };
        let num4 = if num4 < 0.0 { num4 + 360.0 } else { num4 };
        super::Vector3::new(num4, y, z)
    }

    pub fn hsv_to_rgb(hsv: super::Vector3) -> super::Vector3 {
        if hsv.y == 0.0 {
            return super::Vector3::splat(hsv.z);
        }
        let h = hsv.x / 60.0;
        let num = h.floor() as i32;
        let num2 = h - num as f32;
        let num3 = hsv.z * (1.0 - hsv.y);
        let num4 = hsv.z * (1.0 - hsv.y * num2);
        let num5 = hsv.z * (1.0 - hsv.y * (1.0 - num2));
        let (x, y, z) = match num {
            0 => (hsv.z, num5, num3),
            1 => (num4, hsv.z, num3),
            2 => (num3, hsv.z, num5),
            3 => (num3, num4, hsv.z),
            4 => (num5, num3, hsv.z),
            _ => (hsv.z, num3, num4),
        };
        super::Vector3::new(x, y, z)
    }

    #[inline]
    pub fn to_vector4(&self) -> super::Vector4 {
        super::Vector4::new(
            self.r() as f32 / 255.0,
            self.g() as f32 / 255.0,
            self.b() as f32 / 255.0,
            self.a() as f32 / 255.0,
        )
    }

    #[inline]
    pub fn to_vector3(&self) -> super::Vector3 {
        super::Vector3::new(
            self.r() as f32 / 255.0,
            self.g() as f32 / 255.0,
            self.b() as f32 / 255.0,
        )
    }
}

impl From<super::Vector4> for Color {
    fn from(v: super::Vector4) -> Self {
        Color::from_rgba_f(v.x, v.y, v.z, v.w)
    }
}

impl From<super::Vector3> for Color {
    fn from(v: super::Vector3) -> Self {
        Color::from_rgb_f(v.x, v.y, v.z)
    }
}

impl std::ops::Mul<f32> for Color {
    type Output = Color;
    fn mul(self, s: f32) -> Color {
        Color::from_rgba(
            clamp_f32(self.r() as f32 * s, 0.0, 255.0) as u8,
            clamp_f32(self.g() as f32 * s, 0.0, 255.0) as u8,
            clamp_f32(self.b() as f32 * s, 0.0, 255.0) as u8,
            clamp_f32(self.a() as f32 * s, 0.0, 255.0) as u8,
        )
    }
}

impl std::ops::Mul for Color {
    type Output = Color;
    fn mul(self, rhs: Color) -> Color {
        Color::from_rgba(
            (self.r() as u32 * rhs.r() as u32 / 255) as u8,
            (self.g() as u32 * rhs.g() as u32 / 255) as u8,
            (self.b() as u32 * rhs.b() as u32 / 255) as u8,
            (self.a() as u32 * rhs.a() as u32 / 255) as u8,
        )
    }
}

impl std::ops::Add for Color {
    type Output = Color;
    fn add(self, rhs: Color) -> Color {
        Color::from_rgba(
            (self.r() as u16 + rhs.r() as u16).min(255) as u8,
            (self.g() as u16 + rhs.g() as u16).min(255) as u8,
            (self.b() as u16 + rhs.b() as u16).min(255) as u8,
            (self.a() as u16 + rhs.a() as u16).min(255) as u8,
        )
    }
}

impl std::ops::Sub for Color {
    type Output = Color;
    fn sub(self, rhs: Color) -> Color {
        Color::from_rgba(
            (self.r() as i16 - rhs.r() as i16).max(0) as u8,
            (self.g() as i16 - rhs.g() as i16).max(0) as u8,
            (self.b() as i16 - rhs.b() as i16).max(0) as u8,
            (self.a() as i16 - rhs.a() as i16).max(0) as u8,
        )
    }
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{},{},{}", self.r(), self.g(), self.b(), self.a())
    }
}
