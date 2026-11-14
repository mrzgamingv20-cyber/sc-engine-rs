/// Block trait - core properties used by generator and mesher.
pub trait Block: Send + Sync {
    /// Block index (id = index in registry).
    fn block_index(&self) -> i32;
    /// Set block index.
    fn set_block_index(&mut self, index: i32);
    /// Default display name.
    fn default_display_name(&self) -> &str;
    /// Default description.
    fn default_description(&self) -> &str;
    /// Default category.
    fn default_category(&self) -> &str;
    /// Display order.
    fn display_order(&self) -> i32;
    /// Is collidable.
    fn is_collidable(&self) -> bool;
    /// Is placeable.
    fn is_placeable(&self) -> bool;
    /// Is transparent.
    fn is_transparent(&self) -> bool;
    /// Light attenuation.
    fn light_attenuation(&self) -> i32;
    /// Default emitted light amount.
    fn default_emitted_light(&self) -> i32;
    /// Default texture slot.
    fn default_texture_slot(&self) -> i32;
    /// Default shadow strength.
    fn default_shadow_strength(&self) -> i32;
    /// Is fluid blocker.
    fn is_fluid_blocker(&self) -> bool;
    /// Is gatherable.
    fn is_gatherable(&self) -> bool;
    /// Density.
    fn density(&self) -> f32;
    /// Fuel heat level.
    fn fuel_heat_level(&self) -> f32;
    /// Fuel fire duration.
    fn fuel_fire_duration(&self) -> f32;
    /// Shovel power.
    fn shovel_power(&self) -> f32;
    /// Quarry power.
    fn quarry_power(&self) -> f32;
    /// Hack power.
    fn hack_power(&self) -> f32;
    /// Melee power.
    fn melee_power(&self) -> f32;
    /// Melee hit probability.
    fn melee_hit_probability(&self) -> f32;
    /// Projectile power.
    fn projectile_power(&self) -> f32;
    /// Dig method.
    fn dig_method(&self) -> BlockDigMethod;
    /// Dig resilience.
    fn dig_resilience(&self) -> f32;
    /// Max stacking.
    fn max_stacking(&self) -> i32;
    /// Sleep suitability.
    fn sleep_suitability(&self) -> f32;
    /// Friction factor.
    fn friction_factor(&self) -> f32;
    /// Is collidable at value.
    fn is_collidable_at(&self, value: i32) -> bool { self.is_collidable() }
    /// Is transparent at value.
    fn is_transparent_at(&self, value: i32) -> bool { self.is_transparent() }
    /// Get texture slot for face.
    fn get_face_texture_slot(&self, face: i32, value: i32) -> i32 { self.default_texture_slot() }
    /// Get texture slot count.
    fn get_texture_slot_count(&self, value: i32) -> i32 { 16 }
    /// Get creative values.
    fn get_creative_values(&self) -> Vec<i32> { Vec::new() }
    /// Generate faces for same neighbors.
    fn generate_faces_for_same_neighbors(&self) -> bool { false }

    /// Is suitable for plants.
    fn is_suitable_for_plants(&self, _value: i32, _plant_value: i32) -> bool { false }
    /// Get food type.
    fn food_type(&self) -> FoodType { FoodType::None }
    /// Get sound material name.
    fn sound_material_name(&self) -> &str { "" }
    /// Is index dynamic (can be remapped).
    fn is_index_dynamic(&self) -> bool { true }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum BlockDigMethod {
    Drill = 0,
    Shovel = 1,
    Quarry = 2,
    Hack = 3,
    Chisel = 4,
    Axe = 5,
    Pickaxe = 6,
    Sword = 7,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum FoodType {
    None = 0,
    Meat = 1,
    Plant = 2,
    Fruit = 3,
    Cooked = 4,
    Raw = 5,
}

/// Block data for CSV loading.
#[derive(Debug, Clone)]
pub struct BlockAllocateData {
    pub block: Box<dyn Block>,
    pub index: i32,
    pub allocated: bool,
    pub static_block_index: bool,
    pub mod_entity: Option<usize>, // ModEntity reference
}

/// BlocksManager - registry for all blocks.
pub struct BlocksManager {
    pub blocks: Vec<Option<Box<dyn Block>>>,
    pub fluid_blocks: Vec<Option<Box<dyn Block>>>,
    pub categories: Vec<String>,
    pub block_name_to_index: std::collections::HashMap<String, i32>,
    pub block_type_to_index: std::collections::HashMap<String, i32>,
    pub block_type_to_original_index: std::collections::HashMap<String, i32>,
}

impl BlocksManager {
    pub const BLOCK_COUNT: usize = 1024;
    pub const SURVIVALCRAFT_BLOCK_COUNT: usize = 299;

    pub fn new() -> Self {
        BlocksManager {
            blocks: vec![None; Self::BLOCK_COUNT],
            fluid_blocks: vec![None; Self::BLOCK_COUNT],
            categories: Vec::new(),
            block_name_to_index: std::collections::HashMap::new(),
            block_type_to_index: std::collections::HashMap::new(),
            block_type_to_original_index: std::collections::HashMap::new(),
        }
    }

    pub fn blocks(&self) -> &[Option<Box<dyn Block>>] {
        &self.blocks
    }

    pub fn initialize_categories(&mut self) {
        self.categories.clear();
        self.categories.push("Terrain".to_string());
        self.categories.push("Minerals".to_string());
        self.categories.push("Plants".to_string());
        self.categories.push("Construction".to_string());
        self.categories.push("Items".to_string());
        self.categories.push("Tools".to_string());
        self.categories.push("Weapons".to_string());
        self.categories.push("Clothes".to_string());
        self.categories.push("Electrics".to_string());
        self.categories.push("Food".to_string());
        self.categories.push("Spawner Eggs".to_string());
        self.categories.push("Painted".to_string());
        self.categories.push("Dyed".to_string());
        self.categories.push("Fireworks".to_string());
    }

    pub fn get_block_index(&self, block_name: &str) -> Option<i32> {
        self.block_name_to_index.get(block_name).copied()
    }

    pub fn get_block(&self, block_name: &str) -> Option<&dyn Block> {
        self.block_name_to_index
            .get(block_name)
            .and_then(|&idx| self.blocks[idx as usize].as_ref().map(|b| b.as_ref()))
    }

    pub fn get_block_by_type(&self, type_name: &str) -> Option<&dyn Block> {
        self.block_type_to_index
            .get(type_name)
            .and_then(|&idx| self.blocks[idx as usize].as_ref().map(|b| b.as_ref()))
    }
}

impl Default for BlocksManager {
    fn default() -> Self {
        Self::new()
    }
}
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

impl Block for AirBlock {
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
    fn dig_method(&self) -> BlockDigMethod { BlockDigMethod::Drill }
    fn dig_resilience(&self) -> f32 { 0.0 }
    fn max_stacking(&self) -> i32 { 0 }
    fn sleep_suitability(&self) -> f32 { 0.0 }
    fn friction_factor(&self) -> f32 { 1.0 }
    fn is_index_dynamic(&self) -> bool { false }
    fn get_creative_values(&self) -> Vec<i32> { Vec::new() }

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
