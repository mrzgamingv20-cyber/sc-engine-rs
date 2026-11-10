use super::math_utils::{min_f32, max_f32};
use super::vector2::Vector2;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct BoundingRectangle {
    pub min: Vector2,
    pub max: Vector2,
}

impl BoundingRectangle {
    #[inline]
    pub fn new(x1: f32, y1: f32, x2: f32, y2: f32) -> Self {
        BoundingRectangle {
            min: Vector2::new(x1, y1),
            max: Vector2::new(x2, y2),
        }
    }

    #[inline]
    pub fn from_min_max(min: Vector2, max: Vector2) -> Self {
        BoundingRectangle { min, max }
    }

    pub fn from_points(points: &[Vector2]) -> Self {
        assert!(!points.is_empty(), "points");
        let mut min = Vector2::new(f32::INFINITY, f32::INFINITY);
        let mut max = Vector2::new(f32::NEG_INFINITY, f32::NEG_INFINITY);
        for p in points {
            min.x = min_f32(min.x, p.x);
            min.y = min_f32(min.y, p.y);
            max.x = max_f32(max.x, p.x);
            max.y = max_f32(max.y, p.y);
        }
        BoundingRectangle { min, max }
    }

    #[inline]
    pub fn corners(&self) -> [Vector2; 4] {
        [
            Vector2::new(self.min.x, self.min.y),
            Vector2::new(self.max.x, self.min.y),
            Vector2::new(self.min.x, self.max.y),
            Vector2::new(self.max.x, self.max.y),
        ]
    }

    #[inline]
    pub fn contains(&self, p: Vector2) -> bool {
        p.x >= self.min.x && p.x <= self.max.x
            && p.y >= self.min.y && p.y <= self.max.y
    }

    #[inline]
    pub fn intersects(&self, r: &BoundingRectangle) -> bool {
        self.min.x <= r.max.x && self.max.x >= r.min.x
            && self.min.y <= r.max.y && self.max.y >= r.min.y
    }
}

impl std::fmt::Display for BoundingRectangle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{},{},{}", self.min.x, self.min.y, self.max.x, self.max.y)
    }
}
