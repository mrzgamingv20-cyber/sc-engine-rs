use super::vector2::Vector2;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct BoundingCircle {
    pub center: Vector2,
    pub radius: f32,
}

impl BoundingCircle {
    #[inline]
    pub fn new(center: Vector2, radius: f32) -> Self {
        BoundingCircle { center, radius }
    }

    #[inline]
    pub fn contains(&self, p: Vector2) -> bool {
        Vector2::distance_squared(self.center, p) <= self.radius * self.radius
    }
}

impl std::fmt::Display for BoundingCircle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{}", self.center, self.radius)
    }
}
