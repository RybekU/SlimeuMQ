use crate::gfx::Sprite;
use crate::phx::{OnGround, Velocity};
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
#[write_component(Sprite)]
#[read_component(OnGround)]
pub fn update_fsm(world: &mut SubWorld, #[resource] inputs: &ButtonsState) {
    let mut query = <(Entity, &mut PlayerControlled)>::query().filter(component::<Velocity>());

    let (mut pc_world, mut rest_world) = world.split_for_query(&query);

    for (entity, player_controlled) in query.iter_mut(&mut pc_world) {
        if let Some(transition) = player_controlled
            .state
            .update(entity, &mut rest_world, inputs)
        {
            player_controlled.state = transition;
            player_controlled.state.on_enter(entity, &mut rest_world);
        }
    }
}

enum PlayerState {
    Idle,
    Walk,
    // bool - is_falling
    InAir(bool),
    Attack(f32),
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
            Self::Walk => walk_update(entity, world, inputs),
            Self::InAir(data) => in_air_update(entity, world, data, inputs),
            Self::Attack(data) => attack_update(entity, world, data, inputs),
        }
    }
    fn on_enter(&mut self, entity: &Entity, world: &mut SubWorld) {
        match self {
            Self::Idle => idle_on_enter(entity, world),
            Self::Walk => walk_on_enter(entity, world),
            Self::InAir(data) => in_air_on_enter(entity, world, data),
            Self::Attack(_data) => attack_on_enter(entity, world),
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
    let on_ground = entry.get_component::<OnGround>().unwrap().on_ground;
    let vel = entry.get_component_mut::<Velocity>().unwrap();

    handle_jump(inputs, vel);

    if let atk_option @ Some(_) = handle_attack(inputs) {
        return atk_option;
    }

    if !on_ground {
        return Some(PlayerState::InAir(vel.src.y() > 0.));
    }

    if inputs.is_pressed(Button::Right) ^ inputs.is_pressed(Button::Left) {
        walk_update(entity, world, inputs);
        return Some(PlayerState::Walk);
    }

    // decelerate quickly when no input is given
    vel.src
        .set_x(lerp(0., vel.src.x(), f32::exp2(-DECEL * FRAMETIME)));

    if vel.src.x().abs() < 1. {
        vel.src.set_x(0.);
    }

    None
}

fn idle_on_enter(_entity: &Entity, _world: &SubWorld) {
    log::info!("Player idle");
}

fn walk_update(
    entity: &Entity,
    world: &mut SubWorld,
    inputs: &ButtonsState,
) -> Option<PlayerState> {
    const TARGET_SPEED: f32 = 64.;
    const ACCEL: f32 = 10.0;

    // TODO: add better error checks
    let (vel, on_ground, sprite) = <(&mut Velocity, &OnGround, &mut Sprite)>::query()
        .get_mut(world, *entity)
        .unwrap();
    let flipped = &mut sprite.flip;
    let on_ground = on_ground.on_ground;

    handle_jump(inputs, vel);

    if let atk_option @ Some(_) = handle_attack(inputs) {
        return atk_option;
    }

    let target_speed = {
        let dir =
            (inputs.is_pressed(Button::Right) as i8) - (inputs.is_pressed(Button::Left) as i8);
        if dir == 0 {
            return Some(PlayerState::Idle);
        }
        TARGET_SPEED * dir as f32
    };

    // flip sprite if necessary
    *flipped = match (
        inputs.is_pressed(Button::Left),
        inputs.is_pressed(Button::Right),
    ) {
        (true, false) => true,
        (false, true) => false,
        _ => *flipped,
    };

    vel.src.set_x(lerp(
        target_speed,
        vel.src.x(),
        f32::exp2(-ACCEL * FRAMETIME),
    ));

    if vel.src.x().abs() < 1. {
        vel.src.set_x(0.);
    };

    if !on_ground {
        return Some(PlayerState::InAir(vel.src.y() > 0.));
    }

    None
}

fn walk_on_enter(_entity: &Entity, _world: &SubWorld) {
    log::info!("Player walking");
}

fn in_air_update(
    entity: &Entity,
    world: &mut SubWorld,
    falling: &mut bool,
    inputs: &ButtonsState,
) -> Option<PlayerState> {
    const TARGET_SPEED: f32 = 64.;
    const ACCEL: f32 = 5.0;

    // TODO: add better error checks
    let mut entry = world.entry_mut(*entity).unwrap();
    let on_ground = entry.get_component::<OnGround>().unwrap().on_ground;
    let vel = entry.get_component_mut::<Velocity>().unwrap();

    let target_speed = {
        let dir =
            (inputs.is_pressed(Button::Right) as i8) - (inputs.is_pressed(Button::Left) as i8);
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

    if let atk_option @ Some(_) = handle_attack(inputs) {
        return atk_option;
    }

    if on_ground {
        if target_speed != 0. {
            return Some(PlayerState::Walk);
        } else {
            return Some(PlayerState::Idle);
        }
    }

    if *falling != (vel.src.y() > 0.) {
        *falling = vel.src.y() > 0.;
    }

    None
}

fn in_air_on_enter(_entity: &Entity, _world: &SubWorld, _falling: &mut bool) {
    log::info!("Player in air");
}

fn attack_update(
    entity: &Entity,
    world: &mut SubWorld,
    cd: &mut f32,
    _inputs: &ButtonsState,
) -> Option<PlayerState> {
    const LAND_DECEL: f32 = 10.;
    const AIR_DECEL: f32 = 5.;
    *cd -= FRAMETIME;

    let (vel, on_ground) = <(&mut Velocity, &OnGround)>::query()
        .get_mut(world, *entity)
        .unwrap();
    let on_ground = on_ground.on_ground;

    let decel = if on_ground { LAND_DECEL } else { AIR_DECEL };

    vel.src
        .set_x(lerp(0., vel.src.x(), f32::exp2(-decel * FRAMETIME)));

    if vel.src.x().abs() < 1. {
        vel.src.set_x(0.);
    }

    if *cd <= 0. {
        return Some(PlayerState::Idle);
    }
    None
}

fn attack_on_enter(_entity: &Entity, _world: &SubWorld) {
    log::info!("Player attacks");
}

fn handle_jump(inputs: &ButtonsState, vel: &mut Velocity) {
    if inputs.pressed(Button::Jump) {
        vel.src.set_y(-156.);
    }
}

fn handle_attack(inputs: &ButtonsState) -> Option<PlayerState> {
    if inputs.is_pressed(Button::Attack) {
        Some(PlayerState::Attack(0.5))
    } else {
        None
    }
}
