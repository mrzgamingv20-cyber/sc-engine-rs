use crate::math::point2::Point2;
use super::mouse_button::MouseButton;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct MouseButtonEvent {
    pub position: Point2,
    pub button: MouseButton,
}
