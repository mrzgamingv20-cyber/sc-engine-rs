#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OpenFileMode {
    Read = 0,
    ReadWrite = 1,
    Create = 2,
    CreateOrOpen = 3,
}
