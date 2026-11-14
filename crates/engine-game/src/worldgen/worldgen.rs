use crate::worldgen::SimplexNoise;
use crate::worldgen::worldgen::{Grid2d, Grid3d, squish, LerpExt, TerrainGeneratorConstants};

/// Generate terrain for a chunk region.
/// Ported from TerrainContentsGenerator24.cs GenerateTerrain.
pub fn generate_terrain(
    chunk_origin_x: i32,
    chunk_origin_z: i32,
    x1: usize,
    z1: usize,
    x2: usize,
    z2: usize,
    constants: &TerrainGeneratorConstants,
    ocean_level: i32,
) -> Vec<i32> {
    let num = x2 - x1;
    let num2 = z2 - z1;
    let num3 = chunk_origin_x + x1 as i32;
    let num4 = chunk_origin_z + z1 as i32;

    // Grid2d for ocean shore distance and mountain range factor
    let mut grid2d = Grid2d::new(num, num2);
    let mut grid2d2 = Grid2d::new(num, num2);

    for i in 0..num2 {
        for j in 0..num {
            // CalculateOceanShoreDistance
            grid2d.set(j, i, 0.0); // placeholder - needs actual implementation
            // CalculateMountainRangeFactor
            grid2d2.set(j, i, SimplexNoise::octaved_noise_2d(
                (j + num3) as f32 + constants.tg_mountains_offset_x,
                (i + num4) as f32 + constants.tg_mountains_offset_y,
                constants.tg_mountain_range_freq / constants.tg_biome_scaling,
                3,
                1.91,
                0.75,
                true,
            ));
        }
    }

    // Grid3d for 3D terrain data
    let grid3d_size_x = num / 4 + 1;
    let grid3d_size_y = 33;
    let grid3d_size_z = num2 / 4 + 1;
    let mut grid3d = Grid3d::new(grid3d_size_x, grid3d_size_y, grid3d_size_z);

    for k in 0..grid3d_size_x {
        for l in 0..grid3d_size_z {
            let num5 = (k * 4) as i32 + num3;
            let num6 = (l * 4) as i32 + num4;
            let num7 = 64.0 + 0.0; // CalculateHeight placeholder
            let v = grid2d2.get(k * 4, l * 4);
            let num8 = 1.0; // turbulence
            for m in 0..grid3d_size_y {
                let num9 = (m * 8) as i32;
                let num10 = constants.tg_turbulence_strength
                    * num8
                    * (num7 - num9 as f32).clamp(0.0, f32::MAX)
                    * (2.0 * SimplexNoise::octaved_noise_3d(
                        num5 as f32,
                        num9 as f32,
                        num6 as f32,
                        constants.tg_turbulence_freq,
                        constants.tg_turbulence_octaves,
                        4.0,
                        constants.tg_turbulence_persistence,
                        false,
                    ) - 1.0);
                let num11 = num9 as f32 + num10;
                let mut num12 = num7 - num11;
                num12 += (4.0 * (constants.tg_density_bias - num9 as f32)).max(0.0);
                grid3d.set(k, m, l, num12);
            }
        }
    }

    // Generate cells
    let mut cells = vec![0i32; num * num2 * 256];
    
    for n in 0..(grid3d_size_x - 1) {
        for num13 in 0..(grid3d_size_z - 1) {
            for num14 in 0..(grid3d_size_y - 1) {
                let (v2, v3, v4, v5, v6, v7, v8, v9) = grid3d.get8(n, num14, num13);
                let num15 = (v3 - v2) / 4.0;
                let num16 = (v5 - v4) / 4.0;
                let num17 = (v7 - v6) / 4.0;
                let num18 = (v9 - v8) / 4.0;
                let mut num19 = v2;
                let mut num20 = v4;
                let mut num21 = v6;
                let mut num22 = v8;
                for num23 in 0..4 {
                    let num24 = (num21 - num19) / 4.0;
                    let num25 = (num22 - num20) / 4.0;
                    let mut num26 = num19;
                    let mut num27 = num20;
                    for num28 in 0..4 {
                        let num29 = (num27 - num26) / 8.0;
                        let mut num30 = num26;
                        let num31 = num23 + n * 4;
                        let num32 = num28 + num13 * 4;
                        let x3 = x1 as i32 + num31;
                        let z3 = z1 as i32 + num32;
                        let x4 = grid2d.get(num31 as usize, num32 as usize);
                        let num33 = grid2d2.get(num31 as usize, num32 as usize);
                        // Temperature and humidity from chunk (simplified - use 0)
                        let temperature_fast = 8;
                        let humidity_fast = 8;
                        let f = num33 - 0.01 * humidity_fast as f32;
                        let num34 = (100.0f32).lerp(0.0, f.clamp(0.0, 1.0));
                        let num35 = (300.0f32).lerp(30.0, f.clamp(0.0, 1.0));
                        let flag = (temperature_fast > 8 && humidity_fast < 8 && num33 < 0.97)
                            || (x4.abs() < 16.0 && num33 < 0.97);
                        for num37 in 0..8 {
                            let num38 = num37 + num14 * 8;
                            let value = if num30 < 0.0 {
                                if num38 <= ocean_level {
                                    18 // Water
                                } else {
                                    0 // Air
                                }
                            } else if !flag {
                                if num30 < num35 {
                                    3 // Dirt
                                } else {
                                    67 // Stone
                                }
                            } else if num30 < num34 {
                                if num30 < num35 {
                                    3 // Dirt
                                } else {
                                    67 // Stone
                                }
                            } else {
                                if num30 < num35 {
                                    3 // Dirt
                                } else {
                                    67 // Stone
                                }
                            };
                            // Set cell value
                            let cell_idx = (num38 + (x3 as usize) * 256 + (z3 as usize) * 256 * 16) as usize;
                            if cell_idx < cells.len() {
                                cells[cell_idx] = value;
                            }
                            num30 += num29;
                        }
                        num26 += num24;
                        num27 += num25;
                    }
                    num19 += num15;
                    num20 += num16;
                    num21 += num17;
                    num22 += num18;
                }
            }
        }
    }

    cells
}

/// Generate surface parameters.
pub fn generate_surface_parameters(
    chunk_origin_x: i32,
    chunk_origin_z: i32,
    x1: usize,
    z1: usize,
    x2: usize,
    z2: usize,
) {
    // Simplified - sets temperature and humidity for each cell
    for i in z1..z2 {
        for j in x1..x2 {
            let x = chunk_origin_x + j as i32;
            let z = chunk_origin_z + i as i32;
            // Temperature and humidity would be set here
            // Simplified for now
            let _ = (x, z);
        }
    }
}