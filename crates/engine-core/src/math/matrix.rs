use super::vector3::Vector3;
use super::quaternion::Quaternion;

/// 4x4 matrix, column-major storage matching the C# Engine layout.
/// Memory layout: M11,M12,M13,M14, M21,M22,M23,M24, M31,M32,M33,M34, M41,M42,M43,M44
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Matrix {
    pub m11: f32, pub m12: f32, pub m13: f32, pub m14: f32,
    pub m21: f32, pub m22: f32, pub m23: f32, pub m24: f32,
    pub m31: f32, pub m32: f32, pub m33: f32, pub m34: f32,
    pub m41: f32, pub m42: f32, pub m43: f32, pub m44: f32,
}

impl Matrix {
    pub const ZERO: Matrix = Matrix {
        m11: 0.0, m12: 0.0, m13: 0.0, m14: 0.0,
        m21: 0.0, m22: 0.0, m23: 0.0, m24: 0.0,
        m31: 0.0, m32: 0.0, m33: 0.0, m34: 0.0,
        m41: 0.0, m42: 0.0, m43: 0.0, m44: 0.0,
    };

    pub const IDENTITY: Matrix = Matrix {
        m11: 1.0, m12: 0.0, m13: 0.0, m14: 0.0,
        m21: 0.0, m22: 1.0, m23: 0.0, m24: 0.0,
        m31: 0.0, m32: 0.0, m33: 1.0, m34: 0.0,
        m41: 0.0, m42: 0.0, m43: 0.0, m44: 1.0,
    };

    #[allow(clippy::too_many_arguments)]
    pub fn new(
        m11: f32, m12: f32, m13: f32, m14: f32,
        m21: f32, m22: f32, m23: f32, m24: f32,
        m31: f32, m32: f32, m33: f32, m34: f32,
        m41: f32, m42: f32, m43: f32, m44: f32,
    ) -> Self {
        Matrix { m11, m12, m13, m14, m21, m22, m23, m24, m31, m32, m33, m34, m41, m42, m43, m44 }
    }

    pub fn right(&self) -> Vector3 {
        Vector3::new(self.m11, self.m12, self.m13)
    }

    pub fn set_right(&mut self, v: Vector3) {
        self.m11 = v.x;
        self.m12 = v.y;
        self.m13 = v.z;
    }

    pub fn up(&self) -> Vector3 {
        Vector3::new(self.m21, self.m22, self.m23)
    }

    pub fn set_up(&mut self, v: Vector3) {
        self.m21 = v.x;
        self.m22 = v.y;
        self.m23 = v.z;
    }

    pub fn forward(&self) -> Vector3 {
        Vector3::new(-self.m31, -self.m32, -self.m33)
    }

    pub fn set_forward(&mut self, v: Vector3) {
        self.m31 = -v.x;
        self.m32 = -v.y;
        self.m33 = -v.z;
    }

    pub fn translation(&self) -> Vector3 {
        Vector3::new(self.m41, self.m42, self.m43)
    }

    pub fn set_translation(&mut self, v: Vector3) {
        self.m41 = v.x;
        self.m42 = v.y;
        self.m43 = v.z;
    }

    pub fn scale(&self) -> Vector3 {
        let sx = if self.m11 * self.m12 * self.m13 * self.m14 >= 0.0 { 1.0 } else { -1.0 };
        let sy = if self.m21 * self.m22 * self.m23 * self.m24 >= 0.0 { 1.0 } else { -1.0 };
        let sz = if self.m31 * self.m32 * self.m33 * self.m34 >= 0.0 { 1.0 } else { -1.0 };
        let x = sx * (self.m11 * self.m11 + self.m12 * self.m12 + self.m13 * self.m13).sqrt();
        if x == 0.0 { return Vector3::ZERO; }
        let y = sy * (self.m21 * self.m21 + self.m22 * self.m22 + self.m23 * self.m23).sqrt();
        if y == 0.0 { return Vector3::ZERO; }
        let z = sz * (self.m31 * self.m31 + self.m32 * self.m32 + self.m33 * self.m33).sqrt();
        if z == 0.0 { return Vector3::ZERO; }
        Vector3::new(x, y, z)
    }

