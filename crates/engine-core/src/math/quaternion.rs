use super::math_utils::*;
use super::vector3::Vector3;
use super::matrix::Matrix;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Quaternion {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Quaternion {
    pub const IDENTITY: Quaternion = Quaternion { x: 0.0, y: 0.0, z: 0.0, w: 1.0 };

    #[inline]
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Quaternion { x, y, z, w }
    }

    #[inline]
    pub fn from_vector3(v: Vector3, s: f32) -> Self {
        Quaternion { x: v.x, y: v.y, z: v.z, w: s }
    }

    #[inline]
    pub fn conjugate(q: Quaternion) -> Quaternion {
        Quaternion { x: -q.x, y: -q.y, z: -q.z, w: q.w }
    }

    pub fn from_axis_angle(axis: Vector3, angle: f32) -> Quaternion {
        let half = angle * 0.5;
        let sin = half.sin();
        let cos = half.cos();
        Quaternion {
            x: axis.x * sin,
            y: axis.y * sin,
            z: axis.z * sin,
            w: cos,
        }
    }

    pub fn from_rotation_matrix(m: &Matrix) -> Quaternion {
        let trace = m.m11 + m.m22 + m.m33;
        if trace > 0.0 {
            let s = (trace + 1.0).sqrt() * 0.5;
            let inv_s = 0.5 / s;
            Quaternion {
                x: (m.m23 - m.m32) * inv_s,
                y: (m.m31 - m.m13) * inv_s,
                z: (m.m12 - m.m21) * inv_s,
                w: s,
            }
        } else if m.m11 >= m.m22 && m.m11 >= m.m33 {
            let s = (1.0 + m.m11 - m.m22 - m.m33).sqrt();
            let inv_s = 0.5 / s;
            Quaternion {
                x: 0.5 * s,
                y: (m.m12 + m.m21) * inv_s,
                z: (m.m13 + m.m31) * inv_s,
                w: (m.m23 - m.m32) * inv_s,
            }
        } else if m.m22 > m.m33 {
            let s = (1.0 + m.m22 - m.m11 - m.m33).sqrt();
            let inv_s = 0.5 / s;
            Quaternion {
                x: (m.m21 + m.m12) * inv_s,
                y: 0.5 * s,
                z: (m.m32 + m.m23) * inv_s,
                w: (m.m31 - m.m13) * inv_s,
            }
        } else {
            let s = (1.0 + m.m33 - m.m11 - m.m22).sqrt();
            let inv_s = 0.5 / s;
            Quaternion {
                x: (m.m31 + m.m13) * inv_s,
                y: (m.m32 + m.m23) * inv_s,
                z: 0.5 * s,
                w: (m.m12 - m.m21) * inv_s,
            }
        }
    }

    pub fn from_yaw_pitch_roll(yaw: f32, pitch: f32, roll: f32) -> Quaternion {
        let (sr, cr) = ((roll * 0.5).sin(), (roll * 0.5).cos());
        let (sp, cp) = ((pitch * 0.5).sin(), (pitch * 0.5).cos());
        let (sy, cy) = ((yaw * 0.5).sin(), (yaw * 0.5).cos());
        Quaternion {
            x: cy * sp * cr + sy * cp * sr,
            y: sy * cp * cr - cy * sp * sr,
            z: cy * cp * sr - sy * sp * cr,
            w: cy * cp * cr + sy * sp * sr,
        }
    }

    #[inline]
    pub fn dot(q1: Quaternion, q2: Quaternion) -> f32 {
        q1.x * q2.x + q1.y * q2.y + q1.z * q2.z + q1.w * q2.w
    }

    pub fn inverse(q: Quaternion) -> Quaternion {
        let num = q.x * q.x + q.y * q.y + q.z * q.z + q.w * q.w;
        let inv = 1.0 / num;
        Quaternion {
            x: -q.x * inv,
            y: -q.y * inv,
            z: -q.z * inv,
            w: q.w * inv,
        }
    }

    #[inline]
    pub fn length(self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w).sqrt()
    }

    #[inline]
    pub fn length_squared(self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w
    }

    pub fn lerp(q1: Quaternion, q2: Quaternion, f: f32) -> Quaternion {
        let num = 1.0 - f;
        let dot = Quaternion::dot(q1, q2);
        let (a, b) = if dot >= 0.0 { (num, f) } else { (num, -f) };
        let mut result = Quaternion {
            x: a * q1.x + b * q2.x,
            y: a * q1.y + b * q2.y,
            z: a * q1.z + b * q2.z,
            w: a * q1.w + b * q2.w,
        };
        let inv_len = 1.0 / result.length();
        result.x *= inv_len;
        result.y *= inv_len;
        result.z *= inv_len;
        result.w *= inv_len;
        result
    }

    pub fn slerp(q1: Quaternion, q2: Quaternion, f: f32) -> Quaternion {
        let mut num = Quaternion::dot(q1, q2);
        let mut flag = false;
        if num < 0.0 {
            flag = true;
            num = -num;
        }
        let (num2, num3);
        if num > 0.999999 {
            num2 = 1.0 - f;
            num3 = if flag { -f } else { f };
        } else {
            let num4 = num.acos();
            let num5 = 1.0 / num4.sin();
            num2 = ((1.0 - f) * num4).sin() * num5;
            num3 = if flag {
                -(f * num4).sin() * num5
            } else {
                (f * num4).sin() * num5
            };
        }
        Quaternion {
            x: num2 * q1.x + num3 * q2.x,
            y: num2 * q1.y + num3 * q2.y,
            z: num2 * q1.z + num3 * q2.z,
            w: num2 * q1.w + num3 * q2.w,
        }
    }

    pub fn normalize(q: Quaternion) -> Quaternion {
        let len = q.length();
        if len == 0.0 {
            Quaternion::IDENTITY
        } else {
            q / len
        }
    }

    pub fn to_matrix(&self) -> Matrix {
        let (x, y, z, w) = (self.x, self.y, self.z, self.w);
        let x2 = x * x;
        let y2 = y * y;
        let z2 = z * z;
        let xy = x * y;
        let zw = z * w;
        let xz = x * z;
        let yw = y * w;
        let yz = y * z;
        let xw = x * w;
        Matrix::new(
            1.0 - 2.0 * (y2 + z2),
            2.0 * (xy + zw),
            2.0 * (xz - yw), 0.0,
            2.0 * (xy - zw),
            1.0 - 2.0 * (z2 + x2),
            2.0 * (yz + xw), 0.0,
            2.0 * (xz + yw),
            2.0 * (yz - xw),
            1.0 - 2.0 * (y2 + x2), 0.0,
            0.0, 0.0, 0.0, 1.0,
        )
    }

    pub fn get_right_vector(&self) -> Vector3 {
        Vector3::new(
            1.0 - 2.0 * (self.y * self.y + self.z * self.z),
            2.0 * (self.x * self.y + self.z * self.w),
            2.0 * (self.x * self.z - self.w * self.y),
        )
    }

    pub fn get_up_vector(&self) -> Vector3 {
        Vector3::new(
            2.0 * (self.x * self.y - self.z * self.w),
            1.0 - 2.0 * (self.x * self.x + self.z * self.z),
            2.0 * (self.y * self.z + self.x * self.w),
        )
    }

    pub fn get_forward_vector(&self) -> Vector3 {
        Vector3::new(
            -2.0 * (self.y * self.w + self.x * self.z),
            2.0 * (self.x * self.w - self.y * self.z),
            2.0 * (self.x * self.x + self.y * self.y) - 1.0,
        )
    }

    pub fn to_yaw_pitch_roll(&self) -> Vector3 {
        let num = -2.0 * (self.y * self.w + self.x * self.z);
        let x = 2.0 * (self.x * self.w - self.y * self.z);
        let num2 = 2.0 * (self.x * self.x + self.y * self.y) - 1.0;
        let y = 2.0 * (self.x * self.y + self.z * self.w);
        let x2 = 1.0 - 2.0 * (self.x * self.x + self.z * self.z);
        let x3 = (-num).atan2(-num2);
        let y2 = x.asin();
        let z = y.atan2(x2);
        Vector3::new(x3, y2, z)
    }

    pub fn fix_nan(q: Quaternion) -> Quaternion {
        Quaternion {
            x: if q.x.is_nan() { 0.0 } else { q.x },
            y: if q.y.is_nan() { 0.0 } else { q.y },
            z: if q.z.is_nan() { 0.0 } else { q.z },
            w: if q.w.is_nan() { 0.0 } else { q.w },
        }
    }
}

