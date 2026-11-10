use std::sync::atomic::{AtomicI32, AtomicUsize, Ordering};
use super::sound_state::SoundState;

pub struct SoundBuffer {
    pub(crate) buffer: AtomicI32,
    pub channels_count: AtomicI32,
    pub sampling_frequency: AtomicI32,
    pub samples_count: AtomicI32,
    pub use_count: AtomicUsize,
}

impl SoundBuffer {
    pub fn new() -> Self {
        SoundBuffer {
            buffer: AtomicI32::new(0),
            channels_count: AtomicI32::new(0),
            sampling_frequency: AtomicI32::new(0),
            samples_count: AtomicI32::new(0),
            use_count: AtomicUsize::new(0),
        }
    }

    pub fn from_pcm(data: &[u8], channels_count: i32, sampling_frequency: i32) -> Self {
        let buf = SoundBuffer::new();
        buf.initialize_properties(
            data.len() as i32 / channels_count / 2,
            channels_count,
            sampling_frequency,
        );
        buf
    }

    pub fn from_pcm_i16(data: &[i16], channels_count: i32, sampling_frequency: i32) -> Self {
        let buf = SoundBuffer::new();
        buf.initialize_properties(
            data.len() as i32 / channels_count,
            channels_count,
            sampling_frequency,
        );
        buf
    }

    pub fn initialize_properties(&self, samples_count: i32, channels_count: i32, sampling_frequency: i32) {
        assert!(samples_count > 0, "Buffer cannot have zero samples.");
        assert!((1..=2).contains(&channels_count), "channelsCount out of range");
        assert!((8000..=192000).contains(&sampling_frequency), "samplingFrequency out of range");
        self.channels_count.store(channels_count, Ordering::Relaxed);
        self.sampling_frequency.store(sampling_frequency, Ordering::Relaxed);
        self.samples_count.store(samples_count, Ordering::Relaxed);
    }

    pub fn increment_use_count(&self) {
        self.use_count.fetch_add(1, Ordering::Relaxed);
    }

    pub fn decrement_use_count(&self) {
        self.use_count.fetch_sub(1, Ordering::Relaxed);
    }

    pub fn use_count(&self) -> usize {
        self.use_count.load(Ordering::Relaxed)
    }

    pub fn dispose(&self) {
        assert_eq!(self.use_count(), 0, "Cannot dispose SoundBuffer in use");
        let buf = self.buffer.swap(0, Ordering::Relaxed);
        if buf != 0 {
            // TODO: delete OpenAL buffer
        }
    }
}

impl Default for SoundBuffer {
    fn default() -> Self {
        Self::new()
    }
}