    pub fn determinant(&self) -> f32 {
        let n = self.m33 * self.m44 - self.m34 * self.m43;
        let n2 = self.m32 * self.m44 - self.m34 * self.m42;
        let n3 = self.m32 * self.m43 - self.m33 * self.m42;
        let n4 = self.m31 * self.m44 - self.m34 * self.m41;
        let n5 = self.m31 * self.m43 - self.m33 * self.m41;
        let n6 = self.m31 * self.m42 - self.m32 * self.m41;
        self.m11 * (self.m22 * n - self.m23 * n2 + self.m24 * n3)
            - self.m12 * (self.m21 * n - self.m23 * n4 + self.m24 * n5)
            + self.m13 * (self.m21 * n2 - self.m22 * n4 + self.m24 * n6)
            - self.m14 * (self.m21 * n3 - self.m22 * n5 + self.m23 * n6)
    }

    pub fn decompose(&self) -> Option<(Vector3, Quaternion, Vector3)> {
        let translation = Vector3::new(self.m41, self.m42, self.m43);
        let sx_sign = if self.m11 * self.m12 * self.m13 * self.m14 < 0.0 { -1.0 } else { 1.0 };
        let sy_sign = if self.m21 * self.m22 * self.m23 * self.m24 < 0.0 { -1.0 } else { 1.0 };
        let sz_sign = if self.m31 * self.m32 * self.m33 * self.m34 < 0.0 { -1.0 } else { 1.0 };
        let scale = Vector3::new(
            sx_sign * (self.m11 * self.m11 + self.m12 * self.m12 + self.m13 * self.m13).sqrt(),
            sy_sign * (self.m21 * self.m21 + self.m22 * self.m22 + self.m23 * self.m23).sqrt(),
            sz_sign * (self.m31 * self.m31 + self.m32 * self.m32 + self.m33 * self.m33).sqrt(),
        );
        if scale.x == 0.0 || scale.y == 0.0 || scale.z == 0.0 {
            return Some((scale, Quaternion::IDENTITY, translation));
        }
        let rot_matrix = Matrix::new(
            self.m11 / scale.x, self.m12 / scale.x, self.m13 / scale.x, 0.0,
            self.m21 / scale.y, self.m22 / scale.y, self.m23 / scale.y, 0.0,
            self.m31 / scale.z, self.m32 / scale.z, self.m33 / scale.z, 0.0,
            0.0, 0.0, 0.0, 1.0,
        );
        let rotation = Quaternion::from_rotation_matrix(&rot_matrix);
        Some((scale, rotation, translation))
    }

    pub fn from_axis_angle(axis: Vector3, angle: f32) -> Matrix {
        let (x, y, z) = (axis.x, axis.y, axis.z);
        let sin = angle.sin();
        let cos = angle.cos();
        let x2 = x * x;
        let y2 = y * y;
        let z2 = z * z;
        let xy = x * y;
        let xz = x * z;
        let yz = y * z;
        Matrix::new(
            x2 + cos * (1.0 - x2),
            xy - cos * xy + sin * z,
            xz - cos * xz - sin * y, 0.0,
            xy - cos * xy - sin * z,
            y2 + cos * (1.0 - y2),
            yz - cos * yz + sin * x, 0.0,
            xz - cos * xz + sin * y,
            yz - cos * yz - sin * x,
            z2 + cos * (1.0 - z2), 0.0,
            0.0, 0.0, 0.0, 1.0,
        )
    }

    pub fn from_quaternion(q: &Quaternion) -> Matrix {
        q.to_matrix()
    }

    pub fn from_yaw_pitch_roll(yaw: f32, pitch: f32, roll: f32) -> Matrix {
        Quaternion::from_yaw_pitch_roll(yaw, pitch, roll).to_matrix()
    }

    pub fn create_look_at(position: Vector3, target: Vector3, up: Vector3) -> Matrix {
        let z = Vector3::normalize(position - target);
        let x = Vector3::normalize(Vector3::cross(up, z));
        let up = Vector3::normalize(Vector3::cross(z, x));
        Matrix::new(
            x.x, up.x, z.x, 0.0,
            x.y, up.y, z.y, 0.0,
            x.z, up.z, z.z, 0.0,
            -Vector3::dot(x, position),
            -Vector3::dot(up, position),
            -Vector3::dot(z, position),
            1.0,
        )
    }