impl std::ops::Add for Quaternion {
    type Output = Quaternion;
    fn add(self, rhs: Quaternion) -> Quaternion {
        Quaternion::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z, self.w + rhs.w)
    }
}

impl std::ops::Sub for Quaternion {
    type Output = Quaternion;
    fn sub(self, rhs: Quaternion) -> Quaternion {
        Quaternion::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z, self.w - rhs.w)
    }
}

impl std::ops::Mul for Quaternion {
    type Output = Quaternion;
    fn mul(self, rhs: Quaternion) -> Quaternion {
        let (x, y, z, w) = (self.x, self.y, self.z, self.w);
        let (x2, y2, z2, w2) = (rhs.x, rhs.y, rhs.z, rhs.w);
        let n1 = y * z2 - z * y2;
        let n2 = z * x2 - x * z2;
        let n3 = x * y2 - y * x2;
        let n4 = x * x2 + y * y2 + z * z2;
        Quaternion::new(
            x * w2 + x2 * w + n1,
            y * w2 + y2 * w + n2,
            z * w2 + z2 * w + n3,
            w * w2 - n4,
        )
    }
}

impl std::ops::Mul<f32> for Quaternion {
    type Output = Quaternion;
    fn mul(self, s: f32) -> Quaternion {
        Quaternion::new(self.x * s, self.y * s, self.z * s, self.w * s)
    }
}

impl std::ops::Div for Quaternion {
    type Output = Quaternion;
    fn div(self, rhs: Quaternion) -> Quaternion {
        let (x, y, z, w) = (self.x, self.y, self.z, self.w);
        let num = rhs.x * rhs.x + rhs.y * rhs.y + rhs.z * rhs.z + rhs.w * rhs.w;
        let inv = 1.0 / num;
        let num3 = -rhs.x * inv;
        let num4 = -rhs.y * inv;
        let num5 = -rhs.z * inv;
        let num6 = rhs.w * inv;
        let num7 = y * num5 - z * num4;
        let num8 = z * num3 - x * num5;
        let num9 = x * num4 - y * num3;
        let num10 = x * num3 + y * num4 + z * num5;
        Quaternion::new(
            x * num6 + num3 * w + num7,
            y * num6 + num4 * w + num8,
            z * num6 + num5 * w + num9,
            w * num6 - num10,
        )
    }
}

impl std::ops::Div<f32> for Quaternion {
    type Output = Quaternion;
    fn div(self, d: f32) -> Quaternion {
        let inv = 1.0 / d;
        Quaternion::new(self.x * inv, self.y * inv, self.z * inv, self.w * inv)
    }
}

impl std::ops::Neg for Quaternion {
    type Output = Quaternion;
    fn neg(self) -> Quaternion {
        Quaternion::new(-self.x, -self.y, -self.z, -self.w)
    }
}

impl std::fmt::Display for Quaternion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{},{},{}", self.x, self.y, self.z, self.w)
    }
}
