//! Immutable render state objects.

use crate::math::Vector4;
use super::enums::{Blend, BlendFunction, CompareFunction, CullMode, TextureAddressMode, TextureFilterMode};
use super::lock_on_first_use::LockOnFirstUse;

#[derive(Debug, Clone, PartialEq)]
pub struct BlendState {
    pub lock: LockOnFirstUse,
    pub color_source_blend: Blend,
    pub color_destination_blend: Blend,
    pub color_blend_function: BlendFunction,
    pub alpha_source_blend: Blend,
    pub alpha_destination_blend: Blend,
    pub alpha_blend_function: BlendFunction,
    pub blend_factor: Vector4,
}

impl Default for BlendState {
    fn default() -> Self {
        BlendState {
            lock: LockOnFirstUse::new(),
            color_source_blend: Blend::One,
            color_destination_blend: Blend::Zero,
            color_blend_function: BlendFunction::Add,
            alpha_source_blend: Blend::One,
            alpha_destination_blend: Blend::Zero,
            alpha_blend_function: BlendFunction::Add,
            blend_factor: Vector4::ZERO,
        }
    }
}

impl BlendState {
    /// Fully opaque: writes replace destination colour and alpha.
    pub fn opaque() -> Self {
        BlendState { lock: LockOnFirstUse::locked(), ..Default::default() }
    }

    /// Additive blending, useful for glows and particles.
    pub fn additive() -> Self {
        BlendState {
            lock: LockOnFirstUse::locked(),
            color_source_blend: Blend::SourceAlpha,
            color_destination_blend: Blend::One,
            alpha_source_blend: Blend::SourceAlpha,
            alpha_destination_blend: Blend::One,
            ..Default::default()
        }
    }

    /// Classic `src * 1 + dst * (1 - srcAlpha)` blending with separate alpha channel.
    pub fn alpha_blend() -> Self {
        BlendState {
            lock: LockOnFirstUse::locked(),
            color_source_blend: Blend::One,
            color_destination_blend: Blend::InverseSourceAlpha,
            alpha_source_blend: Blend::One,
            alpha_destination_blend: Blend::InverseSourceAlpha,
            ..Default::default()
        }
    }

    /// Non-premultiplied alpha blending.
    pub fn non_premultiplied() -> Self {
        BlendState {
            lock: LockOnFirstUse::locked(),
            color_source_blend: Blend::SourceAlpha,
            color_destination_blend: Blend::InverseSourceAlpha,
            alpha_source_blend: Blend::SourceAlpha,
            alpha_destination_blend: Blend::InverseSourceAlpha,
            ..Default::default()
        }
    }

