#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct TouchLocation {
    pub id: i32,
    pub position: crate::math::vector2::Vector2,
    pub state: super::touch_location_state::TouchLocationState,
    pub release_queued: bool,
}
