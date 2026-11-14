// Use types from the parent terrain module
use super::terrain::{Point2, Point3, Vector2, Vector3, BoundingBox, TerrainId, TerrainBrush, TerrainChunkGeometry, TerrainGeometry};

// Terrain chunk constants.
pub const CHUNK_SIZE_BITS: u8 = 4;
pub const CHUNK_SIZE: usize = 16;
pub const CHUNK_SIZE_MINUS_ONE: usize = 15;
pub const CHUNK_HEIGHT_BITS: u8 = 8;
pub const CHUNK_HEIGHT: usize = 256;
pub const CHUNK_HEIGHT_MINUS_ONE: usize = 255;
pub const SLICE_HEIGHT: usize = 16;
pub const SLICES_COUNT: usize = 16;

/// Terrain cell (chunk-relative coordinates).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CellPos {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl CellPos {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        CellPos { x, y, z }
    }
}

/// Terrain chunk state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum TerrainChunkState {
    NotLoaded = 0,
    InvalidContents1 = 1,
    InvalidContents2 = 2,
    InvalidContents3 = 3,
    InvalidContents4 = 4,
    InvalidLight = 5,
    InvalidPropagatedLight = 6,
    InvalidVertices1 = 7,
    InvalidVertices2 = 8,
    Valid = 9,
}

/// Terrain chunk geometry buffer.
#[derive(Debug, Clone, Default)]
pub struct TerrainChunkGeometry {
    // Placeholder for geometry data
}

/// Terrain chunk.
pub struct TerrainChunk {
    /// Terrain this chunk belongs to.
    pub terrain: TerrainId,
    /// Chunk coordinates.
    pub coords: Point2,
    /// Origin in world space.
    pub origin: Point2,
    /// Bounding box.
    pub bounding_box: BoundingBox,
    /// Center in world space.
    pub center: Vector2,
    /// Current state.
    pub state: TerrainChunkState,
    /// Thread state.
    pub thread_state: TerrainChunkState,
    /// Was downgraded.
    pub was_downgraded: bool,
    /// Downgraded state.
    pub downgraded_state: Option<TerrainChunkState>,
    /// Was upgraded.
    pub was_upgraded: bool,
    /// Upgraded state.
    pub upgraded_state: Option<TerrainChunkState>,
    /// Modification counter.
    pub modification_counter: i32,
    /// Haze ends.
    pub haze_ends: [f32; 4],
    /// Are behaviors notified.
    pub are_behaviors_notified: bool,
    /// Is loaded.
    pub is_loaded: bool,
    /// New geometry data.
    pub new_geometry_data: bool,
    /// Geometry data.
    pub geometry: TerrainChunkGeometry,
    /// Cell values (65536 ints).
    pub cells: Vec<i32>,
    /// Shaft values (256 ints).
    pub shafts: Vec<i32>,
    /// Brush paints.
    pub brush_paints: Vec<BrushPaint>,
    /// Slice geometry data.
    pub chunk_slice_geometries: [TerrainChunkGeometry; SLICES_COUNT],
    /// Buffers.
    pub buffers: Vec<TerrainChunkGeometry>,
    /// Slice contents hashes.
    pub slice_contents_hashes: [i32; SLICES_COUNT],
    /// Generated slice contents hashes.
    pub generated_slice_contents_hashes: [i32; SLICES_COUNT],
}

impl TerrainChunk {
    /// Create a new terrain chunk.
    pub fn new(terrain_id: TerrainId, x: i32, z: i32) -> Self {
        let coords = Point2::new(x, z);
        let origin = Point2::new(x * CHUNK_SIZE as i32, z * CHUNK_SIZE as i32);
        let bounding_box = BoundingBox::new(
            Vector3::new(origin.x as f32, 0.0, origin.y as f32),
            Vector3::new(origin.x as f32 + CHUNK_SIZE as f32, CHUNK_HEIGHT as f32, origin.y as f32 + CHUNK_SIZE as f32),
        );
        let center = Vector2::new(
            origin.x as f32 + CHUNK_SIZE as f32 / 2.0,
            origin.y as f32 + CHUNK_SIZE as f32 / 2.0,
        );
        TerrainChunk {
            terrain: terrain_id,
            coords,
            origin,
            bounding_box,
            center,
            state: TerrainChunkState::NotLoaded,
            thread_state: TerrainChunkState::NotLoaded,
            was_downgraded: false,
            downgraded_state: None,
            was_upgraded: false,
            upgraded_state: None,
            modification_counter: 0,
            haze_ends: [0.0; 4],
            are_behaviors_notified: false,
            is_loaded: false,
            new_geometry_data: false,
            geometry: TerrainChunkGeometry::default(),
            cells: vec![0; CHUNK_SIZE * CHUNK_SIZE * CHUNK_HEIGHT],
            shafts: vec![0; CHUNK_SIZE * CHUNK_SIZE],
            brush_paints: Vec::new(),
            chunk_slice_geometries: [TerrainChunkGeometry::default(); SLICES_COUNT],
            buffers: Vec::new(),
            slice_contents_hashes: [0; SLICES_COUNT],
            generated_slice_contents_hashes: [0; SLICES_COUNT],
        }
    }

