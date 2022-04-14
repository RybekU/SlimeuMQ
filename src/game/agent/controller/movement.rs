use hecs::{Entity, World};

use crate::{
    game::resources::Resources,
    gfx::{Animation, Sprite},
    phx::Velocity,
    util::{input::Button, lerp},
    FRAMETIME,
};

// TODO: Change the prints into debug-mode console/entity-state info

// TRANSITIONS
pub fn move_directional(_entity: Entity, _world: &World, resources: &Resources) -> bool {
    let inputs = &resources.input_buttons;

    inputs.is_pressed(Button::Right) || inputs.is_pressed(Button::Left)
}

// HELPERS
pub fn handle_movement(
    entity: Entity,
    world: &World,
    resources: &mut Resources,
    target_speed: f32,
    acceleration: f32,
) {
    let inputs = &resources.input_buttons;

    let mut query = world.query_one::<(&mut Velocity, &mut Sprite)>(entity).unwrap();
    let (vel, sprite) = query.get().unwrap();

    let target_speed = {
        let dir =
            (inputs.is_pressed(Button::Right) as i8) - (inputs.is_pressed(Button::Left) as i8);
        target_speed * dir as f32
    };

    vel.src.x = lerp(target_speed, vel.src.x, f32::exp2(-acceleration * FRAMETIME));

    sprite.face_left = match (inputs.is_pressed(Button::Left), inputs.is_pressed(Button::Right)) {
        (true, false) => true,
        (false, true) => false,
        _ => sprite.face_left,
    };
}

// STATES
pub fn idle_on_enter(entity: Entity, world: &World, _resources: &mut Resources) {
    log::info!("Player standing v2");
    let mut animation = world.get_mut::<Animation>(entity).unwrap();
    animation.change("slimeu_idle");
}

pub fn idle_on_update(entity: Entity, world: &World, _resources: &mut Resources) {
    const DECEL: f32 = 20.0;

    let mut vel = world.get_mut::<Velocity>(entity).unwrap();

    // decelerate quickly when no input is given
    vel.src.x = lerp(0., vel.src.x, f32::exp2(-DECEL * FRAMETIME));

    if vel.src.x.abs() < 1. {
        vel.src.x = 0.;
    }
}

pub fn run_on_enter(entity: Entity, world: &World, _resources: &mut Resources) {
    log::info!("Player walking v2");
    let mut animation = world.get_mut::<Animation>(entity).unwrap();
    animation.change("slimeu_run");
}

pub fn run_on_update(entity: Entity, world: &World, resources: &mut Resources) {
    const TARGET_SPEED: f32 = 64.;
    const ACCEL: f32 = 10.0;

    handle_movement(entity, world, resources, TARGET_SPEED, ACCEL);
}
