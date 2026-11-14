use crate::world::terrain_chunk::*;

/// Terrain constants from C# Terrain.cs.
pub const CONTENTS_MASK: i32 = 1023;
pub const LIGHT_MASK: i32 = 15360;
pub const LIGHT_SHIFT: i32 = 10;
pub const DATA_MASK: i32 = -16384;
pub const DATA_SHIFT: i32 = 14;
pub const TOP_HEIGHT_MASK: i32 = 255;
pub const TOP_HEIGHT_SHIFT: i32 = 0;
pub const TEMPERATURE_MASK: i32 = 3840;
pub const TEMPERATURE_SHIFT: i32 = 8;
pub const HUMIDITY_MASK: i32 = 61440;
pub const HUMIDITY_SHIFT: i32 = 12;
pub const BOTTOM_HEIGHT_MASK: i32 = 16711680;
pub const BOTTOM_HEIGHT_SHIFT: i32 = 16;
pub const SUNLIGHT_HEIGHT_MASK: i32 = -16777216;
pub const SUNLIGHT_HEIGHT_SHIFT: i32 = 24;

/// Chunks storage with open addressing hash table.
pub struct ChunksStorage {
    pub array: Vec<Option<TerrainChunk>>,
    pub capacity: usize,
}

impl ChunksStorage {
    pub const SHIFT: u8 = 8;
    pub const CAPACITY: usize = 65536;
    pub const CAPACITY_MINUS_ONE: usize = 65535;

    pub fn new() -> Self {
        ChunksStorage {
            array: vec![None; Self::CAPACITY],
            capacity: Self::CAPACITY,
        }
    }

    /// Get chunk at coordinates.
    pub fn get(&self, x: i32, y: i32) -> Option<&TerrainChunk> {
        let mut num = ((x + (y << Self::SHIFT)) & Self::CAPACITY_MINUS_ONE as i32) as usize;
        loop {
            let chunk = &self.array[num];
            match chunk {
                None => return None,
                Some(c) if c.coords.x == x && c.coords.y == y => return Some(c),
                _ => {
                    num = (num + 1) & Self::CAPACITY_MINUS_ONE;
                }
            }
        }
    }

    /// Get chunk at coordinates (mutable).
    pub fn get_mut(&mut self, x: i32, y: i32) -> Option<&mut TerrainChunk> {
        let mut num = ((x + (y << Self::SHIFT)) & Self::CAPACITY_MINUS_ONE as i32) as usize;
        loop {
            match &self.array[num] {
                None => return None,
                Some(c) if c.coords.x == x && c.coords.y == y => {
                    return Some(self.array[num].as_mut().unwrap());
                }
                _ => {
                    num = (num + 1) & Self::CAPACITY_MINUS_ONE;
                }
            }
        }
    }

    /// Add chunk to storage.
    pub fn add(&mut self, x: i32, y: i32, chunk: TerrainChunk) {
        let mut num = ((x + (y << Self::SHIFT)) & Self::CAPACITY_MINUS_ONE as i32) as usize;
        while self.array[num].is_some() {
            num = (num + 1) & Self::CAPACITY_MINUS_ONE;
        }
        self.array[num] = Some(chunk);
    }

    /// Remove chunk from storage.
    pub fn remove(&mut self, x: i32, y: i32) {
        let mut num = ((x + (y << Self::SHIFT)) & Self::CAPACITY_MINUS_ONE as i32) as usize;
        loop {
            match &self.array[num] {
                None => return,
                Some(c) if c.coords.x == x && c.coords.y == y => {
                    self.array[num] = None;
                    return;
                }
                _ => {
                    num = (num + 1) & Self::CAPACITY_MINUS_ONE;
                }
            }
        }
    }
}

