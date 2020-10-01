use crate::phx::Velocity;
use glam::Vec2;
use legion::system;

#[derive(Debug)]
pub struct Gravity {
    enabled: bool,
    strength: Vec2,
}

impl Gravity {
    pub fn new(strength: Vec2) -> Self {
        Self {
            strength,
            enabled: true,
        }
    }
}

#[system(for_each)]
pub fn gravity(vel: &mut Velocity, gravity: &Gravity) {
    if gravity.enabled {
        vel.src += gravity.strength;
    }
}
