use crate::{
    game::combat::{DamageQueue, HurtQueue},
    gfx::AnimationStorage,
    phx::{BodyEntityMap, BodySet, ColliderSet, PhysicsWorld},
    util::{ButtonsState, Camera},
};

use super::stage::Stage;

pub struct Resources {
    pub animations: AnimationStorage,
    pub camera: Camera,
    pub phys: PhysicsWorld,
    pub phys_bodies: BodySet,
    pub phys_colliders: ColliderSet,
    pub input_buttons: ButtonsState,
    pub hurt_queue: HurtQueue,
    pub damage_queue: DamageQueue,
    pub body_entity_map: BodyEntityMap,
    pub stage: Stage,
}

impl Default for Resources {
    fn default() -> Self {
        Self::new()
    }
}

impl Resources {
    pub fn new() -> Self {
        let animations = AnimationStorage::default();
        let camera = Camera::new();
        let phys = PhysicsWorld::new();
        let phys_bodies = BodySet::new();
        let phys_colliders = ColliderSet::new();
        let input_buttons = ButtonsState::new();
        // TODO: Replace HurtQueue and DamageQueue with an event system.
        let hurt_queue = HurtQueue::new();
        let damage_queue = DamageQueue::new();
        let body_entity_map = BodyEntityMap::default();
        let stage = Stage::from_ldtk();
        Self {
            animations,
            camera,
            phys,
            phys_bodies,
            phys_colliders,
            input_buttons,
            hurt_queue,
            damage_queue,
            body_entity_map,
            stage,
        }
    }
}
