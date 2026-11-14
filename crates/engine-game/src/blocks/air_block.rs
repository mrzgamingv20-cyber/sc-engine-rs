use super::block::{Block, BlockDigMethod};
/// Air block - empty/non-solid.
pub struct AirBlock {
    pub block_index: i32,
}

impl AirBlock {
    pub const INDEX: i32 = 0;
    pub const NAME: &'static str = "AirBlock";

    pub fn new() -> Self {
        AirBlock { block_index: Self::INDEX }
    }
}

impl super::block::Block for AirBlock {
    fn block_index(&self) -> i32 { self.block_index }
    fn set_block_index(&mut self, index: i32) { self.block_index = index; }
    fn default_display_name(&self) -> &str { "Air" }
    fn default_description(&self) -> &str { "Air" }
    fn default_category(&self) -> &str { "Terrain" }
    fn display_order(&self) -> i32 { 0 }
    fn is_collidable(&self) -> bool { false }
    fn is_placeable(&self) -> bool { false }
    fn is_transparent(&self) -> bool { true }
    fn light_attenuation(&self) -> i32 { 0 }
    fn default_emitted_light(&self) -> i32 { 0 }
    fn default_texture_slot(&self) -> i32 { 0 }
    fn default_shadow_strength(&self) -> i32 { 0 }
    fn is_fluid_blocker(&self) -> bool { false }
    fn is_gatherable(&self) -> bool { false }
    fn density(&self) -> f32 { 0.0 }
    fn fuel_heat_level(&self) -> f32 { 0.0 }
    fn fuel_fire_duration(&self) -> f32 { 0.0 }
    fn shovel_power(&self) -> f32 { 0.0 }
    fn quarry_power(&self) -> f32 { 0.0 }
    fn hack_power(&self) -> f32 { 0.0 }
    fn melee_power(&self) -> f32 { 0.0 }
    fn melee_hit_probability(&self) -> f32 { 0.0 }
    fn projectile_power(&self) -> f32 { 0.0 }
    fn dig_method(&self) -> super::block::BlockDigMethod { super::block::BlockDigMethod::Drill }
    fn dig_resilience(&self) -> f32 { 0.0 }
    fn max_stacking(&self) -> i32 { 0 }
    fn sleep_suitability(&self) -> f32 { 0.0 }
    fn friction_factor(&self) -> f32 { 1.0 }
    fn is_index_dynamic(&self) -> bool { false }
    fn get_creative_values(&self) -> Vec<i32> { Vec::new() }
}