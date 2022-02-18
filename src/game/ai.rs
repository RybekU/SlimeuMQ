use hecs::{Entity, With, World};

use crate::phx::Velocity;
use crate::util::lerp;
use crate::FRAMETIME;

use super::resources::Resources;

pub struct AiControlled {
    state: AiState,
}

// entities with this component want to remember getting hit
// TODO: should be replaced with generic event when FSM is reworked
#[derive(Debug, Clone)]
pub struct HitMemory(pub bool);

impl HitMemory {
    pub fn new() -> Self {
        Self(false)
    }
}

impl AiControlled {
    pub fn new() -> Self {
        Self { state: AiState::Idle }
    }
}

enum AiState {
    Idle,
    Hurt(f32),
}

pub fn update_fsm_system(world: &mut World, resources: &mut Resources) {
    let mut query = world.query::<With<Velocity, &mut AiControlled>>();

    for (entity, ai_controlled) in query.iter() {
        if let Some(transition) = ai_controlled.state.update(entity, world, resources) {
            ai_controlled.state = transition;
            ai_controlled.state.on_enter(entity, world, resources);
        }
    }
}

impl AiState {
    fn update(&mut self, entity: Entity, world: &World, resources: &Resources) -> Option<Self> {
        match self {
            Self::Idle => idle_update(entity, world, resources),
            Self::Hurt(timer) => hurt_update(entity, world, timer, resources),
        }
    }
    fn on_enter(&mut self, entity: Entity, world: &World, _resources: &mut Resources) {
        match self {
            Self::Idle => idle_on_enter(entity, world),
            Self::Hurt(_timer) => hurt_on_enter(entity, world),
        }
    }
}

fn idle_update(entity: Entity, world: &World, _resources: &Resources) -> Option<AiState> {
    const DECEL: f32 = 20.;

    let mut query_o = world.query_one::<(&mut HitMemory, &mut Velocity)>(entity).unwrap();
    let (HitMemory(is_hit), Velocity { src: vel }) = query_o.get().unwrap();

    vel.set_x(lerp(0., vel.x(), f32::exp2(-DECEL * FRAMETIME)));

    if vel.x().abs() < 1. {
        vel.set_x(0.);
    }

    if *is_hit {
        *is_hit = false;
        return Some(AiState::Hurt(2.));
    }

    None
}

fn idle_on_enter(_entity: Entity, _world: &World) {
    log::info!("Enemy idle");
}

fn hurt_update(
    entity: Entity,
    world: &World,
    timer: &mut f32,
    _resources: &Resources,
) -> Option<AiState> {
    const DECEL: f32 = 10.0;

    let mut query_o = world.query_one::<(&mut HitMemory, &mut Velocity)>(entity).unwrap();
    let (HitMemory(is_hit), Velocity { src: vel }) = query_o.get().unwrap();
    *is_hit = false;

    *timer -= FRAMETIME;

    vel.set_x(lerp(0., vel.x(), f32::exp2(-DECEL * FRAMETIME)));

    if vel.x().abs() < 1. {
        vel.set_x(0.);
    }

    if *timer <= 0. {
        return Some(AiState::Idle);
    }

    None
}

fn hurt_on_enter(_entity: Entity, _world: &World) {
    log::info!("Enemy got hit");
}
