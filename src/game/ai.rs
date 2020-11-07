use crate::gfx::Sprite;
use crate::phx::{OnGround, Position, Velocity};
use crate::util::lerp;
use crate::FRAMETIME;
use legion::{component, system, world::SubWorld, Entity, IntoQuery};

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
        Self {
            state: AiState::Idle,
        }
    }
}

enum AiState {
    Idle,
    Hurt(f32),
}

struct ResourceRefs {}

#[system]
#[write_component(Velocity)]
#[write_component(AiControlled)]
#[write_component(Sprite)]
#[write_component(HitMemory)]
#[read_component(OnGround)]
#[read_component(Position)]
pub fn update_fsm(world: &mut SubWorld) {
    let mut query = <(Entity, &mut AiControlled)>::query().filter(component::<Velocity>());

    let mut resources = ResourceRefs {};

    let (mut ac_world, mut rest_world) = world.split_for_query(&query);

    for (entity, ai_controlled) in query.iter_mut(&mut ac_world) {
        if let Some(transition) = ai_controlled
            .state
            .update(entity, &mut rest_world, &resources)
        {
            ai_controlled.state = transition;
            ai_controlled
                .state
                .on_enter(entity, &mut rest_world, &mut resources);
        }
    }
}

impl AiState {
    fn update(
        &mut self,
        entity: &Entity,
        world: &mut SubWorld,
        resources: &ResourceRefs,
    ) -> Option<Self> {
        match self {
            Self::Idle => idle_update(entity, world, resources),
            Self::Hurt(timer) => hurt_update(entity, world, timer, resources),
        }
    }
    fn on_enter(&mut self, entity: &Entity, world: &mut SubWorld, _resources: &mut ResourceRefs) {
        match self {
            Self::Idle => idle_on_enter(entity, world),
            Self::Hurt(_timer) => hurt_on_enter(entity, world),
        }
    }
}

fn idle_update(
    entity: &Entity,
    world: &mut SubWorld,
    _resources: &ResourceRefs,
) -> Option<AiState> {
    const DECEL: f32 = 20.;

    let (HitMemory(is_hit), Velocity { src: vel }) = <(&mut HitMemory, &mut Velocity)>::query()
        .get_mut(world, *entity)
        .unwrap();

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

fn idle_on_enter(_entity: &Entity, _world: &SubWorld) {
    log::info!("Enemy idle");
}

fn hurt_update(
    entity: &Entity,
    world: &mut SubWorld,
    timer: &mut f32,
    _resources: &ResourceRefs,
) -> Option<AiState> {
    const DECEL: f32 = 10.0;

    let (HitMemory(is_hit), Velocity { src: vel }) = <(&mut HitMemory, &mut Velocity)>::query()
        .get_mut(world, *entity)
        .unwrap();
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

fn hurt_on_enter(_entity: &Entity, _world: &SubWorld) {
    log::info!("Enemy got hit");
}
