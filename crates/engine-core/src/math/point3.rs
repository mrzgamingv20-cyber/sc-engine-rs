#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Point3 {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Point3 {
    pub const ZERO: Point3 = Point3 { x: 0, y: 0, z: 0 };
    pub const ONE: Point3 = Point3 { x: 1, y: 1, z: 1 };
    pub const UNIT_X: Point3 = Point3 { x: 1, y: 0, z: 0 };
    pub const UNIT_Y: Point3 = Point3 { x: 0, y: 1, z: 0 };
    pub const UNIT_Z: Point3 = Point3 { x: 0, y: 0, z: 1 };

    #[inline]
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Point3 { x, y, z }
    }

    #[inline]
    pub fn splat(v: i32) -> Self {
        Point3 { x: v, y: v, z: v }
    }

    #[inline]
    pub fn dot(p1: Point3, p2: Point3) -> i32 {
        p1.x * p2.x + p1.y * p2.y + p1.z * p2.z
    }

    #[inline]
    pub fn cross(p1: Point3, p2: Point3) -> Point3 {
        Point3::new(
            p1.y * p2.z - p1.z * p2.y,
            p1.z * p2.x - p1.x * p2.z,
            p1.x * p2.y - p1.y * p2.x,
        )
    }

    #[inline]
    pub fn min(p: Point3, v: i32) -> Point3 {
        Point3::new(p.x.min(v), p.y.min(v), p.z.min(v))
    }

    #[inline]
    pub fn min_vec(p1: Point3, p2: Point3) -> Point3 {
        Point3::new(p1.x.min(p2.x), p1.y.min(p2.y), p1.z.min(p2.z))
    }

    #[inline]
    pub fn max(p: Point3, v: i32) -> Point3 {
        Point3::new(p.x.max(v), p.y.max(v), p.z.max(v))
    }

    #[inline]
    pub fn max_vec(p1: Point3, p2: Point3) -> Point3 {
        Point3::new(p1.x.max(p2.x), p1.y.max(p2.y), p1.z.max(p2.z))
    }

    #[inline]
    pub fn min_element(p: Point3) -> i32 {
        p.x.min(p.y).min(p.z)
    }

    #[inline]
    pub fn max_element(p: Point3) -> i32 {
        p.x.max(p.y).max(p.z)
    }

    #[inline]
    pub fn to_vector3(&self) -> super::Vector3 {
        super::Vector3::new(self.x as f32, self.y as f32, self.z as f32)
    }

    pub fn round(v: super::Vector3) -> Point3 {
        Point3::new(v.x.round() as i32, v.y.round() as i32, v.z.round() as i32)
    }

    pub fn ceiling(v: super::Vector3) -> Point3 {
        Point3::new(v.x.ceil() as i32, v.y.ceil() as i32, v.z.ceil() as i32)
    }

    pub fn floor(v: super::Vector3) -> Point3 {
        Point3::new(v.x.floor() as i32, v.y.floor() as i32, v.z.floor() as i32)
    }
}

impl From<super::Vector3> for Point3 {
    fn from(v: super::Vector3) -> Self {
        Point3::new(v.x as i32, v.y as i32, v.z as i32)
    }
}

impl std::ops::Add for Point3 {
    type Output = Point3;
    fn add(self, rhs: Point3) -> Point3 {
        Point3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl std::ops::Sub for Point3 {
    type Output = Point3;
    fn sub(self, rhs: Point3) -> Point3 {
        Point3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl std::ops::Mul<i32> for Point3 {
    type Output = Point3;
    fn mul(self, n: i32) -> Point3 {
        Point3::new(self.x * n, self.y * n, self.z * n)
    }
}

impl std::ops::Mul<Point3> for i32 {
    type Output = Point3;
    fn mul(self, p: Point3) -> Point3 {
        Point3::new(p.x * self, p.y * self, p.z * self)
    }
}

impl std::ops::Div<i32> for Point3 {
    type Output = Point3;
    fn div(self, n: i32) -> Point3 {
        Point3::new(self.x / n, self.y / n, self.z / n)
    }
}

impl std::ops::Neg for Point3 {
    type Output = Point3;
    fn neg(self) -> Point3 {
        Point3::new(-self.x, -self.y, -self.z)
    }
}

impl std::fmt::Display for Point3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{},{}", self.x, self.y, self.z)
    }
}