    /// Check if a cell is valid.
    pub fn is_cell_valid(x: i32, y: i32, z: i32) -> bool {
        x >= 0 && x < CHUNK_SIZE as i32
            && y >= 0 && y < CHUNK_HEIGHT as i32
            && z >= 0 && z < CHUNK_SIZE as i32
    }

    /// Check if a shaft is valid.
    pub fn is_shaft_valid(x: i32, z: i32) -> bool {
        x >= 0 && x < CHUNK_SIZE as i32
            && z >= 0 && z < CHUNK_SIZE as i32
    }

    /// Calculate cell index.
    pub fn calculate_cell_index(x: i32, y: i32, z: i32) -> i32 {
        if y >= 0 && y < CHUNK_HEIGHT as i32 {
            return y | (x << CHUNK_HEIGHT_BITS) | (z << 12);
        }
        let abs_y = y.abs();
        let y_upper_bits = (abs_y >> CHUNK_HEIGHT_BITS) as i32;
        if y_upper_bits > 0x7FFF {
            panic!("Height is too large.");
        }
        let y_lower_8_bits = (abs_y & 0xFF) as i32;
        let y_upper_bits = y_upper_bits & 0x7FFF;
        ((if y < 0 { 1 } else { 0 }) << 31) | (y_upper_bits << 16) | (z << 12) | (x << CHUNK_HEIGHT_BITS) | y_lower_8_bits
    }

    /// Get cell value fast.
    pub fn get_cell_value_fast(&self, x: i32, y: i32, z: i32) -> i32 {
        self.cells[(y + x * CHUNK_HEIGHT as i32 + z * CHUNK_HEIGHT as i32 * CHUNK_SIZE as i32) as usize]
    }

    /// Set cell value fast.
    pub fn set_cell_value_fast(&mut self, x: i32, y: i32, z: i32, value: i32) {
        self.cells[(y + x * CHUNK_HEIGHT as i32 + z * CHUNK_HEIGHT as i32 * CHUNK_SIZE as i32) as usize] = value;
    }

    /// Get shaft value fast.
    pub fn get_shaft_value_fast(&self, x: i32, z: i32) -> i32 {
        self.shafts[(x + z * CHUNK_SIZE as i32) as usize]
    }

    /// Set shaft value fast.
    pub fn set_shaft_value_fast(&mut self, x: i32, z: i32, value: i32) {
        self.shafts[(x + z * CHUNK_SIZE as i32) as usize] = value;
    }
}

/// Brush paint for terrain modifications.
#[derive(Debug, Clone)]
pub struct BrushPaint {
    pub position: Point3,
    pub brush: TerrainBrush,
}

/// Terrain brush placeholder.
#[derive(Debug, Clone)]
pub struct TerrainBrush;

impl TerrainBrush {
    pub fn paint_fast(&self, _chunk: &mut TerrainChunk, _x: i32, _y: i32, _z: i32) {}
}

/// Terrain chunk state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum TerrainChunkState {
    NotLoaded = 0,
    InvalidContents1 = 1,
    InvalidContents2 = 2,
    InvalidContents3 = 3,
    InvalidContents4 = 4,
    InvalidLight = 5,
    InvalidPropagatedLight = 6,
    InvalidVertices1 = 7,
    InvalidVertices2 = 8,
    Valid = 9,
}

/// Terrain brush paint.
#[derive(Debug, Clone)]
pub struct BrushPaint {
    pub position: Point3,
    pub brush: TerrainBrush,
}