    /// Compare only the blend factors, matching the C# `BaseEquals`.
    pub fn base_equals(&self, other: &BlendState) -> bool {
        self.color_source_blend == other.color_source_blend
            && self.color_destination_blend == other.color_destination_blend
            && self.alpha_source_blend == other.alpha_source_blend
            && self.alpha_destination_blend == other.alpha_destination_blend
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DepthStencilState {
    pub lock: LockOnFirstUse,
    pub depth_buffer_test_enable: bool,
    pub depth_buffer_write_enable: bool,
    pub depth_buffer_function: CompareFunction,
}

impl Default for DepthStencilState {
    fn default() -> Self {
        DepthStencilState {
            lock: LockOnFirstUse::new(),
            depth_buffer_test_enable: true,
            depth_buffer_write_enable: true,
            depth_buffer_function: CompareFunction::LessEqual,
        }
    }
}

impl DepthStencilState {
    pub fn locked_default() -> Self {
        DepthStencilState { lock: LockOnFirstUse::locked(), ..Default::default() }
    }

    pub fn depth_read() -> Self {
        DepthStencilState {
            lock: LockOnFirstUse::locked(),
            depth_buffer_write_enable: false,
            ..Default::default()
        }
    }

    pub fn depth_write() -> Self {
        DepthStencilState {
            lock: LockOnFirstUse::locked(),
            depth_buffer_test_enable: false,
            ..Default::default()
        }
    }

    pub fn none() -> Self {
        DepthStencilState {
            lock: LockOnFirstUse::locked(),
            depth_buffer_test_enable: false,
            depth_buffer_write_enable: false,
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RasterizerState {
    pub lock: LockOnFirstUse,
    pub cull_mode: CullMode,
    pub scissor_test_enable: bool,
    pub depth_bias: f32,
    pub slope_scale_depth_bias: f32,
}

impl Default for RasterizerState {
    fn default() -> Self {
        RasterizerState {
            lock: LockOnFirstUse::new(),
            cull_mode: CullMode::CullCounterClockwise,
            scissor_test_enable: false,
            depth_bias: 0.0,
            slope_scale_depth_bias: 0.0,
        }
    }
}

impl RasterizerState {
    pub fn cull_none() -> Self {
        RasterizerState {
            lock: LockOnFirstUse::locked(),
            cull_mode: CullMode::None,
            ..Default::default()
        }
    }

    pub fn cull_none_scissor() -> Self {
        RasterizerState {
            lock: LockOnFirstUse::locked(),
            cull_mode: CullMode::None,
            scissor_test_enable: true,
            ..Default::default()
        }
    }

    pub fn cull_clockwise() -> Self {
        RasterizerState {
            lock: LockOnFirstUse::locked(),
            cull_mode: CullMode::CullClockwise,
            ..Default::default()
        }
    }

    pub fn cull_clockwise_scissor() -> Self {
        RasterizerState {
            lock: LockOnFirstUse::locked(),
            cull_mode: CullMode::CullClockwise,
            scissor_test_enable: true,
            ..Default::default()
        }
    }

    pub fn cull_counter_clockwise() -> Self {
        RasterizerState {
            lock: LockOnFirstUse::locked(),
            cull_mode: CullMode::CullCounterClockwise,
            ..Default::default()
        }
    }

    pub fn cull_counter_clockwise_scissor() -> Self {
        RasterizerState {
            lock: LockOnFirstUse::locked(),
            cull_mode: CullMode::CullCounterClockwise,
            scissor_test_enable: true,
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SamplerState {
    pub lock: LockOnFirstUse,
    pub filter_mode: TextureFilterMode,
    pub address_mode_u: TextureAddressMode,
    pub address_mode_v: TextureAddressMode,
    pub max_anisotropy: i32,
    pub min_lod: f32,
    pub max_lod: f32,
    pub mip_lod_bias: f32,
}

impl Default for SamplerState {
    fn default() -> Self {
        SamplerState {
            lock: LockOnFirstUse::new(),
            filter_mode: TextureFilterMode::Point,
            address_mode_u: TextureAddressMode::Clamp,
            address_mode_v: TextureAddressMode::Clamp,
            max_anisotropy: 1,
            min_lod: -1000.0,
            max_lod: 1000.0,
            mip_lod_bias: 0.0,
        }
    }
}

impl SamplerState {
    pub fn point_clamp() -> Self {
        SamplerState { lock: LockOnFirstUse::locked(), ..Default::default() }
    }

    pub fn point_wrap() -> Self {
        SamplerState {
            lock: LockOnFirstUse::locked(),
            address_mode_u: TextureAddressMode::Wrap,
            address_mode_v: TextureAddressMode::Wrap,
            ..Default::default()
        }
    }

    pub fn linear_clamp() -> Self {
        SamplerState {
            lock: LockOnFirstUse::locked(),
            filter_mode: TextureFilterMode::Linear,
            ..Default::default()
        }
    }

    pub fn linear_wrap() -> Self {
        SamplerState {
            lock: LockOnFirstUse::locked(),
            filter_mode: TextureFilterMode::Linear,
            address_mode_u: TextureAddressMode::Wrap,
            address_mode_v: TextureAddressMode::Wrap,
            ..Default::default()
        }
    }

    pub fn anisotropic_clamp() -> Self {
        SamplerState {
            lock: LockOnFirstUse::locked(),
            filter_mode: TextureFilterMode::Anisotropic,
            max_anisotropy: 16,
            ..Default::default()
        }
    }

    pub fn anisotropic_wrap() -> Self {
        SamplerState {
            lock: LockOnFirstUse::locked(),
            filter_mode: TextureFilterMode::Anisotropic,
            address_mode_u: TextureAddressMode::Wrap,
            address_mode_v: TextureAddressMode::Wrap,
            max_anisotropy: 16,
            ..Default::default()
        }
    }
}