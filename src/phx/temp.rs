use super::{Hitbox, PhysicsWorld, Velocity};
use crate::util::{input::Button, ButtonsState};
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

pub struct PlayerControlled {}

/// `accel` is a number from 0 to 1, 1 for instant max speed
// FRAME_DEPENDANT
#[system(for_each)]
pub fn left_right(
    #[state] target_speed: &mut f32,
    #[state] accel: &mut f32,
    #[resource] inputs: &ButtonsState,
    vel: &mut Velocity,
    _pc: &PlayerControlled,
) {
    let accel = *accel;
    let target_speed = {
        let dir =
            (inputs.is_pressed(Button::Right) as i8) - (inputs.is_pressed(Button::Left) as i8);
        *target_speed * dir as f32
    };

    vel.src
        .set_x(accel * target_speed + (1. - accel) * vel.src.x());
    if vel.src.x().abs() < 1. {
        vel.src.set_x(0.);
    }
    if inputs.pressed(Button::Jump) {
        vel.src.set_y(-156.);
    }
}

// FRAME DEPENDANT possible change
// modify all "multipliers" that accumulate over multiple frames to follow this model:
// the same as the "pow" version of frame-indepentant variant but
// speed *= exp2(frictionRate * deltaTime)
// frictionRate = log2(friction)/originalFixedDeltaTime
