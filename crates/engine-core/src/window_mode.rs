#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WindowMode {
    Resizable = 0,
    Fixed = 1,
    Borderless = 2,
    Fullscreen = 3,
}