    pub fn create_orthographic(width: f32, height: f32, near: f32, far: f32) -> Matrix {
        Matrix::new(
            2.0 / width, 0.0, 0.0, 0.0,
            0.0, 2.0 / height, 0.0, 0.0,
            0.0, 0.0, 1.0 / (near - far), 0.0,
            0.0, 0.0, near / (near - far), 1.0,
        )
    }

    pub fn create_orthographic_off_center(left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) -> Matrix {
        Matrix::new(
            2.0 / (right - left), 0.0, 0.0, 0.0,
            0.0, 2.0 / (top - bottom), 0.0, 0.0,
            0.0, 0.0, 1.0 / (near - far), 0.0,
            (left + right) / (left - right),
            (top + bottom) / (bottom - top),
            near / (near - far), 1.0,
        )
    }

    pub fn create_perspective(width: f32, height: f32, near: f32, far: f32) -> Matrix {
        Matrix::new(
            2.0 * near / width, 0.0, 0.0, 0.0,
            0.0, 2.0 * near / height, 0.0, 0.0,
            0.0, 0.0, far / (near - far), -1.0,
            0.0, 0.0, near * far / (near - far), 0.0,
        )
    }

    pub fn create_perspective_field_of_view(fov_y: f32, aspect: f32, near: f32, far: f32) -> Matrix {
        let num = 1.0 / (fov_y * 0.5).tan();
        Matrix::new(
            num / aspect, 0.0, 0.0, 0.0,
            0.0, num, 0.0, 0.0,
            0.0, 0.0, far / (near - far), -1.0,
            0.0, 0.0, near * far / (near - far), 0.0,
        )
    }

    pub fn create_perspective_off_center(left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) -> Matrix {
        Matrix::new(
            2.0 * near / (right - left), 0.0, 0.0, 0.0,
            0.0, 2.0 * near / (top - bottom), 0.0, 0.0,
            (left + right) / (right - left),
            (top + bottom) / (top - bottom),
            far / (near - far), -1.0,
            0.0, 0.0,
            near * far / (near - far), 0.0,
        )
    }

    pub fn create_rotation_x(radians: f32) -> Matrix {
        let (cos, sin) = (radians.cos(), radians.sin());
        Matrix::new(
            1.0, 0.0, 0.0, 0.0,
            0.0, cos, sin, 0.0,
            0.0, -sin, cos, 0.0,
            0.0, 0.0, 0.0, 1.0,
        )
    }

    pub fn create_rotation_x_center(radians: f32, center: Vector3) -> Matrix {
        let (cos, sin) = (radians.cos(), radians.sin());
        Matrix::new(
            1.0, 0.0, 0.0, 0.0,
            0.0, cos, sin, 0.0,
            0.0, -sin, cos, 0.0,
            0.0,
            sin * center.z - cos * center.y + center.y,
            -cos * center.z - sin * center.y + center.z,
            1.0,
        )
    }

    pub fn create_rotation_y(radians: f32) -> Matrix {
        let (cos, sin) = (radians.cos(), radians.sin());
        Matrix::new(
            cos, 0.0, -sin, 0.0,
            0.0, 1.0, 0.0, 0.0,
            sin, 0.0, cos, 0.0,
            0.0, 0.0, 0.0, 1.0,
        )
    }

    pub fn create_rotation_y_center(radians: f32, center: Vector3) -> Matrix {
        let (cos, sin) = (radians.cos(), radians.sin());
        Matrix::new(
            cos, 0.0, -sin, 0.0,
            0.0, 1.0, 0.0, 0.0,
            sin, 0.0, cos, 0.0,
            -sin * center.z - cos * center.x + center.x,
            0.0,
            -cos * center.z + sin * center.x + center.z,
            1.0,
        )
    }

    pub fn create_rotation_z(radians: f32) -> Matrix {
        let (cos, sin) = (radians.cos(), radians.sin());
        Matrix::new(
            cos, sin, 0.0, 0.0,
            -sin, cos, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 1.0,
        )
    }

