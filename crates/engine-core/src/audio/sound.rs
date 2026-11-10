use std::sync::Arc;
use super::base_sound::BaseSound;
use super::sound_buffer::SoundBuffer;
use crate::math::vector3::Vector3;

pub struct Sound {
    pub base: BaseSound,
    pub sound_buffer: Option<Arc<SoundBuffer>>,
}

impl Sound {
    pub fn new(
        sound_buffer: Arc<SoundBuffer>,
        volume: f32,
        pitch: f32,
        pan: f32,
        is_looped: bool,
        dispose_on_stop: bool,
    ) -> Self {
        let sound = Sound {
            base: BaseSound::new(),
            sound_buffer: Some(sound_buffer.clone()),
        };
        sound_buffer.increment_use_count();
        sound.base.channels_count.store(
            sound_buffer.channels_count.load(Ordering::Relaxed),
            Ordering::Relaxed,
        );
        sound.base.sampling_frequency.store(
            sound_buffer.sampling_frequency.load(Ordering::Relaxed),
            Ordering::Relaxed,
        );
        sound.base.set_volume(volume);
        sound.base.set_pitch(pitch);
        sound.base.set_pan(pan);
        sound.base.set_is_looped(is_looped);
        sound.base.set_dispose_on_stop(dispose_on_stop);
        sound
    }

    pub fn buffer(&self) -> Option<&Arc<SoundBuffer>> {
        self.sound_buffer.as_ref()
    }

    pub fn internal_play(&self, direction: Vector3) {
        let _ = direction;
        // TODO: OpenAL
    }

    pub fn dispose(&self) {
        self.base.dispose();
        if let Some(buf) = &self.sound_buffer {
            buf.decrement_use_count();
        }
    }
}
