/// Chunk coordinate in the world (16x16 block columns).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ChunkPos {
    pub x: i32,
    pub z: i32,
}

impl ChunkPos {
    pub const fn new(x: i32, z: i32) -> Self {
        Self { x, z }
    }

    pub const fn offset(self, dx: i32, dz: i32) -> Self {
        Self {
            x: self.x + dx,
            z: self.z + dz,
        }
    }
}

/// World instance: seed plus generated state.
pub struct World {
    seed: u64,
}

impl World {
    pub fn new(seed: u64) -> Self {
        Self { seed }
    }

    pub fn seed(&self) -> u64 {
        self.seed
    }
}
