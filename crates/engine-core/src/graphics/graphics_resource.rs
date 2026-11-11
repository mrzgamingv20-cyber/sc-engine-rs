//! Base class for GPU-side resources (textures, buffers, shaders).

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;

/// Process-wide registry of live graphics resources.
static RESOURCES: Mutex<Vec<usize>> = Mutex::new(Vec::new());

/// A resource that owns GPU memory and must be explicitly disposed.
pub trait GraphicsResource: Send + Sync {
    /// Approximate GPU memory usage in bytes.
    fn gpu_memory_usage(&self) -> i64;

    /// Called when the graphics device is lost; implementations must drop GPU handles.
    fn handle_device_lost(&self);

    /// Called when the graphics device is restored; implementations must recreate GPU handles.
    fn handle_device_reset(&self);
}

/// Tracks disposal state for a graphics resource.
#[derive(Debug)]
pub struct ResourceState {
    is_disposed: AtomicBool,
}

impl ResourceState {
    pub fn new() -> Self {
        ResourceState { is_disposed: AtomicBool::new(false) }
    }

    #[inline]
    pub fn is_disposed(&self) -> bool {
        self.is_disposed.load(Ordering::Relaxed)
    }

    #[inline]
    pub fn dispose(&self) {
        self.is_disposed.store(true, Ordering::Relaxed);
    }

    /// Returns `Err` when the resource has already been disposed.
    #[inline]
    pub fn verify_not_disposed(&self) -> Result<(), &'static str> {
        if self.is_disposed() {
            Err("GraphicsResource is disposed.")
        } else {
            Ok(())
        }
    }
}

impl Default for ResourceState {
    fn default() -> Self {
        Self::new()
    }
}

pub fn register_resource(id: usize) {
    RESOURCES.lock().unwrap().push(id);
}

pub fn unregister_resource(id: usize) {
    RESOURCES.lock().unwrap().retain(|x| *x != id);
}

pub fn resource_count() -> usize {
    RESOURCES.lock().unwrap().len()
}