    pub fn create_rotation_z_center(radians: f32, center: Vector3) -> Matrix {
        let (cos, sin) = (radians.cos(), radians.sin());
        Matrix::new(
            cos, sin, 0.0, 0.0,
            -sin, cos, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            sin * center.y - cos * center.x + center.x,
            -cos * center.y - sin * center.x + center.y,
            0.0, 1.0,
        )
    }

    pub fn create_scale(scale: f32) -> Matrix {
        Matrix::new(
            scale, 0.0, 0.0, 0.0,
            0.0, scale, 0.0, 0.0,
            0.0, 0.0, scale, 0.0,
            0.0, 0.0, 0.0, 1.0,
        )
    }

    pub fn create_scale_xyz(x: f32, y: f32, z: f32) -> Matrix {
        Matrix::new(
            x, 0.0, 0.0, 0.0,
            0.0, y, 0.0, 0.0,
            0.0, 0.0, z, 0.0,
            0.0, 0.0, 0.0, 1.0,
        )
    }

    pub fn create_scale_vec(scale: Vector3) -> Matrix {
        Matrix::create_scale_xyz(scale.x, scale.y, scale.z)
    }

    pub fn create_translation(x: f32, y: f32, z: f32) -> Matrix {
        Matrix::new(
            1.0, 0.0, 0.0, 0.0,
            0.0, 1.0, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            x, y, z, 1.0,
        )
    }

    pub fn create_translation_vec(position: Vector3) -> Matrix {
        Matrix::create_translation(position.x, position.y, position.z)
    }

    pub fn create_world(position: Vector3, forward: Vector3, up: Vector3) -> Matrix {
        let forward = Vector3::normalize(forward);
        let right = Vector3::normalize(Vector3::cross(forward, Vector3::normalize(up)));
        let up = Vector3::normalize(Vector3::cross(right, forward));
        let mut m = Matrix::IDENTITY;
        m.set_right(right);
        m.set_up(up);
        m.set_forward(forward);
        m.set_translation(position);
        m
    }

    pub fn create_shadow(light_dir: super::Vector4, plane: &super::Plane) -> Matrix {
        let dot = plane.normal.x * light_dir.x + plane.normal.y * light_dir.y + plane.normal.z * light_dir.z;
        let nx = -plane.normal.x;
        let ny = -plane.normal.y;
        let nz = -plane.normal.z;
        let nd = -plane.d;
        Matrix::new(
            nx * light_dir.x + dot,
            nx * light_dir.y,
            nx * light_dir.z,
            nx * light_dir.w,
            ny * light_dir.x,
            ny * light_dir.y + dot,
            ny * light_dir.z,
            ny * light_dir.w,
            nz * light_dir.x,
            nz * light_dir.y,
            nz * light_dir.z + dot,
            nz * light_dir.w,
            nd * light_dir.x,
            nd * light_dir.y,
            nd * light_dir.z,
            nd * light_dir.w + dot,
        )
    }

    pub fn transpose(m: &Matrix) -> Matrix {
        Matrix::new(
            m.m11, m.m21, m.m31, m.m41,
            m.m12, m.m22, m.m32, m.m42,
            m.m13, m.m23, m.m33, m.m43,
            m.m14, m.m24, m.m34, m.m44,
        )
    }

