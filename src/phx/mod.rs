mod gravity;
mod hitbox;
pub mod temp;

use bitflags::bitflags;
use glam::Vec2;

pub type PhysicsWorld = resphys::PhysicsWorld<ColliderTag>;
pub type BodySet = resphys::BodySet;
pub type ColliderSet = resphys::ColliderSet<ColliderTag>;

pub type BodyEntityMap = fxhash::FxHashMap<resphys::BodyHandle, legion::Entity>;

pub use gravity::*;
pub use hitbox::*;
pub use temp::*;

#[derive(Debug, Clone, Copy)]
pub enum ColliderTag {
    Tile,
    Player,
    // Enemy,
}

bitflags! {
    pub struct Category: u32 {
        /// level geometry
        const GROUND = 0b1 << 1;
        /// player deserves his own category for interaction
        const PLAYER = 0b1 << 2;
        const ENEMY = 0b1 << 3;
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Position {
    pub src: Vec2,
}

#[derive(Debug, Clone, Copy)]
pub struct Velocity {
    pub src: Vec2,
}
