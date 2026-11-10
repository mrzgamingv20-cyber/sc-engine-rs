use super::math_utils::*;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Vector4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Vector4 {
    pub const ZERO: Vector4 = Vector4 { x: 0.0, y: 0.0, z: 0.0, w: 0.0 };
    pub const ONE: Vector4 = Vector4 { x: 1.0, y: 1.0, z: 1.0, w: 1.0 };
    pub const UNIT_X: Vector4 = Vector4 { x: 1.0, y: 0.0, z: 0.0, w: 0.0 };
    pub const UNIT_Y: Vector4 = Vector4 { x: 0.0, y: 1.0, z: 0.0, w: 0.0 };
    pub const UNIT_Z: Vector4 = Vector4 { x: 0.0, y: 0.0, z: 1.0, w: 0.0 };
    pub const UNIT_W: Vector4 = Vector4 { x: 0.0, y: 0.0, z: 0.0, w: 1.0 };

    #[inline]
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Vector4 { x, y, z, w }
    }

    #[inline]
    pub fn splat(v: f32) -> Self {
        Vector4 { x: v, y: v, z: v, w: v }
    }

    #[inline]
    pub fn from_xyz(v: super::Vector3, w: f32) -> Self {
        Vector4 { x: v.x, y: v.y, z: v.z, w }
    }

    #[inline]
    pub fn xyz(self) -> super::Vector3 {
        super::Vector3::new(self.x, self.y, self.z)
    }

    #[inline]
    pub fn xy(self) -> super::Vector2 {
        super::Vector2::new(self.x, self.y)
    }

    #[inline]
    pub fn distance(v1: Vector4, v2: Vector4) -> f32 {
        Vector4::distance_squared(v1, v2).sqrt()
    }

    #[inline]
    pub fn distance_squared(v1: Vector4, v2: Vector4) -> f32 {
        sqr_f32(v1.x - v2.x) + sqr_f32(v1.y - v2.y) + sqr_f32(v1.z - v2.z) + sqr_f32(v1.w - v2.w)
    }

    #[inline]
    pub fn dot(v1: Vector4, v2: Vector4) -> f32 {
        v1.x * v2.x + v1.y * v2.y + v1.z * v2.z + v1.w * v2.w
    }

    #[inline]
    pub fn length(self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    #[inline]
    pub fn length_squared(self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    #[inline]
    pub fn floor(v: Vector4) -> Vector4 {
        Vector4::new(v.x.floor(), v.y.floor(), v.z.floor(), v.w.floor())
    }

    #[inline]
    pub fn ceiling(v: Vector4) -> Vector4 {
        Vector4::new(v.x.ceil(), v.y.ceil(), v.z.ceil(), v.w.ceil())
    }

    #[inline]
    pub fn round(v: Vector4) -> Vector4 {
        Vector4::new(v.x.round(), v.y.round(), v.z.round(), v.w.round())
    }

    #[inline]
    pub fn min(v: Vector4, f: f32) -> Vector4 {
        Vector4::new(v.x.min(f), v.y.min(f), v.z.min(f), v.w.min(f))
    }

    #[inline]
    pub fn min_vec(v1: Vector4, v2: Vector4) -> Vector4 {
        Vector4::new(v1.x.min(v2.x), v1.y.min(v2.y), v1.z.min(v2.z), v1.w.min(v2.w))
    }

    #[inline]
    pub fn max(v: Vector4, f: f32) -> Vector4 {
        Vector4::new(v.x.max(f), v.y.max(f), v.z.max(f), v.w.max(f))
    }

    #[inline]
    pub fn max_vec(v1: Vector4, v2: Vector4) -> Vector4 {
        Vector4::new(v1.x.max(v2.x), v1.y.max(v2.y), v1.z.max(v2.z), v1.w.max(v2.w))
    }

    #[inline]
    pub fn clamp(v: Vector4, min: f32, max: f32) -> Vector4 {
        Vector4::new(
            clamp_f32(v.x, min, max),
            clamp_f32(v.y, min, max),
            clamp_f32(v.z, min, max),
            clamp_f32(v.w, min, max),
        )
    }

    #[inline]
    pub fn saturate(v: Vector4) -> Vector4 {
        Vector4::new(saturate(v.x), saturate(v.y), saturate(v.z), saturate(v.w))
    }

    #[inline]
    pub fn lerp(v1: Vector4, v2: Vector4, f: f32) -> Vector4 {
        Vector4::new(
            lerp(v1.x, v2.x, f),
            lerp(v1.y, v2.y, f),
            lerp(v1.z, v2.z, f),
            lerp(v1.w, v2.w, f),
        )
    }

    #[inline]
    pub fn catmull_rom(v1: Vector4, v2: Vector4, v3: Vector4, v4: Vector4, f: f32) -> Vector4 {
        Vector4::new(
            catmull_rom(v1.x, v2.x, v3.x, v4.x, f),
            catmull_rom(v1.y, v2.y, v3.y, v4.y, f),
            catmull_rom(v1.z, v2.z, v3.z, v4.z, f),
            catmull_rom(v1.w, v2.w, v3.w, v4.w, f),
        )
    }

    pub fn normalize(v: Vector4) -> Vector4 {
        let len = v.length();
        if !(len > 0.0) {
            Vector4::UNIT_X
        } else {
            v / len
        }
    }

    pub fn limit_length(v: Vector4, max_length: f32) -> Vector4 {
        let len_sq = v.length_squared();
        if len_sq > max_length * max_length {
            v * (max_length / len_sq.sqrt())
        } else {
            v
        }
    }

    pub fn transform(v: Vector4, m: &super::Matrix) -> Vector4 {
        Vector4::new(
            v.x * m.m11 + v.y * m.m21 + v.z * m.m31 + m.m41,
            v.x * m.m12 + v.y * m.m22 + v.z * m.m32 + m.m42,
            v.x * m.m13 + v.y * m.m23 + v.z * m.m33 + m.m43,
            v.x * m.m14 + v.y * m.m24 + v.z * m.m34 + m.m44,
        )
    }

    pub fn fix_nan(v: Vector4) -> Vector4 {
        Vector4::new(
            if v.x.is_nan() { 0.0 } else { v.x },
            if v.y.is_nan() { 0.0 } else { v.y },
            if v.z.is_nan() { 0.0 } else { v.z },
            if v.w.is_nan() { 0.0 } else { v.w },
        )
    }

    #[inline]
    pub fn as_slice(&self) -> &[f32; 4] {
        unsafe { &*(self as *const Vector4 as *const [f32; 4]) }
    }
}

impl From<super::Vector3> for Vector4 {
    fn from(v: super::Vector3) -> Self {
        Vector4 { x: v.x, y: v.y, z: v.z, w: 0.0 }
    }
}

impl std::ops::Add for Vector4 {
    type Output = Vector4;
    #[inline]
    fn add(self, rhs: Vector4) -> Vector4 {
        Vector4::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z, self.w + rhs.w)
    }
}

