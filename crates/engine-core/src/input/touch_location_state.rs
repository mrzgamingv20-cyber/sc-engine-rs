#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(i32)]
pub enum TouchLocationState {
    #[default]
    Pressed = 0,
    Moved = 1,
    Released = 2,
}
