use super::block::{Block, BlockDigMethod};
use super::block::FoodType;

/// Dirt block - basic solid cube.
pub struct DirtBlock {
    pub block_index: i32,
}

impl DirtBlock {
    pub const INDEX: i32 = 2;
    pub const NAME: &'static str = "DirtBlock";

    pub fn new() -> Self {
        DirtBlock { block_index: Self::INDEX }
    }
}

impl Block for DirtBlock {
    fn block_index(&self) -> i32 { self.block_index }
    fn set_block_index(&mut self, index: i32) { self.block_index = index; }
    fn default_display_name(&self) -> &str { "Dirt" }
    fn default_description(&self) -> &str { "Dirt" }
    fn default_category(&self) -> &str { "Terrain" }
    fn display_order(&self) -> i32 { 1 }
    fn is_collidable(&self) -> bool { true }
    fn is_placeable(&self) -> bool { true }
    fn is_transparent(&self) -> bool { false }
    fn light_attenuation(&self) -> i32 { 1 }
    fn default_emitted_light(&self) -> i32 { 0 }
    fn default_texture_slot(&self) -> i32 { 1 }
    fn default_shadow_strength(&self) -> i32 { 15 }
    fn is_fluid_blocker(&self) -> bool { true }
    fn is_gatherable(&self) -> bool { true }
    fn density(&self) -> f32 { 1.0 }
    fn fuel_heat_level(&self) -> f32 { 0.0 }
    fn fuel_fire_duration(&self) -> f32 { 0.0 }
    fn shovel_power(&self) -> f32 { 1.0 }
    fn quarry_power(&self) -> f32 { 1.0 }
    fn hack_power(&self) -> f32 { 1.0 }
    fn melee_power(&self) -> f32 { 1.0 }
    fn melee_hit_probability(&self) -> f32 { 0.66 }
    fn projectile_power(&self) -> f32 { 1.0 }
    fn dig_method(&self) -> BlockDigMethod { BlockDigMethod::Shovel }
    fn dig_resilience(&self) -> f32 { 1.0 }
    fn max_stacking(&self) -> i32 { 40 }
    fn sleep_suitability(&self) -> f32 { 0.0 }
    fn friction_factor(&self) -> f32 { 1.0 }
    fn is_suitable_for_plants(&self, _value: i32, _plant_value: i32) -> bool { true }
}

/// Sand block - basic solid cube.
pub struct SandBlock {
    pub block_index: i32,
}

impl SandBlock {
    pub const INDEX: i32 = 3;
    pub const NAME: &'static str = "SandBlock";

    pub fn new() -> Self {
        SandBlock { block_index: Self::INDEX }
    }
}

impl Block for SandBlock {
    fn block_index(&self) -> i32 { self.block_index }
    fn set_block_index(&mut self, index: i32) { self.block_index = index; }
    fn default_display_name(&self) -> &str { "Sand" }
    fn default_description(&self) -> &str { "Sand" }
    fn default_category(&self) -> &str { "Terrain" }
    fn display_order(&self) -> i32 { 2 }
    fn is_collidable(&self) -> bool { true }
    fn is_placeable(&self) -> bool { true }
    fn is_transparent(&self) -> bool { false }
    fn light_attenuation(&self) -> i32 { 1 }
    fn default_emitted_light(&self) -> i32 { 0 }
    fn default_texture_slot(&self) -> i32 { 2 }
    fn default_shadow_strength(&self) -> i32 { 15 }
    fn is_fluid_blocker(&self) -> bool { true }
    fn is_gatherable(&self) -> bool { true }
    fn density(&self) -> f32 { 1.5 }
    fn fuel_heat_level(&self) -> f32 { 0.0 }
    fn fuel_fire_duration(&self) -> f32 { 0.0 }
    fn shovel_power(&self) -> f32 { 1.0 }
    fn quarry_power(&self) -> f32 { 1.0 }
    fn hack_power(&self) -> f32 { 1.0 }
    fn melee_power(&self) -> f32 { 1.0 }
    fn melee_hit_probability(&self) -> f32 { 0.66 }
    fn projectile_power(&self) -> f32 { 1.0 }
    fn dig_method(&self) -> BlockDigMethod { BlockDigMethod::Shovel }
    fn dig_resilience(&self) -> f32 { 1.0 }
    fn max_stacking(&self) -> i32 { 40 }
    fn sleep_suitability(&self) -> f32 { 0.0 }
    fn friction_factor(&self) -> f32 { 1.0 }
}

