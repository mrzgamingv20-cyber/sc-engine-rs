use super::vector2::Vector2;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Line2 {
    pub normal: Vector2,
    pub d: f32,
}

impl Line2 {
    #[inline]
    pub fn new(normal: Vector2, d: f32) -> Self {
        Line2 { normal, d }
    }

    #[inline]
    pub fn from_points(p1: Vector2, p2: Vector2) -> Self {
        let normal = Vector2::normalize(Vector2::new(p1.y - p2.y, p2.x - p1.x));
        let d = -Vector2::dot(normal, p1);
        Line2 { normal, d }
    }

    #[inline]
    pub fn from_xy_d(x: f32, y: f32, d: f32) -> Self {
        Line2::new(Vector2::new(x, y), d)
    }

    #[inline]
    pub fn signed_distance(&self, p: Vector2) -> f32 {
        Vector2::dot(self.normal, p) + self.d
    }

    pub fn normalize(l: &Line2) -> Line2 {
        let num = l.normal.length();
        if num > 0.0 {
            let num2 = 1.0 / num;
            Line2::new(l.normal * num2, l.d * num2)
        } else {
            Line2::new(Vector2::UNIT_X, 0.0)
        }
    }

    pub fn intersection(l1: &Line2, l2: &Line2) -> Option<Vector2> {
        let num = -Vector2::cross(l1.normal, l2.normal);
        if num == 0.0 {
            return None;
        }
        let num2 = 1.0 / num;
        Some(Vector2::new(
            (l2.normal.y * l1.d - l1.normal.y * l2.d) * num2,
            (l1.normal.x * l2.d - l2.normal.x * l1.d) * num2,
        ))
    }
}

impl std::fmt::Display for Line2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{},{}", self.normal.x, self.normal.y, self.d)
    }
}
