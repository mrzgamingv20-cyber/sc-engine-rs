use std::sync::OnceLock;
use std::time::{Instant, Duration};

static START_TIME: OnceLock<Instant> = OnceLock::new();

pub fn init() {
    START_TIME.get_or_init(Instant::now);
}

pub fn total_time() -> Duration {
    START_TIME.get().map(|t| t.elapsed()).unwrap_or_default()
}

pub fn total_seconds() -> f64 {
    total_time().as_secs_f64()
}

pub fn total_milliseconds() -> f64 {
    total_time().as_secs_f64() * 1000.0
}