/// Terrain - the world container.
pub struct Terrain {
    /// All chunks storage.
    pub all_chunks: ChunksStorage,
    /// Allocated chunks set.
    pub allocated_chunks: Vec<TerrainChunk>,
    /// Season temperature.
    pub season_temperature: i32,
    /// Season humidity.
    pub season_humidity: i32,
}

impl Terrain {
    pub fn new() -> Self {
        Terrain {
            all_chunks: ChunksStorage::new(),
            allocated_chunks: Vec::new(),
            season_temperature: 0,
            season_humidity: 0,
        }
    }

    /// Get chunk at coordinates.
    pub fn get_chunk_at_coords(&self, chunk_x: i32, chunk_z: i32) -> Option<&TerrainChunk> {
        self.all_chunks.get(chunk_x, chunk_z)
    }

    /// Get chunk at coordinates (mutable).
    pub fn get_chunk_at_coords_mut(&mut self, chunk_x: i32, chunk_z: i32) -> Option<&mut TerrainChunk> {
        self.all_chunks.get_mut(chunk_x, chunk_z)
    }

    /// Get chunk at cell coordinates.
    pub fn get_chunk_at_cell(&self, x: i32, z: i32) -> Option<&TerrainChunk> {
        self.get_chunk_at_coords(x >> CHUNK_SIZE_BITS, z >> CHUNK_SIZE_BITS)
    }

    /// Get chunk at cell coordinates (mutable).
    pub fn get_chunk_at_cell_mut(&mut self, x: i32, z: i32) -> Option<&mut TerrainChunk> {
        self.get_chunk_at_coords_mut(x >> CHUNK_SIZE_BITS, z >> CHUNK_SIZE_BITS)
    }

    /// Allocate a new chunk.
    pub fn allocate_chunk(&mut self, chunk_x: i32, chunk_z: i32) -> &mut TerrainChunk {
        if self.get_chunk_at_coords(chunk_x, chunk_z).is_some() {
            panic!("Chunk already allocated.");
        }
        let mut chunk = TerrainChunk::new(TerrainId(0), chunk_x, chunk_z);
        self.allocated_chunks.push(chunk);
        // Return a mutable reference to the last pushed chunk
        // Note: this is a simplified version
        let idx = self.allocated_chunks.len() - 1;
        self.all_chunks.add(chunk_x, chunk_z, self.allocated_chunks[idx].clone());
        &mut self.allocated_chunks[idx]
    }

    /// Free a chunk.
    pub fn free_chunk(&mut self, chunk: &TerrainChunk) {
        self.allocated_chunks.retain(|c| c.coords.x != chunk.coords.x || c.coords.y != chunk.coords.y);
        self.all_chunks.remove(chunk.coords.x, chunk.coords.y);
    }

    /// Convert world position to chunk coordinates.
    pub fn to_chunk(p: &Point2) -> Point2 {
        Self::to_chunk_xy(p.x, p.y)
    }

    /// Convert world coordinates to chunk coordinates.
    pub fn to_chunk_xy(x: i32, z: i32) -> Point2 {
        Point2::new(x >> CHUNK_SIZE_BITS, z >> CHUNK_SIZE_BITS)
    }

    /// Convert world position to cell coordinates.
    pub fn to_cell_float(x: f32) -> i32 {
        x.floor() as i32
    }

    /// Convert world coordinates to cell coordinates.
    pub fn to_cell(x: f32, y: f32, z: f32) -> (i32, i32, i32) {
        (x.floor() as i32, y.floor() as i32, z.floor() as i32)
    }

    /// Check if a cell is valid.
    pub fn is_cell_valid(x: i32, y: i32, z: i32) -> bool {
        y >= 0 && y < CHUNK_HEIGHT as i32
    }

    /// Get cell value.
    pub fn get_cell_value(&self, x: i32, y: i32, z: i32) -> i32 {
        if !Self::is_cell_valid(x, y, z) {
            return 0;
        }
        match self.get_chunk_at_cell(x, z) {
            Some(chunk) => chunk.get_cell_value_fast(x & 0xF, y, z & 0xF),
            None => 0,
        }
    }

