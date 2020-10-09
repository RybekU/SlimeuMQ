use crate::phx::{Hitbox, PhysicsWorld, Velocity};
use crate::util::{input::Button, lerp, ButtonsState};
use crate::FRAMETIME;
use legion::{component, system, world::SubWorld, Entity, EntityStore, IntoQuery};

pub struct PlayerControlled {
    state: PlayerState,
}

impl PlayerControlled {
    pub fn new() -> Self {
        Self {
            state: PlayerState::Idle,
        }
    }
}

// FRAME_DEPENDANT
#[system]
#[write_component(Velocity)]
#[write_component(PlayerControlled)]
pub fn update_fsm(world: &mut SubWorld, #[resource] inputs: &ButtonsState) {
    let mut query = <(Entity, &mut PlayerControlled)>::query().filter(component::<Velocity>());

    let (mut pc_world, mut rest_world) = world.split_for_query(&query);

    for (entity, player_controlled) in query.iter_mut(&mut pc_world) {
        if let Some(transition) = player_controlled
            .state
            .update(entity, &mut rest_world, inputs)
        {
            player_controlled.state = transition;
        }
    }
}

enum PlayerState {
    Idle,
    Walking,
}

impl PlayerState {
    fn update(
        &mut self,
        entity: &Entity,
        world: &mut SubWorld,
        inputs: &ButtonsState,
    ) -> Option<Self> {
        match self {
            Self::Idle => idle_update(entity, world, inputs),
            Self::Walking => walking_update(entity, world, inputs),
        }
    }
}
fn idle_update(
    entity: &Entity,
    world: &mut SubWorld,
    inputs: &ButtonsState,
) -> Option<PlayerState> {
    const DECEL: f32 = 20.0;

    // TODO: add better error checks
    let mut entry = world.entry_mut(*entity).unwrap();
    let vel = entry.get_component_mut::<Velocity>().unwrap();

    if inputs.is_pressed(Button::Right) ^ inputs.is_pressed(Button::Left) {
        log::debug!("To walking state");
        walking_update(entity, world, inputs);
        return Some(PlayerState::Walking);
    }

    // decelerate quickly when no input is given
    vel.src
        .set_x(lerp(0., vel.src.x(), f32::exp2(-DECEL * FRAMETIME)));

    if vel.src.x().abs() < 1. {
        vel.src.set_x(0.);
    }

    check_jump(inputs, vel);
    None
}

fn walking_update(
    entity: &Entity,
    world: &mut SubWorld,
    inputs: &ButtonsState,
) -> Option<PlayerState> {
    const TARGET_SPEED: f32 = 64.;
    const ACCEL: f32 = 10.0;
    let mut transition = None;

    // TODO: add better error checks
    let mut entry = world.entry_mut(*entity).unwrap();
    let vel = entry.get_component_mut::<Velocity>().unwrap();

    let target_speed = {
        let dir =
            (inputs.is_pressed(Button::Right) as i8) - (inputs.is_pressed(Button::Left) as i8);
        if dir == 0 {
            transition = Some(PlayerState::Idle);
        }
        TARGET_SPEED * dir as f32
    };

    vel.src.set_x(lerp(
        target_speed,
        vel.src.x(),
        f32::exp2(-ACCEL * FRAMETIME),
    ));

    if vel.src.x().abs() < 1. {
        vel.src.set_x(0.);
    }
    check_jump(inputs, vel);
    transition
}

fn check_jump(inputs: &ButtonsState, vel: &mut Velocity) {
    if inputs.pressed(Button::Jump) {
        vel.src.set_y(-156.);
    }
}

// struct Rising{}
// struct Falling{}
// struct Attacking{}

// update
// transition
// on_enter
// on_exit
