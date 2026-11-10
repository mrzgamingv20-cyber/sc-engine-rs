use super::math_utils::*;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

impl Vector2 {
    pub const ZERO: Vector2 = Vector2 { x: 0.0, y: 0.0 };
    pub const ONE: Vector2 = Vector2 { x: 1.0, y: 1.0 };
    pub const UNIT_X: Vector2 = Vector2 { x: 1.0, y: 0.0 };
    pub const UNIT_Y: Vector2 = Vector2 { x: 0.0, y: 1.0 };

    #[inline]
    pub fn new(x: f32, y: f32) -> Self {
        Vector2 { x, y }
    }

    #[inline]
    pub fn splat(v: f32) -> Self {
        Vector2 { x: v, y: v }
    }

    #[inline]
    pub fn yx(self) -> Vector2 {
        Vector2 { x: self.y, y: self.x }
    }

    pub fn create_from_angle(angle: f32) -> Vector2 {
        let y = angle.cos();
        Vector2::new(-angle.sin(), y)
    }

    #[inline]
    pub fn distance(v1: Vector2, v2: Vector2) -> f32 {
        Vector2::distance_squared(v1, v2).sqrt()
    }

    #[inline]
    pub fn distance_squared(v1: Vector2, v2: Vector2) -> f32 {
        sqr_f32(v1.x - v2.x) + sqr_f32(v1.y - v2.y)
    }

    #[inline]
    pub fn dot(v1: Vector2, v2: Vector2) -> f32 {
        v1.x * v2.x + v1.y * v2.y
    }

    #[inline]
    pub fn cross(v1: Vector2, v2: Vector2) -> f32 {
        v1.x * v2.y - v1.y * v2.x
    }

    #[inline]
    pub fn perpendicular(v: Vector2) -> Vector2 {
        Vector2::new(-v.y, v.x)
    }

    pub fn rotate(v: Vector2, angle: f32) -> Vector2 {
        let cos = angle.cos();
        let sin = angle.sin();
        Vector2::new(cos * v.x + sin * v.y, -sin * v.x + cos * v.y)
    }

