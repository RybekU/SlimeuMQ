use hecs::{Entity, World};

use crate::{
    game::resources::Resources,
    phx::{OnGround, Velocity},
    util::input::Button,
};

// TODO: Allow queue jump when approaching ground (time based)
// TODO: Allow jumping after going to airtime state for a splitsecond (time based)

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

// STATES
pub fn jump_on_enter(entity: Entity, world: &World, _resources: &mut Resources) {
    // full jump height: 56 pixels
    // reach apex in: 0.5s
    const INIT_VELOCITY: f32 = -224.0;

    log::info!("Player jumping v2");
    let mut vel = world.get_mut::<Velocity>(entity).unwrap();
    vel.src.y = INIT_VELOCITY;
}

pub fn jump_on_update(entity: Entity, world: &World, resources: &mut Resources) {
    const TARGET_SPEED: f32 = 64.;
    const ACCEL: f32 = 5.0;

    super::movement::handle_movement(entity, world, resources, TARGET_SPEED, ACCEL);
}

pub fn jump_on_exit(entity: Entity, world: &World, _resources: &mut Resources) {
    let mut vel = world.get_mut::<Velocity>(entity).unwrap();
    vel.src.y = vel.src.y.max(-128.);
}

pub fn airtime_on_enter(_entity: Entity, _world: &World, _resources: &mut Resources) {
    log::info!("Player airtime v2");
}

pub fn airtime_on_update(entity: Entity, world: &World, resources: &mut Resources) {
    const TARGET_SPEED: f32 = 64.;
    const ACCEL: f32 = 5.0;

    super::movement::handle_movement(entity, world, resources, TARGET_SPEED, ACCEL);
}
