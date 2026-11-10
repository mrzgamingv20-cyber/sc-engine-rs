use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::Mutex;
use crate::saturate;

pub struct Mixer;

static MASTER_VOLUME: AtomicU32 = AtomicU32::new(1.0f32.to_bits());
static IS_INITIALIZED: AtomicBool = AtomicBool::new(false);
static SOUNDS_TO_STOP: Mutex<Vec<usize>> = Mutex::new(Vec::new());

impl Mixer {
    pub fn master_volume() -> f32 {
        f32::from_bits(MASTER_VOLUME.load(Ordering::Relaxed))
    }

    pub fn set_master_volume(value: f32) {
        let value = saturate(value);
        let old = f32::from_bits(MASTER_VOLUME.load(Ordering::Relaxed));
        if value != old {
            MASTER_VOLUME.store(value.to_bits(), Ordering::Relaxed);
            // TODO: OpenAL SetListenerProperty Gain
        }
    }

    pub fn is_initialized() -> bool {
        IS_INITIALIZED.load(Ordering::Relaxed)
    }

    pub fn initialize() {
        // TODO: OpenAL context initialization
        IS_INITIALIZED.store(true, Ordering::Relaxed);
    }

    pub fn dispose() {
        IS_INITIALIZED.store(false, Ordering::Relaxed);
        // TODO: OpenAL context disposal
    }

    pub fn before_frame() {
        let mut to_stop = SOUNDS_TO_STOP.lock().unwrap();
        // TODO: check OpenAL source state
        to_stop.clear();
    }
}
