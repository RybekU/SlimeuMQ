use super::{BodySet, ColliderSet, PhysicsWorld, Position, Velocity};
use legion::{maybe_changed, system, world::SubWorld, IntoQuery};
use resphys::ColliderHandle;
#[derive(Debug)]
pub struct Hitbox {
    pub src: ColliderHandle,
}

impl Hitbox {
    pub fn new(chandle: ColliderHandle) -> Self {
        Self { src: chandle }
    }
}

// syncs position and velocity set in ECS with the one in actual physics world
// while velocity is fine to modify position changes should be avoided!
#[system(for_each)]
#[filter(maybe_changed::<Velocity>())]
pub fn resphys_presync(
    #[resource] bodies: &mut BodySet,
    #[resource] colliders: &mut ColliderSet,
    pos: &mut Position,
    vel: &mut Velocity,
    hitbox: &Hitbox,
) {
    let collider = &colliders[hitbox.src];
    let body = &mut bodies[collider.owner];
    body.position = pos.src;
    body.velocity = vel.src;
}

// runs physics step and syncs back position with components
#[system]
#[read_component(Hitbox)]
#[write_component(Position)]
#[write_component(Velocity)]
pub fn resphys_sync(
    world: &mut SubWorld,
    #[resource] phys_world: &mut PhysicsWorld,
    #[resource] bodies: &mut BodySet,
    #[resource] colliders: &mut ColliderSet,
) {
    phys_world.step(crate::FRAMETIME, bodies, colliders);

    let mut query = <(&mut Position, &mut Velocity, &Hitbox)>::query();

    for (pos, vel, hitbox) in query.iter_mut(world) {
        let collider = &colliders[hitbox.src];
        let body = &mut bodies[collider.owner];
        pos.src = body.position;
        vel.src = body.velocity;
    }
}