impl std::ops::Sub for Vector4 {
    type Output = Vector4;
    #[inline]
    fn sub(self, rhs: Vector4) -> Vector4 {
        Vector4::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z, self.w - rhs.w)
    }
}

impl std::ops::Mul for Vector4 {
    type Output = Vector4;
    #[inline]
    fn mul(self, rhs: Vector4) -> Vector4 {
        Vector4::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z, self.w * rhs.w)
    }
}

impl std::ops::Mul<f32> for Vector4 {
    type Output = Vector4;
    #[inline]
    fn mul(self, s: f32) -> Vector4 {
        Vector4::new(self.x * s, self.y * s, self.z * s, self.w * s)
    }
}

impl std::ops::Mul<Vector4> for f32 {
    type Output = Vector4;
    #[inline]
    fn mul(self, v: Vector4) -> Vector4 {
        Vector4::new(v.x * self, v.y * self, v.z * self, v.w * self)
    }
}

impl std::ops::Div for Vector4 {
    type Output = Vector4;
    #[inline]
    fn div(self, rhs: Vector4) -> Vector4 {
        Vector4::new(self.x / rhs.x, self.y / rhs.y, self.z / rhs.z, self.w / rhs.w)
    }
}

impl std::ops::Div<f32> for Vector4 {
    type Output = Vector4;
    #[inline]
    fn div(self, d: f32) -> Vector4 {
        let inv = 1.0 / d;
        Vector4::new(self.x * inv, self.y * inv, self.z * inv, self.w * inv)
    }
}

impl std::ops::Neg for Vector4 {
    type Output = Vector4;
    #[inline]
    fn neg(self) -> Vector4 {
        Vector4::new(-self.x, -self.y, -self.z, -self.w)
    }
}

impl std::fmt::Display for Vector4 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{},{},{}", self.x, self.y, self.z, self.w)
    }
}
