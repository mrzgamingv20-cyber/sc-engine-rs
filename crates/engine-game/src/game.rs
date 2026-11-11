//! Game root: owns subsystems and advances the simulation.

use crate::subsystem::Subsystem;

/// Game instance.
pub struct Game {
    subsystems: Vec<Box<dyn Subsystem>>,
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}

impl Game {
    pub fn new() -> Self {
        Self {
            subsystems: Vec::new(),
        }
    }

    /// Register a subsystem. Update order follows registration order.
    pub fn add_subsystem(&mut self, subsystem: Box<dyn Subsystem>) {
        self.subsystems.push(subsystem);
    }

    /// Advance the simulation by `dt` seconds.
    pub fn update(&mut self, dt: f32) {
        for subsystem in &mut self.subsystems {
            subsystem.update(dt);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};

    struct CountingSubsystem {
        count: Arc<AtomicUsize>,
    }

    impl Subsystem for CountingSubsystem {
        fn update(&mut self, _dt: f32) {
            self.count.fetch_add(1, Ordering::SeqCst);
        }
    }

    #[test]
    fn update_propagates_to_subsystems() {
        let count = Arc::new(AtomicUsize::new(0));
        let mut game = Game::new();
        game.add_subsystem(Box::new(CountingSubsystem {
            count: Arc::clone(&count),
        }));

        game.update(0.016);
        game.update(0.016);

        assert_eq!(count.load(Ordering::SeqCst), 2);
    }
}
