use hecs::{Entity, World};

use crate::{
    game::resources::Resources,
    gfx::Sprite,
    phx::{OnGround, Velocity},
    util::{input::Button, lerp},
    FRAMETIME,
};

// TODO: Put on entity's blackboard or something
pub struct JumpData {
    pub jump_force: f32,
}

// TRANSITIONS
pub fn jump(_entity: Entity, _world: &World, resources: &Resources) -> bool {
    let inputs = &resources.input_buttons;

    inputs.pressed(Button::Jump)
}

pub fn jump_held(_entity: Entity, _world: &World, resources: &Resources) -> bool {
    let inputs = &resources.input_buttons;

    inputs.is_pressed(Button::Jump)
}

pub fn descending(entity: Entity, world: &World, _resources: &Resources) -> bool {
    let vel = world.get::<Velocity>(entity).unwrap();

    vel.src.y > 0.0
}

pub fn land(entity: Entity, world: &World, _resources: &Resources) -> bool {
    let on_ground = world.get::<OnGround>(entity).unwrap();

    on_ground.on_ground
}

// HELPERS
fn variable_jump_height(entity: Entity, world: &World, _resources: &mut Resources) {
    // const TARGET_HEIGHT: f32 = 64.;
    // calculate target speed using default gravity...?
    const DECEL: f32 = 10.0;

    let mut vel = world.get_mut::<Velocity>(entity).unwrap();
    let mut jump_data = world.get_mut::<JumpData>(entity).unwrap();

    // consider it from another side: start with maximal jump height, and start lowering velocity if button isn't pressed?
    // kinda what this code does, but without the floating at the end

    // TRY: keep setting to JUMP_VELOCITY till jump time expires
    // after it expires lower it to some value and let it wear normally
    jump_data.jump_force = lerp(0., jump_data.jump_force, f32::exp2(-DECEL * FRAMETIME));

    if jump_data.jump_force > -8. {
        jump_data.jump_force = 0.
    }

    vel.src.y += jump_data.jump_force;

    vel.src.y = vel.src.y.max(-156.);

    // log::debug!("force: {} velocity: {}", jump_data.jump_force, vel.src.y);
}

// STATES
pub fn jump_on_enter(entity: Entity, world: &World, _resources: &mut Resources) {
    // const JUMP_HALFSIZE: f32 = -156.;
    const JUMP_HALFSIZE: f32 = -128.;

    log::info!("Player jumping v2");
    let mut vel = world.get_mut::<Velocity>(entity).unwrap();
    vel.src.y = JUMP_HALFSIZE;

    let mut jump_data = world.get_mut::<JumpData>(entity).unwrap();
    jump_data.jump_force = JUMP_HALFSIZE / 4.;
}

pub fn jump_on_update(entity: Entity, world: &World, resources: &mut Resources) {
    // TODO: Variable jump height
    const TARGET_SPEED: f32 = 64.;
    const ACCEL: f32 = 5.0;

    variable_jump_height(entity, world, resources);

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

pub fn airtime_on_enter(entity: Entity, world: &World, _resources: &mut Resources) {
    log::info!("Player airtime v2");
}

pub fn airtime_on_update(entity: Entity, world: &World, resources: &mut Resources) {
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
