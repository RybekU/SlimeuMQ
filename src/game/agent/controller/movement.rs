use hecs::{Entity, World};

use crate::{
    game::resources::Resources,
    gfx::{Animation, Sprite},
    phx::{OnGround, Velocity},
    util::{input::Button, lerp, ButtonsState},
    FRAMETIME,
};

fn handle_jump(inputs: &ButtonsState, vel: &mut Velocity) {
    if inputs.pressed(Button::Jump) {
        vel.src.y = -156.;
    }
}

pub fn idle_on_enter(entity: Entity, world: &World, _resources: &mut Resources) {
    log::info!("Player standing v2");
    let mut animation = world.get_mut::<Animation>(entity).unwrap();

    animation.change("slimeu_idle");
}

// TODO: clean up this turbo trash code
pub fn idle_on_update(entity: Entity, world: &World, resources: &mut Resources) {
    const DECEL: f32 = 20.0;

    let inputs = &resources.input_buttons;

    let mut query_o = world.query_one::<(&OnGround, &mut Velocity)>(entity).unwrap();
    let (_on_ground, vel) = query_o.get().unwrap();

    // let on_ground = on_ground.on_ground;

    handle_jump(inputs, vel);

    // decelerate quickly when no input is given
    vel.src.x = lerp(0., vel.src.x, f32::exp2(-DECEL * FRAMETIME));

    if vel.src.x.abs() < 1. {
        vel.src.x = 0.;
    }
}

pub fn move_directional(_entity: Entity, _world: &World, resources: &Resources) -> bool {
    let inputs = &resources.input_buttons;

    inputs.is_pressed(Button::Right) || inputs.is_pressed(Button::Left)
}

pub fn run_on_enter(entity: Entity, world: &World, _resources: &mut Resources) {
    log::info!("Player walking v2");
    let mut animation = world.get_mut::<Animation>(entity).unwrap();
    animation.change("slimeu_run");
}

pub fn run_on_update(entity: Entity, world: &World, resources: &mut Resources) {
    const TARGET_SPEED: f32 = 64.;
    const ACCEL: f32 = 10.0;

    let inputs = &resources.input_buttons;

    let mut query_o = world.query_one::<(&mut Velocity, &OnGround, &mut Sprite)>(entity).unwrap();
    let (vel, _on_ground, sprite) = query_o.get().unwrap();

    let flipped = &mut sprite.flip;
    // let on_ground = on_ground.on_ground;

    handle_jump(inputs, vel);

    let target_speed = {
        let dir =
            (inputs.is_pressed(Button::Right) as i8) - (inputs.is_pressed(Button::Left) as i8);
        TARGET_SPEED * dir as f32
    };

    // flip sprite if necessary
    *flipped = match (inputs.is_pressed(Button::Left), inputs.is_pressed(Button::Right)) {
        (true, false) => true,
        (false, true) => false,
        _ => *flipped,
    };

    vel.src.x = lerp(target_speed, vel.src.x, f32::exp2(-ACCEL * FRAMETIME));

    if vel.src.x.abs() < 1. {
        vel.src.x = 0.;
    };
}
