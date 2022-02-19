use crate::game::combat::HurtInfo;
use crate::gfx::{Animation, Sprite};
use crate::phx::{OnGround, Position, Velocity};
use crate::util::{input::Button, lerp, ButtonsState};
use crate::FRAMETIME;
use glam::Vec2;
use hecs::{Entity, With, World};

use super::resources::Resources;

pub struct PlayerControlled {
    state: PlayerState,
}

impl PlayerControlled {
    pub fn new() -> Self {
        Self { state: PlayerState::Idle }
    }
}

pub fn update_fsm_system(world: &mut World, resources: &mut Resources) {
    let mut query = world.query::<With<Velocity, &mut PlayerControlled>>();

    for (entity, player_controlled) in query.iter() {
        if let Some(transition) = player_controlled.state.update(entity, world, resources) {
            player_controlled.state = transition;
            player_controlled.state.on_enter(entity, world, resources);
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
    fn update(&mut self, entity: Entity, world: &World, resources: &Resources) -> Option<Self> {
        match self {
            Self::Idle => idle_update(entity, world, resources),
            Self::Walk => walk_update(entity, world, resources),
            Self::InAir(data) => in_air_update(entity, world, data, resources),
            Self::Attack(data) => attack_update(entity, world, data, resources),
        }
    }
    fn on_enter(&mut self, entity: Entity, world: &World, resources: &mut Resources) {
        match self {
            Self::Idle => idle_on_enter(entity, world),
            Self::Walk => walk_on_enter(entity, world),
            Self::InAir(data) => in_air_on_enter(entity, world, data),
            Self::Attack(_data) => attack_on_enter(entity, world, resources),
        }
    }
}
fn idle_update(entity: Entity, world: &World, resources: &Resources) -> Option<PlayerState> {
    const DECEL: f32 = 20.0;

    let inputs = &resources.input_buttons;

    let mut query_o = world.query_one::<(&OnGround, &mut Velocity)>(entity).unwrap();
    let (on_ground, vel) = query_o.get().unwrap();

    let on_ground = on_ground.on_ground;

    handle_jump(inputs, vel);

    if let atk_option @ Some(_) = handle_attack(inputs) {
        return atk_option;
    }

    if !on_ground {
        return Some(PlayerState::InAir(vel.src.y > 0.));
    }

    if inputs.is_pressed(Button::Right) ^ inputs.is_pressed(Button::Left) {
        return Some(PlayerState::Walk);
    }

    // decelerate quickly when no input is given
    vel.src.x = lerp(0., vel.src.x, f32::exp2(-DECEL * FRAMETIME));

    if vel.src.x.abs() < 1. {
        vel.src.x = 0.;
    }

    None
}

fn idle_on_enter(entity: Entity, world: &World) {
    log::info!("Player idle");
    let mut animation = world.get_mut::<Animation>(entity).unwrap();

    animation.change("slimeu_idle");
}

fn walk_update(entity: Entity, world: &World, resources: &Resources) -> Option<PlayerState> {
    const TARGET_SPEED: f32 = 64.;
    const ACCEL: f32 = 10.0;

    let inputs = &resources.input_buttons;

    let mut query_o = world.query_one::<(&mut Velocity, &OnGround, &mut Sprite)>(entity).unwrap();
    let (vel, on_ground, sprite) = query_o.get().unwrap();

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
    *flipped = match (inputs.is_pressed(Button::Left), inputs.is_pressed(Button::Right)) {
        (true, false) => true,
        (false, true) => false,
        _ => *flipped,
    };

    vel.src.x = lerp(target_speed, vel.src.x, f32::exp2(-ACCEL * FRAMETIME));

    if vel.src.x.abs() < 1. {
        vel.src.x = 0.;
    };

    if !on_ground {
        return Some(PlayerState::InAir(vel.src.y > 0.));
    }

    None
}

fn walk_on_enter(entity: Entity, world: &World) {
    log::info!("Player walking");
    let mut animation = world.get_mut::<Animation>(entity).unwrap();

    animation.change("slimeu_run");
}

fn in_air_update(
    entity: Entity,
    world: &World,
    falling: &mut bool,
    resources: &Resources,
) -> Option<PlayerState> {
    const TARGET_SPEED: f32 = 64.;
    const ACCEL: f32 = 5.0;

    let inputs = &resources.input_buttons;

    let mut query_o = world.query_one::<(&mut Velocity, &OnGround)>(entity).unwrap();
    let (vel, on_ground) = query_o.get().unwrap();

    let on_ground = on_ground.on_ground;

    let target_speed = {
        let dir =
            (inputs.is_pressed(Button::Right) as i8) - (inputs.is_pressed(Button::Left) as i8);
        TARGET_SPEED * dir as f32
    };

    vel.src.x = lerp(target_speed, vel.src.x, f32::exp2(-ACCEL * FRAMETIME));

    if vel.src.x.abs() < 1. {
        vel.src.x = 0.;
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

    if *falling != (vel.src.y > 0.) {
        *falling = vel.src.y > 0.;
    }

    None
}

fn in_air_on_enter(_entity: Entity, _world: &World, _falling: &mut bool) {
    log::info!("Player in air");
}

fn attack_update(
    entity: Entity,
    world: &World,
    cd: &mut f32,
    _resources: &Resources,
) -> Option<PlayerState> {
    const LAND_DECEL: f32 = 10.;
    const AIR_DECEL: f32 = 5.;
    *cd -= FRAMETIME;

    let mut query_o = world.query_one::<(&mut Velocity, &OnGround)>(entity).unwrap();
    let (vel, on_ground) = query_o.get().unwrap();
    let on_ground = on_ground.on_ground;

    let decel = if on_ground { LAND_DECEL } else { AIR_DECEL };

    vel.src.x = lerp(0., vel.src.x, f32::exp2(-decel * FRAMETIME));

    if vel.src.x.abs() < 1. {
        vel.src.x = 0.;
    }

    if *cd <= 0. {
        return Some(PlayerState::Idle);
    }
    None
}

fn attack_on_enter(entity: Entity, world: &World, resources: &mut Resources) {
    log::info!("Player attacks");

    let mut query_o = world.query_one::<(&Position, &Sprite)>(entity).unwrap();
    let (position, sprite) = query_o.get().unwrap();

    let offset = if sprite.flip { -16. } else { 16. };

    let hurtbox = position.src + Vec2::new(offset, 0.);
    let half_exts = Vec2::new(8., 4.);

    let hurt_info = HurtInfo {
        attacker: entity,
        position: hurtbox,
        half_exts,
        mask: crate::phx::Category::ENEMY.bits(),
    };

    resources.hurt_queue.push(hurt_info);
}

fn handle_jump(inputs: &ButtonsState, vel: &mut Velocity) {
    if inputs.pressed(Button::Jump) {
        vel.src.y = -156.;
    }
}

fn handle_attack(inputs: &ButtonsState) -> Option<PlayerState> {
    if inputs.pressed(Button::Attack) {
        Some(PlayerState::Attack(0.1))
    } else {
        None
    }
}