    /// Set cell value.
    pub fn set_cell_value_fast(&mut self, x: i32, y: i32, z: i32, value: i32) {
        if let Some(chunk) = self.get_chunk_at_cell_mut(x, z) {
            chunk.set_cell_value_fast(x & 0xF, y, z & 0xF, value);
        }
    }

    /// Get cell contents.
    pub fn get_cell_contents(&self, x: i32, y: i32, z: i32) -> i32 {
        Self::extract_contents(self.get_cell_value(x, y, z))
    }

    /// Get cell light.
    pub fn get_cell_light(&self, x: i32, y: i32, z: i32) -> i32 {
        Self::extract_light(self.get_cell_value(x, y, z))
    }

    /// Get shaft value.
    pub fn get_shaft_value(&self, x: i32, z: i32) -> i32 {
        match self.get_chunk_at_cell(x, z) {
            Some(chunk) => chunk.get_shaft_value_fast(x & 0xF, z & 0xF),
            None => 0,
        }
    }

    /// Set shaft value.
    pub fn set_shaft_value(&mut self, x: i32, z: i32, value: i32) {
        if let Some(chunk) = self.get_chunk_at_cell_mut(x, z) {
            chunk.set_shaft_value_fast(x & 0xF, z & 0xF, value);
        }
    }

    // Bit-field operations (from C# Terrain.cs)

    /// Make block value with contents only.
    pub fn make_block_value_contents(contents: i32) -> i32 {
        contents & CONTENTS_MASK
    }

    /// Make block value with contents, light, and data.
    pub fn make_block_value(contents: i32, light: i32, data: i32) -> i32 {
        (contents & CONTENTS_MASK) | ((light << LIGHT_SHIFT) & LIGHT_MASK) | ((data << DATA_SHIFT) & DATA_MASK)
    }

    /// Extract contents from cell value.
    pub fn extract_contents(value: i32) -> i32 {
        value & CONTENTS_MASK
    }

    /// Extract light from cell value.
    pub fn extract_light(value: i32) -> i32 {
        (value & LIGHT_MASK) >> LIGHT_SHIFT
    }

    /// Extract data from cell value.
    pub fn extract_data(value: i32) -> i32 {
        (value & DATA_MASK) >> DATA_SHIFT
    }

    /// Extract top height from shaft value.
    pub fn extract_top_height(value: i32) -> i32 {
        value & TOP_HEIGHT_MASK
    }

    /// Extract bottom height from shaft value.
    pub fn extract_bottom_height(value: i32) -> i32 {
        (value & BOTTOM_HEIGHT_MASK) >> BOTTOM_HEIGHT_SHIFT
    }

    /// Extract sunlight height from shaft value.
    pub fn extract_sunlight_height(value: i32) -> i32 {
        ((value & SUNLIGHT_HEIGHT_MASK as i32) as u32 >> SUNLIGHT_HEIGHT_SHIFT) as i32
    }

    /// Extract humidity from shaft value.
    pub fn extract_humidity(value: i32) -> i32 {
        (value & HUMIDITY_MASK) >> HUMIDITY_SHIFT
    }

    /// Extract temperature from shaft value.
    pub fn extract_temperature(value: i32) -> i32 {
        (value & TEMPERATURE_MASK) >> TEMPERATURE_SHIFT
    }

    /// Replace contents in cell value.
    pub fn replace_contents(value: i32, contents: i32) -> i32 {
        value ^ ((value ^ contents) & CONTENTS_MASK)
    }

    /// Replace light in cell value.
    pub fn replace_light(value: i32, light: i32) -> i32 {
        value ^ ((value ^ (light << LIGHT_SHIFT)) & LIGHT_MASK)
    }

    /// Replace data in cell value.
    pub fn replace_data(value: i32, data: i32) -> i32 {
        value ^ ((value ^ (data << DATA_SHIFT)) & DATA_MASK)
    }

