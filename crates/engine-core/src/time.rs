use std::time::{Instant, Duration};

static mut START_TIME: Option<Instant> = None;

pub fn init() {
    unsafe {
        if START_TIME.is_none() {
            START_TIME = Some(Instant::now());
        }
    }
}

pub fn total_time() -> Duration {
    unsafe {
        START_TIME.map(|t| t.elapsed()).unwrap_or_default()
    }
}

pub fn total_seconds() -> f64 {
    total_time().as_secs_f64()
}

pub fn total_milliseconds() -> f64 {
    total_time().as_secs_f64() * 1000.0
}