/// Basalt block - basic solid cube.
pub struct BasaltBlock {
    pub block_index: i32,
}

impl BasaltBlock {
    pub const INDEX: i32 = 4;
    pub const NAME: &'static str = "BasaltBlock";

    pub fn new() -> Self {
        BasaltBlock { block_index: Self::INDEX }
    }
}

impl Block for BasaltBlock {
    fn block_index(&self) -> i32 { self.block_index }
    fn set_block_index(&mut self, index: i32) { self.block_index = index; }
    fn default_display_name(&self) -> &str { "Basalt" }
    fn default_description(&self) -> &str { "Basalt" }
    fn default_category(&self) -> &str { "Terrain" }
    fn display_order(&self) -> i32 { 3 }
    fn is_collidable(&self) -> bool { true }
    fn is_placeable(&self) -> bool { true }
    fn is_transparent(&self) -> bool { false }
    fn light_attenuation(&self) -> i32 { 1 }
    fn default_emitted_light(&self) -> i32 { 0 }
    fn default_texture_slot(&self) -> i32 { 3 }
    fn default_shadow_strength(&self) -> i32 { 15 }
    fn is_fluid_blocker(&self) -> bool { true }
    fn is_gatherable(&self) -> bool { true }
    fn density(&self) -> f32 { 2.0 }
    fn fuel_heat_level(&self) -> f32 { 0.0 }
    fn fuel_fire_duration(&self) -> f32 { 0.0 }
    fn shovel_power(&self) -> f32 { 1.0 }
    fn quarry_power(&self) -> f32 { 1.0 }
    fn hack_power(&self) -> f32 { 1.0 }
    fn melee_power(&self) -> f32 { 1.0 }
    fn melee_hit_probability(&self) -> f32 { 0.66 }
    fn projectile_power(&self) -> f32 { 1.0 }
    fn dig_method(&self) -> BlockDigMethod { BlockDigMethod::Quarry }
    fn dig_resilience(&self) -> f32 { 1.0 }
    fn max_stacking(&self) -> i32 { 40 }
    fn sleep_suitability(&self) -> f32 { 0.0 }
    fn friction_factor(&self) -> f32 { 1.0 }
}

use super::block::{Block, BlockDigMethod};
use super::block::FoodType;

/// Leaves block - transparent, non-collidable.
pub struct LeavesBlock {
    pub block_index: i32,
}

impl LeavesBlock {
    pub const INDEX: i32 = 5;
    pub const NAME: &'static str = "LeavesBlock";

    pub fn new() -> Self {
        LeavesBlock { block_index: Self::INDEX }
    }
}

impl Block for LeavesBlock {
    fn block_index(&self) -> i32 { self.block_index }
    fn set_block_index(&mut self, index: i32) { self.block_index = index; }
    fn default_display_name(&self) -> &str { "Leaves" }
    fn default_description(&self) -> &str { "Leaves" }
    fn default_category(&self) -> &str { "Plants" }
    fn display_order(&self) -> i32 { 4 }
    fn is_collidable(&self) -> bool { false }
    fn is_placeable(&self) -> bool { true }
    fn is_transparent(&self) -> bool { true }
    fn light_attenuation(&self) -> i32 { 1 }
    fn default_emitted_light(&self) -> i32 { 0 }
    fn default_texture_slot(&self) -> i32 { 4 }
    fn default_shadow_strength(&self) -> i32 { 15 }
    fn is_fluid_blocker(&self) -> bool { false }
    fn is_gatherable(&self) -> bool { true }
    fn density(&self) -> f32 { 0.5 }
    fn fuel_heat_level(&self) -> f32 { 0.0 }
    fn fuel_fire_duration(&self) -> f32 { 0.0 }
    fn shovel_power(&self) -> f32 { 1.0 }
    fn quarry_power(&self) -> f32 { 1.0 }
    fn hack_power(&self) -> f32 { 1.0 }
    fn melee_power(&self) -> f32 { 1.0 }
    fn melee_hit_probability(&self) -> f32 { 0.66 }
    fn projectile_power(&self) -> f32 { 1.0 }
    fn dig_method(&self) -> BlockDigMethod { BlockDigMethod::Hack }
    fn dig_resilience(&self) -> f32 { 1.0 }
    fn max_stacking(&self) -> i32 { 40 }
    fn sleep_suitability(&self) -> f32 { 0.0 }
    fn friction_factor(&self) -> f32 { 1.0 }
    fn generate_faces_for_same_neighbors(&self) -> bool { true }
}

