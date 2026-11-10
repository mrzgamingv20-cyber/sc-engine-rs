use super::math_utils::*;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3 {
    pub const ZERO: Vector3 = Vector3 { x: 0.0, y: 0.0, z: 0.0 };
    pub const ONE: Vector3 = Vector3 { x: 1.0, y: 1.0, z: 1.0 };
    pub const UNIT_X: Vector3 = Vector3 { x: 1.0, y: 0.0, z: 0.0 };
    pub const UNIT_Y: Vector3 = Vector3 { x: 0.0, y: 1.0, z: 0.0 };
    pub const UNIT_Z: Vector3 = Vector3 { x: 0.0, y: 0.0, z: 1.0 };

    #[inline]
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vector3 { x, y, z }
    }

    #[inline]
    pub fn splat(v: f32) -> Self {
        Vector3 { x: v, y: v, z: v }
    }

    #[inline]
    pub fn xy(self) -> super::Vector2 {
        super::Vector2::new(self.x, self.y)
    }

    #[inline]
    pub fn xz(self) -> super::Vector2 {
        super::Vector2::new(self.x, self.z)
    }

    #[inline]
    pub fn yz(self) -> super::Vector2 {
        super::Vector2::new(self.y, self.z)
    }

    #[inline]
    pub fn distance(v1: Vector3, v2: Vector3) -> f32 {
        Vector3::distance_squared(v1, v2).sqrt()
    }

    #[inline]
    pub fn distance_squared(v1: Vector3, v2: Vector3) -> f32 {
        sqr_f32(v1.x - v2.x) + sqr_f32(v1.y - v2.y) + sqr_f32(v1.z - v2.z)
    }

    #[inline]
    pub fn dot(v1: Vector3, v2: Vector3) -> f32 {
        v1.x * v2.x + v1.y * v2.y + v1.z * v2.z
    }

    #[inline]
    pub fn cross(v1: Vector3, v2: Vector3) -> Vector3 {
        Vector3::new(
            v1.y * v2.z - v1.z * v2.y,
            v1.z * v2.x - v1.x * v2.z,
            v1.x * v2.y - v1.y * v2.x,
        )
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
    pub fn floor(v: Vector3) -> Vector3 {
        Vector3::new(v.x.floor(), v.y.floor(), v.z.floor())
    }

    #[inline]
    pub fn ceiling(v: Vector3) -> Vector3 {
        Vector3::new(v.x.ceil(), v.y.ceil(), v.z.ceil())
    }

    #[inline]
    pub fn round(v: Vector3) -> Vector3 {
        Vector3::new(v.x.round(), v.y.round(), v.z.round())
    }

    #[inline]
    pub fn min(v: Vector3, f: f32) -> Vector3 {
        Vector3::new(v.x.min(f), v.y.min(f), v.z.min(f))
    }

    #[inline]
    pub fn min_vec(v1: Vector3, v2: Vector3) -> Vector3 {
        Vector3::new(v1.x.min(v2.x), v1.y.min(v2.y), v1.z.min(v2.z))
    }

    #[inline]
    pub fn max(v: Vector3, f: f32) -> Vector3 {
        Vector3::new(v.x.max(f), v.y.max(f), v.z.max(f))
    }

    #[inline]
    pub fn max_vec(v1: Vector3, v2: Vector3) -> Vector3 {
        Vector3::new(v1.x.max(v2.x), v1.y.max(v2.y), v1.z.max(v2.z))
    }

    #[inline]
    pub fn clamp(v: Vector3, min: f32, max: f32) -> Vector3 {
        Vector3::new(
            clamp_f32(v.x, min, max),
            clamp_f32(v.y, min, max),
            clamp_f32(v.z, min, max),
        )
    }

    #[inline]
    pub fn saturate(v: Vector3) -> Vector3 {
        Vector3::new(saturate(v.x), saturate(v.y), saturate(v.z))
    }

    #[inline]
    pub fn lerp(v1: Vector3, v2: Vector3, f: f32) -> Vector3 {
        Vector3::new(lerp(v1.x, v2.x, f), lerp(v1.y, v2.y, f), lerp(v1.z, v2.z, f))
    }

    #[inline]
    pub fn catmull_rom(v1: Vector3, v2: Vector3, v3: Vector3, v4: Vector3, f: f32) -> Vector3 {
        Vector3::new(
            catmull_rom(v1.x, v2.x, v3.x, v4.x, f),
            catmull_rom(v1.y, v2.y, v3.y, v4.y, f),
            catmull_rom(v1.z, v2.z, v3.z, v4.z, f),
        )
    }

    pub fn normalize(v: Vector3) -> Vector3 {
        let len = v.length();
        if len <= 0.0 {
            Vector3::UNIT_X
        } else {
            v / len
        }
    }

    pub fn limit_length(v: Vector3, max_length: f32) -> Vector3 {
        let len_sq = v.length_squared();
        if len_sq > max_length * max_length {
            v * (max_length / len_sq.sqrt())
        } else {
            v
        }
    }

    pub fn transform(v: Vector3, m: &super::Matrix) -> Vector3 {
        Vector3::new(
            v.x * m.m11 + v.y * m.m21 + v.z * m.m31 + m.m41,
            v.x * m.m12 + v.y * m.m22 + v.z * m.m32 + m.m42,
            v.x * m.m13 + v.y * m.m23 + v.z * m.m33 + m.m43,
        )
    }

    pub fn transform_quat(v: Vector3, q: &super::Quaternion) -> Vector3 {
        let num = q.x + q.x;
        let num2 = q.y + q.y;
        let num3 = q.z + q.z;
        let num4 = q.w * num;
        let num5 = q.w * num2;
        let num6 = q.w * num3;
        let num7 = q.x * num;
        let num8 = q.x * num2;
        let num9 = q.x * num3;
        let num10 = q.y * num2;
        let num11 = q.y * num3;
        let num12 = q.z * num3;
        Vector3::new(
            v.x * (1.0 - num10 - num12) + v.y * (num8 - num6) + v.z * (num9 + num5),
            v.x * (num8 + num6) + v.y * (1.0 - num7 - num12) + v.z * (num11 - num4),
            v.x * (num9 - num5) + v.y * (num11 + num4) + v.z * (1.0 - num7 - num10),
        )
    }

    pub fn transform_normal(v: Vector3, m: &super::Matrix) -> Vector3 {
        Vector3::new(
            v.x * m.m11 + v.y * m.m21 + v.z * m.m31,
            v.x * m.m12 + v.y * m.m22 + v.z * m.m32,
            v.x * m.m13 + v.y * m.m23 + v.z * m.m33,
        )
    }

    pub fn fix_nan(v: Vector3) -> Vector3 {
        Vector3::new(
            if v.x.is_nan() { 0.0 } else { v.x },
            if v.y.is_nan() { 0.0 } else { v.y },
            if v.z.is_nan() { 0.0 } else { v.z },
        )
    }

    #[inline]
    pub fn as_slice(&self) -> &[f32; 3] {
        unsafe { &*(self as *const Vector3 as *const [f32; 3]) }
    }
}

