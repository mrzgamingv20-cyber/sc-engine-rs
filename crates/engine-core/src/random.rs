use std::sync::atomic::{AtomicU32, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

static COUNTER: AtomicU32 = AtomicU32::new(0);

pub struct Random {
    s0: u32,
    s1: u32,
}

impl Random {
    pub fn new() -> Self {
        let mut r = Random { s0: 0, s1: 0 };
        r.seed_auto();
        r
    }

    pub fn with_seed(seed: i32) -> Self {
        let mut r = Random { s0: 0, s1: 0 };
        r.seed(seed);
        r
    }

    pub fn seed_auto(&mut self) {
        let counter = COUNTER.fetch_add(1, Ordering::Relaxed);
        let ticks = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.subsec_nanos())
            .unwrap_or(0);
        let seed = (counter as i32).wrapping_add(ticks as i32);
        self.seed(seed);
    }

    pub fn seed(&mut self, seed: i32) {
        self.s0 = (seed as u32).wrapping_add(42817);
        self.s1 = (seed as u32).wrapping_add(324508639);
    }

    pub fn state(&self) -> u64 {
        self.s0 as u64 | ((self.s1 as u64) << 32)
    }

    pub fn set_state(&mut self, state: u64) {
        self.s0 = state as u32;
        self.s1 = (state >> 32) as u32;
    }

    pub fn sign(&mut self) -> i32 {
        ((self.uint() & 1) * 2 - 1) as i32
    }

    pub fn boolean(&mut self) -> bool {
        (self.uint() & 1) != 0
    }

    pub fn boolean_prob(&mut self, probability: f32) -> bool {
        self.int() as f32 / 2_147_483_648.0f32 < probability
    }

    pub fn uint(&mut self) -> u32 {
        let s = self.s0;
        let mut s2 = self.s1;
        s2 ^= s;
        self.s0 = s.rotate_left(26) ^ s2 ^ (s2 << 9);
        self.s1 = s2.rotate_left(13);
        (s.wrapping_mul(2654435771)).rotate_left(5).wrapping_mul(5)
    }

    pub fn int(&mut self) -> i32 {
        (self.uint() & 0x7FFF_FFFF) as i32
    }

    pub fn int_bound(&mut self, bound: i32) -> i32 {
        ((self.int() as i64 * bound as i64) / 2_147_483_648u64 as i64) as i32
    }

    pub fn int_range(&mut self, min: i32, max: i32) -> i32 {
        (min as i64 + (self.int() as i64 * (max - min + 1) as i64) / 2_147_483_648u64 as i64) as i32
    }

    pub fn float(&mut self) -> f32 {
        self.int() as f32 / 2_147_483_648.0f32
    }

    pub fn float_range(&mut self, min: f32, max: f32) -> f32 {
        min + self.float() * (max - min)
    }

    pub fn normal_float(&mut self, mean: f32, stddev: f32) -> f32 {
        let u1 = self.float().max(1e-7);
        let u2 = self.float();
        let z = (-2.0 * u1.ln()).sqrt() * (2.0 * std::f32::consts::PI * u2).cos();
        mean + z * stddev
    }
}

impl Default for Random {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for Random {
    fn clone(&self) -> Self {
        Random { s0: self.s0, s1: self.s1 }
    }
}