    pub fn invert(m: &Matrix) -> Matrix {
        let (m2, m3, m4, m5) = (m.m11, m.m12, m.m13, m.m14);
        let (m6, m7, m8, m9) = (m.m21, m.m22, m.m23, m.m24);
        let (m10, m11, m12, m13) = (m.m31, m.m32, m.m33, m.m34);
        let (m14, m15, m16, m17) = (m.m41, m.m42, m.m43, m.m44);

        let n = m12 * m17 - m13 * m16;
        let n2 = m11 * m17 - m13 * m15;
        let n3 = m11 * m16 - m12 * m15;
        let n4 = m10 * m17 - m13 * m14;
        let n5 = m10 * m16 - m12 * m14;
        let n6 = m10 * m15 - m11 * m14;
        let n7 = m7 * n - m8 * n2 + m9 * n3;
        let n8 = -(m6 * n - m8 * n4 + m9 * n5);
        let n9 = m6 * n2 - m7 * n4 + m9 * n6;
        let n10 = -(m6 * n3 - m7 * n5 + m8 * n6);
        let n11 = 1.0 / (m2 * n7 + m3 * n8 + m4 * n9 + m5 * n10);

        let mut result = Matrix::ZERO;
        result.m11 = n7 * n11;
        result.m21 = n8 * n11;
        result.m31 = n9 * n11;
        result.m41 = n10 * n11;
        result.m12 = -(m3 * n - m4 * n2 + m5 * n3) * n11;
        result.m22 = (m2 * n - m4 * n4 + m5 * n5) * n11;
        result.m32 = -(m2 * n2 - m3 * n4 + m5 * n6) * n11;
        result.m42 = (m2 * n3 - m3 * n5 + m4 * n6) * n11;

        let n12 = m8 * m17 - m9 * m16;
        let n13 = m7 * m17 - m9 * m15;
        let n14 = m7 * m16 - m8 * m15;
        let n15 = m6 * m17 - m9 * m14;
        let n16 = m6 * m16 - m8 * m14;
        let n17 = m6 * m15 - m7 * m14;
        result.m13 = (m3 * n12 - m4 * n13 + m5 * n14) * n11;
        result.m23 = -(m2 * n12 - m4 * n15 + m5 * n16) * n11;
        result.m33 = (m2 * n13 - m3 * n15 + m5 * n17) * n11;
        result.m43 = -(m2 * n14 - m3 * n16 + m4 * n17) * n11;

        let n18 = m8 * m13 - m9 * m12;
        let n19 = m7 * m13 - m9 * m11;
        let n20 = m7 * m12 - m8 * m11;
        let n21 = m6 * m13 - m9 * m10;
        let n22 = m6 * m12 - m8 * m10;
        let n23 = m6 * m11 - m7 * m10;
        result.m14 = -(m3 * n18 - m4 * n19 + m5 * n20) * n11;
        result.m24 = (m2 * n18 - m4 * n21 + m5 * n22) * n11;
        result.m34 = -(m2 * n19 - m3 * n21 + m5 * n23) * n11;
        result.m44 = (m2 * n20 - m3 * n22 + m4 * n23) * n11;
        result
    }

    pub fn lerp(m1: &Matrix, m2: &Matrix, f: f32) -> Matrix {
        let lerp_f = |a: f32, b: f32| a + (b - a) * f;
        Matrix::new(
            lerp_f(m1.m11, m2.m11), lerp_f(m1.m12, m2.m12),
            lerp_f(m1.m13, m2.m13), lerp_f(m1.m14, m2.m14),
            lerp_f(m1.m21, m2.m21), lerp_f(m1.m22, m2.m22),
            lerp_f(m1.m23, m2.m23), lerp_f(m1.m24, m2.m24),
            lerp_f(m1.m31, m2.m31), lerp_f(m1.m32, m2.m32),
            lerp_f(m1.m33, m2.m33), lerp_f(m1.m34, m2.m34),
            lerp_f(m1.m41, m2.m41), lerp_f(m1.m42, m2.m42),
            lerp_f(m1.m43, m2.m43), lerp_f(m1.m44, m2.m44),
        )
    }

    pub fn to_yaw_pitch_roll(&self) -> Vector3 {
        let (_, rotation, _) = self.decompose().unwrap_or((Vector3::ZERO, Quaternion::IDENTITY, Vector3::ZERO));
        rotation.to_yaw_pitch_roll()
    }
}

