//! Entity identifiers.

/// Identifier of an entity in the world.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct EntityId(pub u32);

impl EntityId {
    /// Invalid/unset entity handle.
    pub const INVALID: EntityId = EntityId(0);

    pub const fn is_valid(&self) -> bool {
        self.0 != 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn invalid_entity_is_zero() {
        assert_eq!(EntityId::INVALID, EntityId(0));
        assert!(!EntityId::INVALID.is_valid());
        assert!(EntityId(1).is_valid());
    }
}