impl From<super::Vector2> for Vector3 {
    fn from(v: super::Vector2) -> Self {
        Vector3::new(v.x, v.y, 0.0)
    }
}

impl From<super::Point3> for Vector3 {
    fn from(p: super::Point3) -> Self {
        Vector3::new(p.x as f32, p.y as f32, p.z as f32)
    }
}

impl std::ops::Add for Vector3 {
    type Output = Vector3;
    #[inline]
    fn add(self, rhs: Vector3) -> Vector3 {
        Vector3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl std::ops::Sub for Vector3 {
    type Output = Vector3;
    #[inline]
    fn sub(self, rhs: Vector3) -> Vector3 {
        Vector3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl std::ops::Mul for Vector3 {
    type Output = Vector3;
    #[inline]
    fn mul(self, rhs: Vector3) -> Vector3 {
        Vector3::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }
}

impl std::ops::Mul<f32> for Vector3 {
    type Output = Vector3;
    #[inline]
    fn mul(self, s: f32) -> Vector3 {
        Vector3::new(self.x * s, self.y * s, self.z * s)
    }
}

impl std::ops::Mul<Vector3> for f32 {
    type Output = Vector3;
    #[inline]
    fn mul(self, v: Vector3) -> Vector3 {
        Vector3::new(v.x * self, v.y * self, v.z * self)
    }
}

impl std::ops::Div for Vector3 {
    type Output = Vector3;
    #[inline]
    fn div(self, rhs: Vector3) -> Vector3 {
        Vector3::new(self.x / rhs.x, self.y / rhs.y, self.z / rhs.z)
    }
}

impl std::ops::Div<f32> for Vector3 {
    type Output = Vector3;
    #[inline]
    fn div(self, d: f32) -> Vector3 {
        let inv = 1.0 / d;
        Vector3::new(self.x * inv, self.y * inv, self.z * inv)
    }
}

impl std::ops::DivAssign<f32> for Vector3 {
    #[inline]
    fn div_assign(&mut self, d: f32) {
        let inv = 1.0 / d;
        self.x *= inv;
        self.y *= inv;
        self.z *= inv;
    }
}

impl std::ops::Neg for Vector3 {
    type Output = Vector3;
    #[inline]
    fn neg(self) -> Vector3 {
        Vector3::new(-self.x, -self.y, -self.z)
    }
}

impl std::fmt::Display for Vector3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{},{}", self.x, self.y, self.z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let v = Vector3::new(1.0, 2.0, 2.0);
        assert_eq!(v.length(), 3.0);
        assert_eq!(Vector3::dot(v, v), 9.0);
    }

    #[test]
    fn test_cross() {
        let c = Vector3::cross(Vector3::UNIT_X, Vector3::UNIT_Y);
        assert_eq!(c, Vector3::UNIT_Z);
    }

    #[test]
    fn test_normalize() {
        let n = Vector3::normalize(Vector3::new(10.0, 0.0, 0.0));
        assert_eq!(n, Vector3::UNIT_X);
    }
}
