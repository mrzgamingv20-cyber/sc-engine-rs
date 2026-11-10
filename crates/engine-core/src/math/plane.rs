use super::vector3::Vector3;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Plane {
    pub normal: Vector3,
    pub d: f32,
}

impl Plane {
    #[inline]
    pub fn new(normal: Vector3, d: f32) -> Self {
        Plane { normal, d }
    }

    #[inline]
    pub fn from_xyzd(x: f32, y: f32, z: f32, d: f32) -> Self {
        Plane::new(Vector3::new(x, y, z), d)
    }

    pub fn from_points(a: Vector3, b: Vector3, c: Vector3) -> Self {
        let normal = Vector3::normalize(Vector3::cross(b - a, c - a));
        let d = -Vector3::dot(normal, a);
        Plane::new(normal, d)
    }

    pub fn from_vector4(v: super::Vector4) -> Self {
        Plane::new(Vector3::new(v.x, v.y, v.z), v.w)
    }

    pub fn normalize(p: &Plane) -> Plane {
        let len = p.normal.length();
        if len > 0.0 {
            let inv = 1.0 / len;
            Plane::new(p.normal * inv, p.d * inv)
        } else {
            Plane::new(Vector3::UNIT_X, 0.0)
        }
    }
}

impl std::fmt::Display for Plane {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{},{},{}", self.normal.x, self.normal.y, self.normal.z, self.d)
    }
}
