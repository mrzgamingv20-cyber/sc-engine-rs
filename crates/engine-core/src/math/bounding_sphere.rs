use super::vector3::Vector3;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct BoundingSphere {
    pub center: Vector3,
    pub radius: f32,
}

impl BoundingSphere {
    #[inline]
    pub fn new(center: Vector3, radius: f32) -> Self {
        BoundingSphere { center, radius }
    }
}

impl std::fmt::Display for BoundingSphere {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{}", self.center, self.radius)
    }
}
