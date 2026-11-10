use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;
use super::base_sound::BaseSound;
use crate::math::vector3::Vector3;

pub trait StreamingSource: Send {
    fn read(&mut self, buffer: &mut [u8]) -> usize;
    fn position(&self) -> u64;
    fn set_position(&mut self, pos: u64);
    fn bytes_count(&self) -> u64;
    fn channels_count(&self) -> i32;
    fn sampling_frequency(&self) -> i32;
}

pub struct StreamingSound {
    pub base: BaseSound,
    pub streaming_source: Mutex<Option<Box<dyn StreamingSource>>>,
    no_more_data: AtomicBool,
    pub buffer_duration: f32,
}

impl StreamingSound {
    pub fn new(
        streaming_source: Box<dyn StreamingSource>,
        volume: f32,
        pitch: f32,
        pan: f32,
        is_looped: bool,
        dispose_on_stop: bool,
        buffer_duration: f32,
    ) -> Self {
        let channels = streaming_source.channels_count();
        let freq = streaming_source.sampling_frequency();
        assert!((1..=2).contains(&channels), "Unsupported channels count.");
        assert!((8000..=192000).contains(&freq), "Unsupported frequency.");

        let sound = StreamingSound {
            base: BaseSound::new(),
            streaming_source: Mutex::new(Some(streaming_source)),
            no_more_data: AtomicBool::new(false),
            buffer_duration: buffer_duration.clamp(0.01, 10.0),
        };
        sound.base.channels_count.store(channels, Ordering::Relaxed);
        sound.base.sampling_frequency.store(freq, Ordering::Relaxed);
        sound.base.set_volume(volume);
        sound.base.set_pitch(pitch);
        sound.base.set_pan(pan);
        sound.base.set_is_looped(is_looped);
        sound.base.set_dispose_on_stop(dispose_on_stop);
        sound
    }

    pub fn read_streaming_source(&self, buffer: &mut [u8]) -> usize {
        let mut src = self.streaming_source.lock().unwrap();
        if let Some(streaming) = src.as_mut() {
            if streaming.bytes_count() > 0 {
                let mut total = 0;
                while buffer.len() - total > 0 {
                    let n = streaming.read(&mut buffer[total..]);
                    if n > 0 {
                        total += n;
                    } else {
                        if !self.base.is_looped() {
                            break;
                        }
                        streaming.set_position(0);
                    }
                }
                return total;
            }
        }
        0
    }

    pub fn internal_play(&self, _direction: Vector3) {
        // TODO: OpenAL
    }

    pub fn internal_pause(&self) {
        // TODO: OpenAL
    }

    pub fn internal_stop(&self) {
        self.no_more_data.store(false, Ordering::Relaxed);
        if let Some(src) = self.streaming_source.lock().unwrap().as_mut() {
            src.set_position(0);
        }
        // TODO: OpenAL
    }

    pub fn dispose(&self) {
        self.base.dispose();
        *self.streaming_source.lock().unwrap() = None;
    }
}
