//! Block identifiers and static block behavior.

/// Identifier of a block type in the world.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct BlockId(pub u16);

impl BlockId {
    /// Air (empty) block.
    pub const AIR: BlockId = BlockId(0);

    /// Returns `true` if this is the air block.
    pub const fn is_air(&self) -> bool {
        self.0 == 0
    }
}

/// Static behavior of a block type.
pub trait Block: Send + Sync {
    /// Block identifier.
    fn id(&self) -> BlockId;

    /// Whether the block is solid for collision and rendering.
    fn is_solid(&self) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn air_id_is_zero() {
        assert_eq!(BlockId::AIR, BlockId(0));
        assert!(BlockId::AIR.is_air());
        assert!(!BlockId(1).is_air());
    }
}
