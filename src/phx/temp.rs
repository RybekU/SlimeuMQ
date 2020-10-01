use super::{Hitbox, PhysicsWorld, Velocity};
use legion::system;

#[system(for_each)]
pub fn reset_velocity(#[resource] phys_world: &PhysicsWorld, hitbox: &Hitbox, vel: &mut Velocity) {
    for (_, info) in phys_world.collisions_of(hitbox.src) {
        if info.normal.x() != 0. {
            vel.src.set_x(0.);
        } else {
            vel.src.set_y(0.);
        }
    }
}
