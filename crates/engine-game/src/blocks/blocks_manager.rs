use std::collections::HashMap;
use super::block::Block;

/// Blocks manager - registry for all blocks.
pub struct BlocksManager {
    pub blocks: Vec<Option<Box<dyn Block>>>,
    pub fluid_blocks: Vec<Option<Box<dyn Block>>>,
    pub categories: Vec<String>,
    pub block_name_to_index: HashMap<String, i32>,
    pub block_type_to_index: HashMap<String, i32>,
    pub block_type_to_original_index: HashMap<String, i32>,
}

impl BlocksManager {
    pub const BLOCK_COUNT: usize = 1024;
    pub const SURVIVALCRAFT_BLOCK_COUNT: usize = 299;

    pub fn new() -> Self {
        BlocksManager {
            blocks: (0..Self::BLOCK_COUNT).map(|_| None).collect(),
            fluid_blocks: (0..Self::BLOCK_COUNT).map(|_| None).collect(),
            categories: Vec::new(),
            block_name_to_index: HashMap::new(),
            block_type_to_index: HashMap::new(),
            block_type_to_original_index: HashMap::new(),
        }
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

    /// Parse CSV blocks data string.
    pub fn load_blocks_data(&mut self, blocks_data_string: &str) -> Result<(), String> {
        let cleaned = blocks_data_string.replace("\r", "");
        let lines: Vec<&str> = cleaned
            .split('\n')
            .filter(|s| !s.is_empty())
            .collect();

        if lines.is_empty() {
            return Ok(());
        }

        let first_line: Vec<&str> = lines[0].split(';').collect();
        let expected_columns = first_line.len();

        for line in &lines[1..] {
            if line.is_empty() {
                continue;
            }
            let array: Vec<&str> = line.split(';').collect();
            if array.len() != expected_columns {
                return Err(format!(
                    "Column count mismatch: expected {}, got {}",
                    expected_columns,
                    array.len()
                ));
            }

            let type_name = array[0];
            if type_name.is_empty() {
                continue;
            }

            // Check if block type exists
            let block = self.blocks.iter().find(|b| {
                b.as_ref()
                    .map(|b| b.default_display_name() == type_name)
                    .unwrap_or(false)
            });

            if block.is_none() {
                // Block type not found, skip
                continue;
            }

            // Process each field
            for j in 1..array.len() {
                let field_name = first_line[j];
                let data = array[j];
                if !data.is_empty() {
                    if data.starts_with('#') {
                        // Reference to another block by name
                        let ref_type_name = &data[1..];
                        if !ref_type_name.is_empty() {
                            // Find the block by name
                            if self.block_name_to_index.get(ref_type_name).is_none() {
                                return Err(format!(
                                    "Reference type {} not found",
                                    ref_type_name
                                ));
                            }
                        }
                    }
                }
            }
        }

        Ok(())
    }
}

impl Default for BlocksManager {
    fn default() -> Self {
        Self::new()
    }
}