impl std::ops::Mul for Matrix {
    type Output = Matrix;
    fn mul(self, rhs: Matrix) -> Matrix {
        let a = &self;
        let b = &rhs;
        Matrix::new(
            a.m11 * b.m11 + a.m12 * b.m21 + a.m13 * b.m31 + a.m14 * b.m41,
            a.m11 * b.m12 + a.m12 * b.m22 + a.m13 * b.m32 + a.m14 * b.m42,
            a.m11 * b.m13 + a.m12 * b.m23 + a.m13 * b.m33 + a.m14 * b.m43,
            a.m11 * b.m14 + a.m12 * b.m24 + a.m13 * b.m34 + a.m14 * b.m44,
            a.m21 * b.m11 + a.m22 * b.m21 + a.m23 * b.m31 + a.m24 * b.m41,
            a.m21 * b.m12 + a.m22 * b.m22 + a.m23 * b.m32 + a.m24 * b.m42,
            a.m21 * b.m13 + a.m22 * b.m23 + a.m23 * b.m33 + a.m24 * b.m43,
            a.m21 * b.m14 + a.m22 * b.m24 + a.m23 * b.m34 + a.m24 * b.m44,
            a.m31 * b.m11 + a.m32 * b.m21 + a.m33 * b.m31 + a.m34 * b.m41,
            a.m31 * b.m12 + a.m32 * b.m22 + a.m33 * b.m32 + a.m34 * b.m42,
            a.m31 * b.m13 + a.m32 * b.m23 + a.m33 * b.m33 + a.m34 * b.m43,
            a.m31 * b.m14 + a.m32 * b.m24 + a.m33 * b.m34 + a.m34 * b.m44,
            a.m41 * b.m11 + a.m42 * b.m21 + a.m43 * b.m31 + a.m44 * b.m41,
            a.m41 * b.m12 + a.m42 * b.m22 + a.m43 * b.m32 + a.m44 * b.m42,
            a.m41 * b.m13 + a.m42 * b.m23 + a.m43 * b.m33 + a.m44 * b.m43,
            a.m41 * b.m14 + a.m42 * b.m24 + a.m43 * b.m34 + a.m44 * b.m44,
        )
    }
}

impl std::ops::Mul<f32> for Matrix {
    type Output = Matrix;
    fn mul(self, s: f32) -> Matrix {
        Matrix::new(
            self.m11 * s, self.m12 * s, self.m13 * s, self.m14 * s,
            self.m21 * s, self.m22 * s, self.m23 * s, self.m24 * s,
            self.m31 * s, self.m32 * s, self.m33 * s, self.m34 * s,
            self.m41 * s, self.m42 * s, self.m43 * s, self.m44 * s,
        )
    }
}

impl std::ops::Mul<Matrix> for f32 {
    type Output = Matrix;
    fn mul(self, m: Matrix) -> Matrix {
        m * self
    }
}

impl std::ops::Add for Matrix {
    type Output = Matrix;
    fn add(self, rhs: Matrix) -> Matrix {
        Matrix::new(
            self.m11 + rhs.m11, self.m12 + rhs.m12, self.m13 + rhs.m13, self.m14 + rhs.m14,
            self.m21 + rhs.m21, self.m22 + rhs.m22, self.m23 + rhs.m23, self.m24 + rhs.m24,
            self.m31 + rhs.m31, self.m32 + rhs.m32, self.m33 + rhs.m33, self.m34 + rhs.m34,
            self.m41 + rhs.m41, self.m42 + rhs.m42, self.m43 + rhs.m43, self.m44 + rhs.m44,
        )
    }
}

impl std::ops::Sub for Matrix {
    type Output = Matrix;
    fn sub(self, rhs: Matrix) -> Matrix {
        Matrix::new(
            self.m11 - rhs.m11, self.m12 - rhs.m12, self.m13 - rhs.m13, self.m14 - rhs.m14,
            self.m21 - rhs.m21, self.m22 - rhs.m22, self.m23 - rhs.m23, self.m24 - rhs.m24,
            self.m31 - rhs.m31, self.m32 - rhs.m32, self.m33 - rhs.m33, self.m34 - rhs.m34,
            self.m41 - rhs.m41, self.m42 - rhs.m42, self.m43 - rhs.m43, self.m44 - rhs.m44,
        )
    }
}

impl std::ops::Neg for Matrix {
    type Output = Matrix;
    fn neg(self) -> Matrix {
        Matrix::new(
            -self.m11, -self.m12, -self.m13, -self.m14,
            -self.m21, -self.m22, -self.m23, -self.m24,
            -self.m31, -self.m32, -self.m33, -self.m34,
            -self.m41, -self.m42, -self.m43, -self.m44,
        )
    }
}

impl std::fmt::Display for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{},{},{}, {},{},{},{}, {},{},{},{}, {},{},{},{}",
            self.m11, self.m12, self.m13, self.m14,
            self.m21, self.m22, self.m23, self.m24,
            self.m31, self.m32, self.m33, self.m34,
            self.m41, self.m42, self.m43, self.m44)
    }
}
