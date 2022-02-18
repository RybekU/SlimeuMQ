use super::{BodySet, ColliderSet, PhysicsWorld, Position, Velocity};
use hecs::World;
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

pub fn resphys_sync_system(
    world: &mut World,
    phys_world: &mut PhysicsWorld,
    bodies: &mut BodySet,
    colliders: &mut ColliderSet,
) {
    let query = world.query_mut::<(&mut Position, &mut Velocity, &Hitbox)>();

    //  keep position and velocity the same in the physics world and the rest of the engine
    //  TODO: could be done only if velocity/position changes in a smarter manner!
    for (_eid, (pos, vel, hitbox)) in query {
        resphys_presync(bodies, colliders, pos, vel, hitbox);
    }

    phys_world.step(crate::FRAMETIME, bodies, colliders);

    let query = world.query_mut::<(&mut Position, &mut Velocity, &Hitbox)>();
    // update entity position and velocity based physics simulation's state
    for (_eid, (pos, vel, hitbox)) in query {
        resphys_postsync(bodies, colliders, pos, vel, hitbox);
    }
}

fn resphys_presync(
    bodies: &mut BodySet,
    colliders: &mut ColliderSet,
    pos: &mut Position,
    vel: &mut Velocity,
    hitbox: &Hitbox,
) {
    let collider = &colliders[hitbox.src];
    let body = &mut bodies[collider.owner];
    body.position = pos.src;
    body.velocity = vel.src;
}

fn resphys_postsync(
    bodies: &mut BodySet,
    colliders: &mut ColliderSet,
    pos: &mut Position,
    vel: &mut Velocity,
    hitbox: &Hitbox,
) {
    let collider = &colliders[hitbox.src];
    let body = &mut bodies[collider.owner];
    pos.src = body.position;
    vel.src = body.velocity;
}
