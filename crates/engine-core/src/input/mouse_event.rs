use crate::math::point2::Point2;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct MouseEvent {
    pub position: Point2,
}