    #[inline]
    pub fn length(self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    #[inline]
    pub fn length_squared(self) -> f32 {
        self.x * self.x + self.y * self.y
    }

    #[inline]
    pub fn floor(v: Vector2) -> Vector2 {
        Vector2::new(v.x.floor(), v.y.floor())
    }

    #[inline]
    pub fn ceiling(v: Vector2) -> Vector2 {
        Vector2::new(v.x.ceil(), v.y.ceil())
    }

    #[inline]
    pub fn round(v: Vector2) -> Vector2 {
        Vector2::new(v.x.round(), v.y.round())
    }

    #[inline]
    pub fn min(v: Vector2, f: f32) -> Vector2 {
        Vector2::new(v.x.min(f), v.y.min(f))
    }

    #[inline]
    pub fn min_vec(v1: Vector2, v2: Vector2) -> Vector2 {
        Vector2::new(v1.x.min(v2.x), v1.y.min(v2.y))
    }

    #[inline]
    pub fn max(v: Vector2, f: f32) -> Vector2 {
        Vector2::new(v.x.max(f), v.y.max(f))
    }

    #[inline]
    pub fn max_vec(v1: Vector2, v2: Vector2) -> Vector2 {
        Vector2::new(v1.x.max(v2.x), v1.y.max(v2.y))
    }

    #[inline]
    pub fn clamp(v: Vector2, min: f32, max: f32) -> Vector2 {
        Vector2::new(clamp_f32(v.x, min, max), clamp_f32(v.y, min, max))
    }

    #[inline]
    pub fn saturate(v: Vector2) -> Vector2 {
        Vector2::new(saturate(v.x), saturate(v.y))
    }

    #[inline]
    pub fn lerp(v1: Vector2, v2: Vector2, f: f32) -> Vector2 {
        Vector2::new(lerp(v1.x, v2.x, f), lerp(v1.y, v2.y, f))
    }

    #[inline]
    pub fn catmull_rom(v1: Vector2, v2: Vector2, v3: Vector2, v4: Vector2, f: f32) -> Vector2 {
        Vector2::new(
            catmull_rom(v1.x, v2.x, v3.x, v4.x, f),
            catmull_rom(v1.y, v2.y, v3.y, v4.y, f),
        )
    }

    pub fn normalize(v: Vector2) -> Vector2 {
        let len = v.length();
        if !(len > 0.0) {
            Vector2::UNIT_X
        } else {
            v / len
        }
    }

    pub fn limit_length(v: Vector2, max_length: f32) -> Vector2 {
        let len_sq = v.length_squared();
        if len_sq > max_length * max_length {
            v * (max_length / len_sq.sqrt())
        } else {
            v
        }
    }

    pub fn angle(v1: Vector2, v2: Vector2) -> f32 {
        let a1 = v1.y.atan2(v1.x);
        let mut diff = v2.y.atan2(v2.x) - a1;
        if diff > PI {
            diff -= PI * 2.0;
        } else if diff <= -PI {
            diff += PI * 2.0;
        }
        diff
    }

    pub fn transform(v: Vector2, m: &super::Matrix) -> Vector2 {
        Vector2::new(
            v.x * m.m11 + v.y * m.m21 + m.m41,
            v.x * m.m12 + v.y * m.m22 + m.m42,
        )
    }

    pub fn transform_quat(v: Vector2, q: &super::Quaternion) -> Vector2 {
        let num = q.x + q.x;
        let num2 = q.y + q.y;
        let num3 = q.z + q.z;
        let num4 = q.w * num3;
        let num5 = q.x * num;
        let num6 = q.x * num2;
        let num7 = q.y * num2;
        let num8 = q.z * num3;
        Vector2::new(
            v.x * (1.0 - num7 - num8) + v.y * (num6 - num4),
            v.x * (num6 + num4) + v.y * (1.0 - num5 - num8),
        )
    }

    pub fn transform_normal(v: Vector2, m: &super::Matrix) -> Vector2 {
        Vector2::new(
            v.x * m.m11 + v.y * m.m21,
            v.x * m.m12 + v.y * m.m22,
        )
    }

    pub fn fix_nan(v: Vector2) -> Vector2 {
        Vector2::new(
            if v.x.is_nan() { 0.0 } else { v.x },
            if v.y.is_nan() { 0.0 } else { v.y },
        )
    }

    #[inline]
    pub fn as_slice(&self) -> &[f32; 2] {
        unsafe { &*(self as *const Vector2 as *const [f32; 2]) }
    }
}

impl std::ops::Add for Vector2 {
    type Output = Vector2;
    #[inline]
    fn add(self, rhs: Vector2) -> Vector2 {
        Vector2::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl std::ops::Sub for Vector2 {
    type Output = Vector2;
    #[inline]
    fn sub(self, rhs: Vector2) -> Vector2 {
        Vector2::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl std::ops::Mul for Vector2 {
    type Output = Vector2;
    #[inline]
    fn mul(self, rhs: Vector2) -> Vector2 {
        Vector2::new(self.x * rhs.x, self.y * rhs.y)
    }
}

impl std::ops::Mul<f32> for Vector2 {
    type Output = Vector2;
    #[inline]
    fn mul(self, s: f32) -> Vector2 {
        Vector2::new(self.x * s, self.y * s)
    }
}

impl std::ops::Mul<Vector2> for f32 {
    type Output = Vector2;
    #[inline]
    fn mul(self, v: Vector2) -> Vector2 {
        Vector2::new(v.x * self, v.y * self)
    }
}

impl std::ops::Div for Vector2 {
    type Output = Vector2;
    #[inline]
    fn div(self, rhs: Vector2) -> Vector2 {
        Vector2::new(self.x / rhs.x, self.y / rhs.y)
    }
}

impl std::ops::Div<f32> for Vector2 {
    type Output = Vector2;
    #[inline]
    fn div(self, d: f32) -> Vector2 {
        let inv = 1.0 / d;
        Vector2::new(self.x * inv, self.y * inv)
    }
}

impl std::ops::Neg for Vector2 {
    type Output = Vector2;
    #[inline]
    fn neg(self) -> Vector2 {
        Vector2::new(-self.x, -self.y)
    }
}

impl std::fmt::Display for Vector2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{}", self.x, self.y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let v = Vector2::new(3.0, 4.0);
        assert_eq!(v.length(), 5.0);
        assert_eq!(Vector2::dot(v, v), 25.0);
        let n = Vector2::normalize(v);
        assert!((n.length() - 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_lerp() {
        let a = Vector2::new(0.0, 0.0);
        let b = Vector2::new(10.0, 20.0);
        let l = Vector2::lerp(a, b, 0.5);
        assert_eq!(l, Vector2::new(5.0, 10.0));
    }

    #[test]
    fn test_rotate() {
        let v = Vector2::new(1.0, 0.0);
        let r = Vector2::rotate(v, PI / 2.0);
        assert!((r.x - 0.0).abs() < 1e-6);
        assert!((r.y - (-1.0)).abs() < 1e-6);
    }
}
