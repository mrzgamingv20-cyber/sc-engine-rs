#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(i32)]
pub enum MouseButton {
    #[default]
    Left = 0,
    Right = 1,
    Middle = 2,
    Ext1 = 3,
    Ext2 = 4,
}

impl MouseButton {
    pub const COUNT: usize = 5;
}
