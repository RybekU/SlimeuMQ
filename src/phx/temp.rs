use hecs::World;

use super::{Hitbox, PhysicsWorld, Velocity};

pub fn reset_velocity_system(world: &mut World, phys_world: &PhysicsWorld) {
    for (_eid, (hitbox, vel)) in world.query_mut::<(&Hitbox, &mut Velocity)>() {
        reset_velocity(phys_world, hitbox, vel);
    }
}

// TODO: implement as feature in resphys
// TODO: make sure to reset velocity only if normal and direction match
pub fn reset_velocity(phys_world: &PhysicsWorld, hitbox: &Hitbox, vel: &mut Velocity) {
    for (_, info) in phys_world.collisions_of(hitbox.src) {
        if info.normal.x() != 0. {
            vel.src.set_x(0.);
        } else {
            vel.src.set_y(0.);
        }
    }
}
