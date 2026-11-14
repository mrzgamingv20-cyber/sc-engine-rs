/// Terrain world generator constants.
pub struct WorldGenConstants {
    pub tg_biome_scaling: f32,
    pub tg_shore_fluctuations: f32,
    pub tg_shore_fluctuations_scaling: f32,
    pub tg_ocean_slope: f32,
    pub tg_ocean_slope_variation: f32,
    pub tg_islands_frequency: f32,
    pub tg_density_bias: f32,
    pub tg_height_bias: f32,
    pub tg_rivers_strength: f32,
    pub tg_mountains_strength: f32,
    pub tg_mountain_range_freq: f32,
    pub tg_mountains_percentage: f32,
    pub tg_mountains_detail_freq: f32,
    pub tg_mountains_detail_octaves: i32,
    pub tg_mountains_detail_persistence: f32,
    pub tg_hills_percentage: f32,
    pub tg_hills_strength: f32,
    pub tg_hills_octaves: i32,
    pub tg_hills_frequency: f32,
    pub tg_hills_persistence: f32,
    pub tg_turbulence_strength: f32,
    pub tg_turbulence_freq: f32,
    pub tg_turbulence_octaves: i32,
    pub tg_turbulence_persistence: f32,
    pub tg_min_turbulence: f32,
    pub tg_turbulence_zero: f32,
    pub tg_surface_multiplier: f32,
    pub tg_water: bool,
    pub tg_extras: bool,
    pub tg_caves_and_pockets: bool,
    pub ocean_level: i32,
}

impl WorldGenConstants {
    pub fn default() -> Self {
        WorldGenConstants {
            tg_biome_scaling: 1.75,
            tg_shore_fluctuations: 0.0,
            tg_shore_fluctuations_scaling: 1.0,
            tg_ocean_slope: 0.006,
            tg_ocean_slope_variation: 0.004,
            tg_islands_frequency: 0.01,
            tg_density_bias: 55.0,
            tg_height_bias: 1.0,
            tg_rivers_strength: 1.0,
            tg_mountains_strength: 220.0,
            tg_mountain_range_freq: 0.0006,
            tg_mountains_percentage: 0.11,
            tg_mountains_detail_freq: 0.003,
            tg_mountains_detail_octaves: 3,
            tg_mountains_detail_persistence: 0.53,
            tg_hills_percentage: 0.28,
            tg_hills_strength: 32.0,
            tg_hills_octaves: 1,
            tg_hills_frequency: 0.014,
            tg_hills_persistence: 0.5,
            tg_turbulence_strength: 55.0,
            tg_turbulence_freq: 0.03,
            tg_turbulence_octaves: 1,
            tg_turbulence_persistence: 0.5,
            tg_min_turbulence: 0.04,
            tg_turbulence_zero: 0.84,
            tg_surface_multiplier: 2.0,
            tg_water: true,
            tg_extras: true,
            tg_caves_and_pockets: true,
            ocean_level: 64,
        }
    }
}

/// Squish function.
pub fn squish(v: f32, zero: f32, one: f32) -> f32 {
    ((v - zero) / (one - zero)).clamp(0.0, 1.0)
}

/// Lerp extension for f32.
pub trait LerpExt {
    fn lerp(self, other: f32, t: f32) -> f32;
}

impl LerpExt for f32 {
    fn lerp(self, other: f32, t: f32) -> f32 {
        self + (other - self) * t
    }
}