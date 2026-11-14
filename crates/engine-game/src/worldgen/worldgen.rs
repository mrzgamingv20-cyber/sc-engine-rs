use crate::worldgen::SimplexNoise;

/// Grid2d - 2D grid for terrain generation.
pub struct Grid2d {
    pub size_x: usize,
    pub size_y: usize,
    pub data: Vec<f32>,
}

impl Grid2d {
    pub fn new(size_x: usize, size_y: usize) -> Self {
        Grid2d {
            size_x,
            size_y,
            data: vec![0.0; size_x * size_y],
        }
    }

    pub fn get(&self, x: usize, y: usize) -> f32 {
        self.data[x + y * self.size_x]
    }

    pub fn set(&mut self, x: usize, y: usize, value: f32) {
        self.data[x + y * self.size_x] = value;
    }

    pub fn sample(&self, x: f32, y: f32) -> f32 {
        let num = x.floor() as usize;
        let num2 = y.floor() as usize;
        let num3 = x.ceil() as usize;
        let num4 = y.ceil() as usize;
        let f = x - num as f32;
        let f2 = y - num2 as f32;
        let x2 = self.data[num + num2 * self.size_x];
        let x3 = self.data[num3 + num2 * self.size_x];
        let x4 = self.data[num + num4 * self.size_x];
        let x5 = self.data[num3 + num4 * self.size_x];
        let x6 = x2 + f * (x3 - x2);
        let x7 = x4 + f * (x5 - x4);
        x6 + f2 * (x7 - x6)
    }
}

/// Grid3d - 3D grid for terrain generation.
pub struct Grid3d {
    pub size_x: usize,
    pub size_y: usize,
    pub size_z: usize,
    pub size_xy: usize,
    pub data: Vec<f32>,
}

impl Grid3d {
    pub fn new(size_x: usize, size_y: usize, size_z: usize) -> Self {
        Grid3d {
            size_x,
            size_y,
            size_z,
            size_xy: size_x * size_y,
            data: vec![0.0; size_x * size_y * size_z],
        }
    }

    pub fn get(&self, x: usize, y: usize, z: usize) -> f32 {
        self.data[x + y * self.size_x + z * self.size_xy]
    }

    pub fn set(&mut self, x: usize, y: usize, z: usize, value: f32) {
        self.data[x + y * self.size_x + z * self.size_xy] = value;
    }

    pub fn get8(&self, x: usize, y: usize, z: usize) -> (f32, f32, f32, f32, f32, f32, f32, f32) {
        let num = x + y * self.size_x + z * self.size_xy;
        (
            self.data[num],
            self.data[num + 1],
            self.data[num + self.size_x],
            self.data[num + 1 + self.size_x],
            self.data[num + self.size_xy],
            self.data[num + 1 + self.size_xy],
            self.data[num + self.size_x + self.size_xy],
            self.data[num + 1 + self.size_x + self.size_xy],
        )
    }
}

/// Terrain contents generator constants.
pub struct TerrainGeneratorConstants {
    pub tg_ocean_slope: f32,
    pub tg_ocean_slope_variation: f32,
    pub tg_islands_frequency: f32,
    pub tg_hills_percentage: f32,
    pub tg_mountains_percentage: f32,
    pub tg_hills_frequency: f32,
    pub tg_hills_octaves: i32,
    pub tg_hills_persistence: f32,
    pub tg_hills_strength: f32,
    pub tg_mountains_detail_freq: f32,
    pub tg_mountains_detail_octaves: i32,
    pub tg_mountains_detail_persistence: f32,
    pub tg_mountains_strength: f32,
    pub tg_rivers_strength: f32,
    pub tg_height_bias: f32,
    pub tg_shore_fluctuations: f32,
    pub tg_shore_fluctuations_scaling: f32,
    pub tg_biome_scaling: f32,
    pub tg_mountain_range_freq: f32,
    pub tg_mountains_offset_x: f32,
    pub tg_mountains_offset_y: f32,
    pub tg_rivers_offset_x: f32,
    pub tg_rivers_offset_y: f32,
    pub tg_temperature_offset_x: f32,
    pub tg_temperature_offset_y: f32,
    pub tg_humidity_offset_x: f32,
    pub tg_humidity_offset_y: f32,
    pub tg_min_turbulence: f32,
    pub tg_turbulence_zero: f32,
    pub tg_turbulence_strength: f32,
    pub tg_turbulence_freq: f32,
    pub tg_turbulence_octaves: i32,
    pub tg_turbulence_persistence: f32,
    pub tg_density_bias: f32,
    pub tg_surface_multiplier: f32,
    pub ocean_level: i32,
}

impl TerrainGeneratorConstants {
    pub fn default() -> Self {
        TerrainGeneratorConstants {
            tg_ocean_slope: 0.0,
            tg_ocean_slope_variation: 0.0,
            tg_islands_frequency: 0.0,
            tg_hills_percentage: 0.0,
            tg_mountains_percentage: 0.0,
            tg_hills_frequency: 0.0,
            tg_hills_octaves: 0,
            tg_hills_persistence: 0.0,
            tg_hills_strength: 0.0,
            tg_mountains_detail_freq: 0.0,
            tg_mountains_detail_octaves: 0,
            tg_mountains_detail_persistence: 0.0,
            tg_mountains_strength: 0.0,
            tg_rivers_strength: 0.0,
            tg_height_bias: 0.0,
            tg_shore_fluctuations: 0.0,
            tg_shore_fluctuations_scaling: 0.0,
            tg_biome_scaling: 1.0,
            tg_mountain_range_freq: 0.0,
            tg_mountains_offset_x: 0.0,
            tg_mountains_offset_y: 0.0,
            tg_rivers_offset_x: 0.0,
            tg_rivers_offset_y: 0.0,
            tg_temperature_offset_x: 0.0,
            tg_temperature_offset_y: 0.0,
            tg_humidity_offset_x: 0.0,
            tg_humidity_offset_y: 0.0,
            tg_min_turbulence: 0.0,
            tg_turbulence_zero: 0.0,
            tg_turbulence_strength: 0.0,
            tg_turbulence_freq: 0.0,
            tg_turbulence_octaves: 0,
            tg_turbulence_persistence: 0.0,
            tg_density_bias: 0.0,
            tg_surface_multiplier: 0.0,
            ocean_level: 64,
        }
    }
}

/// Lerp extension for f32.
trait LerpExt {
    fn lerp(self, other: f32, t: f32) -> f32;
}

impl LerpExt for f32 {
    fn lerp(self, other: f32, t: f32) -> f32 {
        self + (other - self) * t
    }
}

/// Squish function.
pub fn squish(v: f32, zero: f32, one: f32) -> f32 {
    ((v - zero) / (one - zero)).clamp(0.0, 1.0)
}