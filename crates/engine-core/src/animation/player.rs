use crate::math::{Matrix, Quaternion, Vector3};

/// Animation player state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnimationPlayerState {
    Stopped = 0,
    Playing = 1,
    Paused = 2,
}

/// Animation player that manages animation playback.
pub struct AnimationPlayer {
    /// Is the animation currently playing.
    pub is_playing: bool,
    /// Is the animation looped.
    pub is_looped: bool,
    /// Current playback time in seconds.
    pub time: f32,
    /// Current animation index.
    pub current_animation: usize,
    /// Animations list.
    pub animations: Vec<ModelAnimation>,
    /// Per-animation states.
    pub animation_states: Vec<AnimationPlayerState>,
    /// Whether to preserve the pose when animation stops.
    pub preserve_pose: bool,
    /// Whether the animation has valid data.
    pub has_valid_animation: bool,
}

impl AnimationPlayer {
    /// Create a new animation player.
    pub fn new() -> Self {
        AnimationPlayer {
            is_playing: false,
            is_looped: false,
            time: 0.0,
            current_animation: 0,
            animations: Vec::new(),
            animation_states: Vec::new(),
            preserve_pose: false,
            has_valid_animation: false,
        }
    }

    /// Set the animations list.
    pub fn set_animations(&mut self, animations: Vec<ModelAnimation>) {
        self.animations = animations;
        self.animation_states.resize(animations.len(), AnimationPlayerState::Stopped);
        if !animations.is_empty() {
            self.has_valid_animation = true;
        }
    }

    /// Start playing an animation.
    pub fn play(&mut self, animation_index: usize, loop: bool) {
        if animation_index >= self.animations.len() {
            return;
        }
        self.current_animation = animation_index;
        self.is_playing = true;
        self.is_looped = loop;
        self.animation_states[current_animation] = AnimationPlayerState::Playing;
        self.time = 0.0;
    }

    /// Stop the animation.
    pub fn stop(&mut self) {
        self.is_playing = false;
        self.time = 0.0;
        if self.current_animation < self.animation_states.len() {
            self.animation_states[current_animation] = AnimationPlayerState::Stopped;
        }
    }

    /// Pause the animation.
    pub fn pause(&mut self) {
        if self.is_playing {
            self.is_playing = false;
            if self.current_animation < self.animation_states.len() {
                self.animation_states[current_animation] = AnimationPlayerState::Paused;
            }
        }
    }

    /// Update the animation player.
    pub fn update(&mut self, delta_time: f32) {
        if !self.is_playing {
            return;
        }
        self.time += delta_time;
        
        // Calculate frame based on time
        let animation = &self.animations[self.current_animation];
        let duration = animation.duration;
        
        if duration <= 0.0 {
            return;
        }
        
        // Normalize time
        let normalized_time = if self.is_looped {
            self.time % duration
        } else {
            self.time.min(duration)
        };
        
        // Update animation state
        if self.current_animation < self.animation_states.len() {
            self.animation_states[current_animation] = if self.is_looped {
                AnimationPlayerState::Playing
            } else if self.time >= duration {
                AnimationPlayerState::Stopped
            } else {
                AnimationPlayerState::Playing
            };
        }
    }

    /// Sample bone transforms for the current animation frame.
    pub fn sample_bone_transforms(
        &self,
        bone_transforms: &mut [Matrix],
        model: &crate::graphics::Model,
    ) {
        if self.current_animation >= self.animations.len() {
            return;
        }
        
        let animation = &self.animations[self.current_animation];
        let num_bones = model.bones.len();
        
        // Initialize all transforms to identity
        for transform in bone_transforms.iter_mut().take(num_bones) {
            *transform = Matrix::IDENTITY;
        }
        
        // Sample each animation channel
        for channel in &animation.channels {
            self.sample_channel(channel, model, bone_transforms);
        }
    }
    
    fn sample_channel(&self, channel: &AnimationChannel, model: &crate::graphics::Model, bone_transforms: &mut [Matrix]) {
        let bone_index = model.bone_index(&channel.target_bone_name);
        if bone_index.is_none() {
            return;
        }
        let bone_index = bone_index.unwrap();
        
        let sampler = &channel.sampler;
        let num_frames = sampler.key_times.len();
        if num_frames == 0 {
            return;
        }
        
        // Find the current frame based on time
        let mut frame = 0usize;
        let mut local_time = self.time;
        
        if sampler.interpolation == InterpolationType::Step {
            // Use the closest frame
            frame = ((local_time * sampler.key_times.len() as f32) % sampler.key_times.len() as f32).floor() as usize;
        } else {
            // Linear interpolation
            let time_ratio = local_time / duration(); // need duration
            frame = time_ratio.floor() as usize;
        }
        
        // Clamp frame
        frame = frame.min(num_frames - 1);
        
        // Sample the appropriate property
        match channel.property {
            AnimationProperty::Translation => {
                if sampler.translations.len() > frame {
                    let t = sampler.translations[frame];
                    // Set bone position
                    if bone_index < bone_transforms.len() {
                        // Create translation matrix
                        // ... (simplified)
                    }
                }
            }
            AnimationProperty::Rotation => {
                if sampler.rotations.len() > frame {
                    // Set bone rotation
                }
            }
            AnimationProperty::Scale => {
                if sampler.scales.len() > frame {
                    // Set bone scale
                }
            }
            AnimationProperty::Weights => {
                // Handle morph targets
            }
        }
    }
    
    fn duration(&self) -> f32 {
        if self.current_animation >= self.animations.len() {
            return 0.0;
        }
        self.animations[self.current_animation].duration
    }
}