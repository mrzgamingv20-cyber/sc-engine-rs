/// Base for immutable-once-attached state objects.
///
/// Mirrors the C# `LockOnFirstUse` behaviour: fields may be changed freely until the
/// object is handed to the graphics device, after which mutation is rejected.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct LockOnFirstUse {
    pub is_locked: bool,
}

impl LockOnFirstUse {
    pub const fn new() -> Self {
        LockOnFirstUse { is_locked: false }
    }

    pub const fn locked() -> Self {
        LockOnFirstUse { is_locked: true }
    }

    #[inline]
    pub fn is_locked(&self) -> bool {
        self.is_locked
    }

    #[inline]
    pub fn lock(&mut self) {
        self.is_locked = true;
    }

    /// Returns `Err` when the object has already been locked.
    #[inline]
    pub fn throw_if_locked(&self) -> Result<(), &'static str> {
        if self.is_locked {
            Err("Object was attached to a device and can no longer be modified.")
        } else {
            Ok(())
        }
    }
}