//! Survivalcraft game logic (Rust port).
//!
//! This crate is pure Rust: no Android, C++, or assembly dependency. The
//! Android platform layer lives in `crates/engine-app`, the renderer in `cpp/`,
//! and the hot kernels in `asm/`.

#![forbid(unsafe_code)]

pub mod block;
pub mod entity;
pub mod game;
pub mod subsystem;
pub mod world;
pub mod entity_system;

pub use block::{Block, BlockId};
pub use entity::EntityId;
pub use game::Game;
pub use subsystem::Subsystem;
pub use world::{ChunkPos, World};
pub use entity_system::*;
