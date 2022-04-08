use hecs::{Entity, World};

use crate::{
    game::resources::Resources,
    gfx::Sprite,
    phx::{OnGround, Velocity},
    util::{input::Button, lerp},
    FRAMETIME,
};

// TRANSITIONS
pub fn jump(_entity: Entity, _world: &World, resources: &Resources) -> bool {
    let inputs = &resources.input_buttons;

    inputs.pressed(Button::Jump)
}

pub fn land(entity: Entity, world: &World, _resources: &Resources) -> bool {
    let on_ground = world.get::<OnGround>(entity).unwrap();

    on_ground.on_ground
}

// STATES
pub fn jump_on_enter(entity: Entity, world: &World, _resources: &mut Resources) {
    log::info!("Player jumping v2");
    let mut vel = world.get_mut::<Velocity>(entity).unwrap();

    vel.src.y = -156.;
}

pub fn jump_on_update(entity: Entity, world: &World, resources: &mut Resources) {
    // TODO: Variable jump height
    const TARGET_SPEED: f32 = 64.;
    const ACCEL: f32 = 5.0;

    let inputs = &resources.input_buttons;

    let mut query_o = world.query_one::<(&mut Velocity, &mut Sprite)>(entity).unwrap();
    let (vel, sprite) = query_o.get().unwrap();

    // flip sprite if necessary
    let flipped = &mut sprite.flip;
    *flipped = match (inputs.is_pressed(Button::Left), inputs.is_pressed(Button::Right)) {
        (true, false) => true,
        (false, true) => false,
        _ => *flipped,
    };

    let target_speed = {
        let dir =
            (inputs.is_pressed(Button::Right) as i8) - (inputs.is_pressed(Button::Left) as i8);
        TARGET_SPEED * dir as f32
    };

    vel.src.x = lerp(target_speed, vel.src.x, f32::exp2(-ACCEL * FRAMETIME));

    if vel.src.x.abs() < 1. {
        vel.src.x = 0.;
    }
}