    /// Replace top height in shaft value.
    pub fn replace_top_height(value: i32, top_height: i32) -> i32 {
        value ^ ((value ^ top_height) & TOP_HEIGHT_MASK)
    }

    /// Replace bottom height in shaft value.
    pub fn replace_bottom_height(value: i32, bottom_height: i32) -> i32 {
        value ^ ((value ^ (bottom_height << BOTTOM_HEIGHT_SHIFT)) & BOTTOM_HEIGHT_MASK)
    }

    /// Replace sunlight height in shaft value.
    pub fn replace_sunlight_height(value: i32, sunlight_height: i32) -> i32 {
        (value & 16777215) | (sunlight_height << SUNLIGHT_HEIGHT_SHIFT)
    }

    /// Replace humidity in shaft value.
    pub fn replace_humidity(value: i32, humidity: i32) -> i32 {
        value ^ ((value ^ (humidity << HUMIDITY_SHIFT)) & HUMIDITY_MASK)
    }

    /// Replace temperature in shaft value.
    pub fn replace_temperature(value: i32, temperature: i32) -> i32 {
        value ^ ((value ^ (temperature << TEMPERATURE_SHIFT)) & TEMPERATURE_MASK)
    }

    /// Compare points for sorting.
    pub fn compare_points(c1: Point2, c2: Point2) -> i32 {
        if c1.y != c2.y {
            return if c1.y <= c2.y { -1 } else { 1 };
        }
        if c1.x != c2.x {
            return if c1.x <= c2.x { -1 } else { 1 };
        }
        0
    }

    /// Get seasonal temperature.
    pub fn get_seasonal_temperature(&self, x: i32, z: i32) -> i32 {
        (self.get_temperature(x, z) + self.season_temperature).max(0)
    }

    /// Get seasonal humidity.
    pub fn get_seasonal_humidity(&self, x: i32, z: i32) -> i32 {
        (self.get_humidity(x, z) + self.season_humidity).max(0)
    }

    /// Get temperature from shaft value.
    pub fn get_temperature(&self, x: i32, z: i32) -> i32 {
        Self::extract_temperature(self.get_shaft_value(x, z))
    }

    /// Get humidity from shaft value.
    pub fn get_humidity(&self, x: i32, z: i32) -> i32 {
        Self::extract_humidity(self.get_shaft_value(x, z))
    }
}

/// Point2 type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Point2 {
    pub x: i32,
    pub y: i32,
}

impl Point2 {
    pub fn new(x: i32, y: i32) -> Self {
        Point2 { x, y }
    }
}

/// Vector2 type.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

impl Vector2 {
    pub fn new(x: f32, y: f32) -> Self {
        Vector2 { x, y }
    }
}

/// Vector3 type.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vector3 { x, y, z }
    }
}

/// BoundingBox type.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct BoundingBox {
    pub min: Vector3,
    pub max: Vector3,
}

impl BoundingBox {
    pub fn new(min: Vector3, max: Vector3) -> Self {
        BoundingBox { min, max }
    }
}

/// TerrainId type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TerrainId(pub u32);

/// Point3 type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Point3 {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Point3 {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Point3 { x, y, z }
    }
}

/// Terrain brush placeholder.
#[derive(Debug, Clone, Default)]
pub struct TerrainBrush;
impl TerrainBrush {
    pub fn paint_fast(&self, _chunk: &mut TerrainChunk, _x: i32, _y: i32, _z: i32) {}
}

/// Terrain geometry placeholder.
#[derive(Debug, Clone, Copy, Default)]
pub struct TerrainChunkGeometry {
    pub buffer: Vec<f32>,
}

/// Terrain geometry per slice.
#[derive(Debug, Clone, Default)]
pub struct TerrainGeometry {
    pub data: Vec<f32>,
}
