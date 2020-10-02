mod gravity;
mod hitbox;
pub mod temp;

use glam::Vec2;

pub type PhysicsWorld = resphys::PhysicsWorld<ColliderTag>;
pub type BodySet = resphys::BodySet;
pub type ColliderSet = resphys::ColliderSet<ColliderTag>;

pub use gravity::*;
pub use hitbox::*;
pub use temp::*;

#[derive(Debug, Clone, Copy)]
pub enum ColliderTag {
    Tile,
    Player,
    // Enemy,
}

#[derive(Debug, Clone, Copy)]
pub struct Position {
    pub src: Vec2,
}

#[derive(Debug, Clone, Copy)]
pub struct Velocity {
    pub src: Vec2,
}
