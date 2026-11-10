#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Point2 {
    pub x: i32,
    pub y: i32,
}

impl Point2 {
    pub const ZERO: Point2 = Point2 { x: 0, y: 0 };
    pub const ONE: Point2 = Point2 { x: 1, y: 1 };
    pub const UNIT_X: Point2 = Point2 { x: 1, y: 0 };
    pub const UNIT_Y: Point2 = Point2 { x: 0, y: 1 };

    #[inline]
    pub fn new(x: i32, y: i32) -> Self {
        Point2 { x, y }
    }

    #[inline]
    pub fn splat(v: i32) -> Self {
        Point2 { x: v, y: v }
    }

    #[inline]
    pub fn dot(p1: Point2, p2: Point2) -> i32 {
        p1.x * p2.x + p1.y * p2.y
    }

    #[inline]
    pub fn cross(p1: Point2, p2: Point2) -> i32 {
        p1.x * p2.y - p1.y * p2.x
    }

    #[inline]
    pub fn perpendicular(p: Point2) -> Point2 {
        Point2::new(-p.y, p.x)
    }

    #[inline]
    pub fn min(p: Point2, v: i32) -> Point2 {
        Point2::new(p.x.min(v), p.y.min(v))
    }

    #[inline]
    pub fn min_vec(p1: Point2, p2: Point2) -> Point2 {
        Point2::new(p1.x.min(p2.x), p1.y.min(p2.y))
    }

    #[inline]
    pub fn max(p: Point2, v: i32) -> Point2 {
        Point2::new(p.x.max(v), p.y.max(v))
    }

    #[inline]
    pub fn max_vec(p1: Point2, p2: Point2) -> Point2 {
        Point2::new(p1.x.max(p2.x), p1.y.max(p2.y))
    }

    #[inline]
    pub fn min_element(p: Point2) -> i32 {
        p.x.min(p.y)
    }

    #[inline]
    pub fn max_element(p: Point2) -> i32 {
        p.x.max(p.y)
    }

    #[inline]
    pub fn to_vector2(&self) -> super::Vector2 {
        super::Vector2::new(self.x as f32, self.y as f32)
    }

    pub fn round(v: super::Vector2) -> Point2 {
        Point2::new(v.x.round() as i32, v.y.round() as i32)
    }

    pub fn ceiling(v: super::Vector2) -> Point2 {
        Point2::new(v.x.ceil() as i32, v.y.ceil() as i32)
    }

    pub fn floor(v: super::Vector2) -> Point2 {
        Point2::new(v.x.floor() as i32, v.y.floor() as i32)
    }
}

impl From<super::Vector2> for Point2 {
    fn from(v: super::Vector2) -> Self {
        Point2::new(v.x as i32, v.y as i32)
    }
}

impl std::ops::Add for Point2 {
    type Output = Point2;
    fn add(self, rhs: Point2) -> Point2 {
        Point2::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl std::ops::Sub for Point2 {
    type Output = Point2;
    fn sub(self, rhs: Point2) -> Point2 {
        Point2::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl std::ops::Mul<i32> for Point2 {
    type Output = Point2;
    fn mul(self, n: i32) -> Point2 {
        Point2::new(self.x * n, self.y * n)
    }
}

impl std::ops::Mul<Point2> for i32 {
    type Output = Point2;
    fn mul(self, p: Point2) -> Point2 {
        Point2::new(p.x * self, p.y * self)
    }
}

impl std::ops::Div<i32> for Point2 {
    type Output = Point2;
    fn div(self, n: i32) -> Point2 {
        Point2::new(self.x / n, self.y / n)
    }
}

impl std::ops::Neg for Point2 {
    type Output = Point2;
    fn neg(self) -> Point2 {
        Point2::new(-self.x, -self.y)
    }
}

impl std::fmt::Display for Point2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{}", self.x, self.y)
    }
}
