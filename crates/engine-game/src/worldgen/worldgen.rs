use crate::worldgen::SimplexNoise;

/// Terrain world generator constants and functions.
pub struct TerrainContentsGenerator24;

impl TerrainContentsGenerator24 {
    // World generation constants (from TerrainContentsGenerator24.cs)
    pub const TG_OCEAN_SLOPE: f32 = 0.0; // placeholder, adjust from source
    pub const TG_OCEAN_SLOPE_VARIATION: f32 = 0.0;
    pub const TG_ISLANDS_FREQUENCY: f32 = 0.0;
    pub const TG_HILLS_PERCENTAGE: f32 = 0.0;
    pub const TG_MOUNTAINS_PERCENTAGE: f32 = 0.0;
    pub const TG_HILLS_FREQUENCY: f32 = 0.0;
    pub const TG_HILLS_OCTAVES: i32 = 0;
    pub const TG_HILLS_PERSISTENCE: f32 = 0.0;
    pub const TG_HILLS_STRENGTH: f32 = 0.0;
    pub const TG_MOUNTAINS_DETAIL_FREQ: f32 = 0.0;
    pub const TG_MOUNTAINS_DETAIL_OCTAVES: i32 = 0;
    pub const TG_MOUNTAINS_DETAIL_PERSISTENCE: f32 = 0.0;
    pub const TG_MOUNTAINS_STRENGTH: f32 = 0.0;
    pub const TG_RIVERS_STRENGTH: f32 = 0.0;
    pub const TG_HEIGHT_BIAS: f32 = 0.0;
    pub const TG_SHORE_FLUCTUATIONS: f32 = 0.0;
    pub const TG_SHORE_FLUCTUATIONS_SCALING: f32 = 0.0;
    pub const TG_BIOME_SCALING: f32 = 1.0;
    pub const TG_MOUNTAIN_RANGE_FREQ: f32 = 0.0;
    pub const TG_MOUNTAINS_OFFSET_X: f32 = 0.0;
    pub const TG_MOUNTAINS_OFFSET_Y: f32 = 0.0;
    pub const TG_RIVERS_OFFSET_X: f32 = 0.0;
    pub const TG_RIVERS_OFFSET_Y: f32 = 0.0;
    pub const TG_TEMPERATURE_OFFSET_X: f32 = 0.0;
    pub const TG_TEMPERATURE_OFFSET_Y: f32 = 0.0;
    pub const TG_HUMIDITY_OFFSET_X: f32 = 0.0;
    pub const TG_HUMIDITY_OFFSET_Y: f32 = 0.0;
    pub const SEA_LEVEL_OFFSET: i32 = 0;

    /// Calculate ocean shore distance.
    pub fn calculate_ocean_shore_distance(x: f32, z: f32, island_size: Option<(f32, f32)>) -> f32 {
        if let Some((sx, sy)) = island_size {
            let num = Self::calculate_ocean_shore_x(z);
            let num2 = Self::calculate_ocean_shore_z(x);
            let num3 = Self::calculate_ocean_shore_x(z + 1000.0) + sx;
            let num4 = Self::calculate_ocean_shore_z(x + 1000.0) + sy;
            (x - num).min(z - num2).min(num3 - x).min(num4 - z)
        } else {
            let num5 = Self::calculate_ocean_shore_x(z);
            let num6 = Self::calculate_ocean_shore_z(x);
            (x - num5).min(z - num6)
        }
    }

    /// Calculate mountain range factor.
    pub fn calculate_mountain_range_factor(x: f32, z: f32) -> f32 {
        SimplexNoise::octaved_noise_2d(
            x + Self::TG_MOUNTAINS_OFFSET_X,
            z + Self::TG_MOUNTAINS_OFFSET_Y,
            Self::TG_MOUNTAIN_RANGE_FREQ / Self::TG_BIOME_SCALING,
            3,
            1.91,
            0.75,
            true,
        )
    }

    /// Squish function.
    pub fn squish(v: f32, zero: f32, one: f32) -> f32 {
        ((v - zero) / (one - zero)).clamp(0.0, 1.0)
    }

    /// Calculate ocean shore X.
    pub fn calculate_ocean_shore_x(z: f32) -> f32 {
        0.0 + Self::TG_SHORE_FLUCTUATIONS * SimplexNoise::octaved_noise_2d(
            z,
            0.0,
            0.005 / Self::TG_SHORE_FLUCTUATIONS_SCALING,
            4,
            1.95,
            1.0,
            false,
        )
    }

    /// Calculate ocean shore Z.
    pub fn calculate_ocean_shore_z(x: f32) -> f32 {
        0.0 + Self::TG_SHORE_FLUCTUATIONS * SimplexNoise::octaved_noise_2d(
            0.0,
            x,
            0.005 / Self::TG_SHORE_FLUCTUATIONS_SCALING,
            4,
            1.95,
            1.0,
            false,
        )
    }