/// Water block - fluid, non-collidable.
pub struct WaterBlock {
    pub block_index: i32,
}

impl WaterBlock {
    pub const INDEX: i32 = 6;
    pub const NAME: &'static str = "WaterBlock";

    pub fn new() -> Self {
        WaterBlock { block_index: Self::INDEX }
    }
}

impl Block for WaterBlock {
    fn block_index(&self) -> i32 { self.block_index }
    fn set_block_index(&mut self, index: i32) { self.block_index = index; }
    fn default_display_name(&self) -> &str { "Water" }
    fn default_description(&self) -> &str { "Water" }
    fn default_category(&self) -> &str { "Terrain" }
    fn display_order(&self) -> i32 { 5 }
    fn is_collidable(&self) -> bool { false }
    fn is_placeable(&self) -> bool { false }
    fn is_transparent(&self) -> bool { true }
    fn light_attenuation(&self) -> i32 { 0 }
    fn default_emitted_light(&self) -> i32 { 0 }
    fn default_texture_slot(&self) -> i32 { 5 }
    fn default_shadow_strength(&self) -> i32 { 0 }
    fn is_fluid_blocker(&self) -> bool { false }
    fn is_gatherable(&self) -> bool { false }
    fn density(&self) -> f32 { 1.0 }
    fn fuel_heat_level(&self) -> f32 { 0.0 }
    fn fuel_fire_duration(&self) -> f32 { 0.0 }
    fn shovel_power(&self) -> f32 { 0.0 }
    fn quarry_power(&self) -> f32 { 0.0 }
    fn hack_power(&self) -> f32 { 0.0 }
    fn melee_power(&self) -> f32 { 0.0 }
    fn melee_hit_probability(&self) -> f32 { 0.0 }
    fn projectile_power(&self) -> f32 { 0.0 }
    fn dig_method(&self) -> BlockDigMethod { BlockDigMethod::Drill }
    fn dig_resilience(&self) -> f32 { 0.0 }
    fn max_stacking(&self) -> i32 { 0 }
    fn sleep_suitability(&self) -> f32 { 0.0 }
    fn friction_factor(&self) -> f32 { 1.0 }
    fn is_index_dynamic(&self) -> bool { false }
}

/// Grass block - top soil with grass color.
pub struct GrassBlock {
    pub block_index: i32,
}

impl GrassBlock {
    pub const INDEX: i32 = 8;
    pub const NAME: &'static str = "GrassBlock";

    pub fn new() -> Self {
        GrassBlock { block_index: Self::INDEX }
    }
}

impl Block for GrassBlock {
    fn block_index(&self) -> i32 { self.block_index }
    fn set_block_index(&mut self, index: i32) { self.block_index = index; }
    fn default_display_name(&self) -> &str { "Grass" }
    fn default_description(&self) -> &str { "Grass" }
    fn default_category(&self) -> &str { "Terrain" }
    fn display_order(&self) -> i32 { 6 }
    fn is_collidable(&self) -> bool { true }
    fn is_placeable(&self) -> bool { true }
    fn is_transparent(&self) -> bool { false }
    fn light_attenuation(&self) -> i32 { 1 }
    fn default_emitted_light(&self) -> i32 { 0 }
    fn default_texture_slot(&self) -> i32 { 6 }
    fn default_shadow_strength(&self) -> i32 { 15 }
    fn is_fluid_blocker(&self) -> bool { true }
    fn is_gatherable(&self) -> bool { true }
    fn density(&self) -> f32 { 1.0 }
    fn fuel_heat_level(&self) -> f32 { 0.0 }
    fn fuel_fire_duration(&self) -> f32 { 0.0 }
    fn shovel_power(&self) -> f32 { 1.0 }
    fn quarry_power(&self) -> f32 { 1.0 }
    fn hack_power(&self) -> f32 { 1.0 }
    fn melee_power(&self) -> f32 { 1.0 }
    fn melee_hit_probability(&self) -> f32 { 0.66 }
    fn projectile_power(&self) -> f32 { 1.0 }
    fn dig_method(&self) -> BlockDigMethod { BlockDigMethod::Shovel }
    fn dig_resilience(&self) -> f32 { 1.0 }
    fn max_stacking(&self) -> i32 { 40 }
    fn sleep_suitability(&self) -> f32 { 0.0 }
    fn friction_factor(&self) -> f32 { 1.0 }
    fn is_suitable_for_plants(&self, _value: i32, _plant_value: i32) -> bool { true }
}