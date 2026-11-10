#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum SoundState {
    Stopped = 0,
    Playing = 1,
    Paused = 2,
    Disposed = 3,
}
