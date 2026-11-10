use std::sync::atomic::{AtomicBool, AtomicI32, AtomicU32, Ordering};
use std::sync::Mutex;
use super::sound_state::SoundState;
use crate::math::vector3::Vector3;

pub struct BaseSound {
    volume: AtomicU32,  // f32 bits
    pitch: AtomicU32,   // f32 bits
    pan: AtomicU32,     // f32 bits
    pub(crate) state: AtomicI32,
    pub(crate) is_looped: AtomicBool,
    pub(crate) dispose_on_stop: AtomicBool,
    pub(crate) source: AtomicI32,
    pub channels_count: AtomicI32,
    pub sampling_frequency: AtomicI32,
    pub(crate) lock: Mutex<()>,
}

impl BaseSound {
    pub fn new() -> Self {
        BaseSound {
            volume: AtomicU32::new(1.0f32.to_bits()),
            pitch: AtomicU32::new(1.0f32.to_bits()),
            pan: AtomicU32::new(0.0f32.to_bits()),
            state: AtomicI32::new(SoundState::Stopped as i32),
            is_looped: AtomicBool::new(false),
            dispose_on_stop: AtomicBool::new(false),
            source: AtomicI32::new(0),
            channels_count: AtomicI32::new(0),
            sampling_frequency: AtomicI32::new(0),
            lock: Mutex::new(()),
        }
    }

    pub fn state(&self) -> SoundState {
        match self.state.load(Ordering::Relaxed) {
            0 => SoundState::Stopped,
            1 => SoundState::Playing,
            2 => SoundState::Paused,
            3 => SoundState::Disposed,
            _ => SoundState::Stopped,
        }
    }

    pub fn set_state(&self, state: SoundState) {
        self.state.store(state as i32, Ordering::Relaxed);
    }

    pub fn volume(&self) -> f32 {
        f32::from_bits(self.volume.load(Ordering::Relaxed))
    }

    pub fn set_volume(&self, value: f32) {
        let value = value.clamp(0.0, 1.0);
        let old = f32::from_bits(self.volume.load(Ordering::Relaxed));
        if value != old {
            self.volume.store(value.to_bits(), Ordering::Relaxed);
            // TODO: InternalSetVolume
        }
    }

    pub fn pitch(&self) -> f32 {
        f32::from_bits(self.pitch.load(Ordering::Relaxed))
    }

    pub fn set_pitch(&self, value: f32) {
        let value = value.clamp(0.5, 2.0);
        let old = f32::from_bits(self.pitch.load(Ordering::Relaxed));
        if value != old {
            self.pitch.store(value.to_bits(), Ordering::Relaxed);
            // TODO: InternalSetPitch
        }
    }

    pub fn pan(&self) -> f32 {
        f32::from_bits(self.pan.load(Ordering::Relaxed))
    }

    pub fn set_pan(&self, value: f32) {
        if self.channels_count.load(Ordering::Relaxed) == 1 {
            let value = value.clamp(-1.0, 1.0);
            let old = f32::from_bits(self.pan.load(Ordering::Relaxed));
            if value != old {
                self.pan.store(value.to_bits(), Ordering::Relaxed);
                // TODO: InternalSetPan
            }
        }
    }

    pub fn is_looped(&self) -> bool {
        self.is_looped.load(Ordering::Relaxed)
    }

    pub fn set_is_looped(&self, value: bool) {
        let _guard = self.lock.lock().unwrap();
        if self.state() == SoundState::Stopped {
            self.is_looped.store(value, Ordering::Relaxed);
        }
    }

    pub fn dispose_on_stop(&self) -> bool {
        self.dispose_on_stop.load(Ordering::Relaxed)
    }

    pub fn set_dispose_on_stop(&self, value: bool) {
        let _guard = self.lock.lock().unwrap();
        if self.state() == SoundState::Stopped {
            self.dispose_on_stop.store(value, Ordering::Relaxed);
        }
    }

    pub fn play(&self) {
        let _guard = self.lock.lock().unwrap();
        let state = self.state();
        if state == SoundState::Stopped || state == SoundState::Paused {
            self.set_state(SoundState::Playing);
            self.internal_play(Vector3::ZERO);
        }
    }

    pub fn play_at(&self, direction: Vector3) {
        let _guard = self.lock.lock().unwrap();
        let state = self.state();
        if state == SoundState::Stopped || state == SoundState::Paused {
            self.set_state(SoundState::Playing);
            self.internal_play(direction);
        }
    }

    pub fn pause(&self) {
        let _guard = self.lock.lock().unwrap();
        if self.state() == SoundState::Playing {
            self.set_state(SoundState::Paused);
            self.internal_pause();
        }
    }

    pub fn stop(&self) {
        if self.dispose_on_stop.load(Ordering::Relaxed) {
            self.dispose();
        }
        let _guard = self.lock.lock().unwrap();
        let state = self.state();
        if state == SoundState::Playing || state == SoundState::Paused {
            self.set_state(SoundState::Stopped);
            self.internal_stop();
        }
    }

    pub fn dispose(&self) {
        if self.state() != SoundState::Disposed {
            self.set_state(SoundState::Disposed);
            self.internal_dispose();
        }
    }

    fn internal_play(&self, _direction: Vector3) {
        // TODO: OpenAL SourcePlay
    }

    fn internal_pause(&self) {
        // TODO: OpenAL SourcePause
    }

    fn internal_stop(&self) {
        // TODO: OpenAL SourceRewind
    }

    fn internal_dispose(&self) {
        let source = self.source.swap(0, Ordering::Relaxed);
        if source != 0 {
            // TODO: OpenAL SourceStop + DeleteSource
        }
    }
}

impl Default for BaseSound {
    fn default() -> Self {
        Self::new()
    }
}
