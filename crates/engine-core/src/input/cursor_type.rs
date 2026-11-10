#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum CursorType {
    Default = 0,
    Arrow = 1,
    IBeam = 2,
    Crosshair = 3,
    Hand = 4,
    HResize = 5,
    VResize = 6,
    NwseResize = 7,
    NeswResize = 8,
    ResizeAll = 9,
    NotAllowed = 10,
    Wait = 11,
    WaitArrow = 12,
    Grab = 13,
    Grabbing = 14,
}
