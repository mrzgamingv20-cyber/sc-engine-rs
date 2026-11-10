#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum TouchLocationState {
    Pressed = 0,
    Moved = 1,
    Released = 2,
}
