//! Game subsystems.

/// A game subsystem updated once per tick.
pub trait Subsystem: Send {
    /// Advance the simulation by `dt` seconds.
    fn update(&mut self, dt: f32);
}