    /// Calculate height (from TerrainContentsGenerator24.cs).
    pub fn calculate_height(x: f32, z: f32) -> f32 {
        let num = Self::TG_OCEAN_SLOPE
            + Self::TG_OCEAN_SLOPE_VARIATION
                * (2.0 * SimplexNoise::octaved_noise_2d(
                    x + Self::TG_MOUNTAINS_OFFSET_X,
                    z + Self::TG_MOUNTAINS_OFFSET_Y,
                    0.01,
                    1,
                    2.0,
                    0.5,
                    false,
                ) - 1.0)
                    .abs()
                    .powf(0.5)
                    * (if 2.0 * SimplexNoise::octaved_noise_2d(
                        x + Self::TG_MOUNTAINS_OFFSET_X,
                        z + Self::TG_MOUNTAINS_OFFSET_Y,
                        0.01,
                        1,
                        2.0,
                        0.5,
                        false,
                    ) - 1.0 < 0.0 { -1.0 } else { 1.0 });

        let num2 = Self::calculate_ocean_shore_distance(x, z, None);
        let num3 = (2.0 - 0.05 * num2.abs()).clamp(0.0, 1.0);
        let num4 = (num2 * 0.05).sin().clamp(0.0, 1.0);
        let num5 = ((-num) * num2).clamp(0.0, 1.0) - 0.85 * num4;
        let num5 = num5.clamp(0.0, 1.0);
        let num6 = ((-0.05 * (num2 + 10.0)).clamp(0.0, 1.0) - num4).clamp(0.0, 1.0);
        let v = Self::calculate_mountain_range_factor(x, z);
        let f = (1.0 - num3) * SimplexNoise::octaved_noise_2d(x, z, 0.001 / Self::TG_BIOME_SCALING, 2, 2.0, 0.5, false);
        let f2 = (1.0 - num3) * SimplexNoise::octaved_noise_2d(x, z, 0.0017 / Self::TG_BIOME_SCALING, 2, 4.0, 0.7, false);
        let num7 = (1.0 - num6) * (1.0 - num3) * Self::squish(v, 1.0 - Self::TG_HILLS_PERCENTAGE, 1.0 - Self::TG_MOUNTAINS_PERCENTAGE);
        let num8 = (1.0 - num6) * Self::squish(v, 1.0 - Self::TG_MOUNTAINS_PERCENTAGE, 1.0);
        let num9 = Self::TG_HILLS_STRENGTH * SimplexNoise::octaved_noise_2d(x, z, Self::TG_HILLS_FREQUENCY, Self::TG_HILLS_OCTAVES, 1.93, Self::TG_HILLS_PERSISTENCE, false);
        let amplitude_step = (0.75 * Self::TG_MOUNTAINS_DETAIL_PERSISTENCE).lerp(1.33 * Self::TG_MOUNTAINS_DETAIL_PERSISTENCE, f.clamp(0.0, 1.0));
        let num10 = 1.5 * SimplexNoise::octaved_noise_2d(x, z, Self::TG_MOUNTAINS_DETAIL_FREQ, Self::TG_MOUNTAINS_DETAIL_OCTAVES, 1.98, amplitude_step, false) - 0.5;
        let num11 = (80.0).lerp(35.0, (1.0 * num8 + 0.5 * num7 + (1.0 - num2 / 30.0).clamp(0.0, 1.0)).clamp(0.0, 1.0));
        let x2 = (-2.0).lerp(-4.0, (num8 + 0.5 * num7).clamp(0.0, 1.0));
        let num12 = (1.5 - num11 * (2.0 * SimplexNoise::octaved_noise_2d(x + Self::TG_RIVERS_OFFSET_X, z + Self::TG_RIVERS_OFFSET_Y, 0.001, 4, 2.0, 0.5, false) - 1.0).abs()).clamp(0.0, 1.0);
        let num13 = -50.0 * num5 + Self::TG_HEIGHT_BIAS;
        let num14 = (0.0).lerp(8.0, f);
        let num15 = (0.0).lerp(-6.0, f2);
        let num16 = Self::TG_HILLS_STRENGTH * num7 * num9;
        let num17 = Self::TG_MOUNTAINS_STRENGTH * num8 * num10;
        let f3 = Self::TG_RIVERS_STRENGTH * num12;
        let num18 = num13 + num14 + num15 + num17 + num16;
        let num19 = num18.min(x2 * f3 + num18 * (1.0 - f3));
        (64.0 + num19).clamp(10.0, 251.0)
    }

    /// Calculate temperature.
    pub fn calculate_temperature(x: f32, z: f32, temperature_offset: f32) -> i32 {
        (3.0 * SimplexNoise::octaved_noise_2d(
            x + Self::TG_TEMPERATURE_OFFSET_X,
            z + Self::TG_TEMPERATURE_OFFSET_Y,
            0.0015 / Self::TG_BIOME_SCALING,
            5,
            2.0,
            0.6,
            false,
        ) - 1.1 + temperature_offset / 16.0)
            .clamp(0.0, 1.0) * 16.0) as i32
    }

    /// Calculate humidity.
    pub fn calculate_humidity(x: f32, z: f32, humidity_offset: f32) -> i32 {
        (3.0 * SimplexNoise::octaved_noise_2d(
            x + Self::TG_HUMIDITY_OFFSET_X,
            z + Self::TG_HUMIDITY_OFFSET_Y,
            0.0012 / Self::TG_BIOME_SCALING,
            5,
            2.0,
            0.6,
            false,
        ) - 0.9 + humidity_offset / 16.0)
            .clamp(0.0, 1.0) * 16.0) as i32
    }

    /// Calculate forest density.
    pub fn calculate_forest_density(x: f32, z: f32) -> f32 {
        // Simplified - uses chunk coordinates
        let chunk_x = (x / 16.0).floor() as i32;
        let chunk_z = (z / 16.0).floor() as i32;
        let flag = (chunk_x + 1000 * chunk_z) as u32 % 1000 < 300;
        (SimplexNoise::octaved_noise_2d(chunk_x as f32, chunk_z as f32, 0.1, 2, 2.0, 0.5, false) - 0.25) / 0.2 + if flag { 0.6 } else { 0.0 }
    }
}

/// Extension trait for f32 lerp.
trait LerpExt {
    fn lerp(self, other: f32, t: f32) -> f32;
}

impl LerpExt for f32 {
    fn lerp(self, other: f32, t: f32) -> f32 {
        self + (other - self) * t
    }
}