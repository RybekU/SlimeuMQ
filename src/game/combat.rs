use super::player::PlayerControlled;
use crate::phx::{BodySet, ColliderSet, PhysicsWorld};
use glam::Vec2;
use legion::{system, Entity};

// Treat things that just react on getting hit (change direction, disappear) in a different way to things that actually have some sort of HP
// ^ the above statement is not a decision set in stone yet

/// Every entity that is truly alive has it
#[derive(Debug, Clone)]
pub struct Vitality {
    // TODO: Store the action (sound+behavior?) when hit?
}

#[derive(Debug, Clone)]
pub struct DamageEvent {
    pub input: Entity,
    pub output: Entity,
}

pub struct DamageQueue {
    events: Vec<DamageEvent>,
}

impl DamageQueue {
    pub fn new() -> Self {
        Self {
            events: Vec::with_capacity(128),
        }
    }
    pub fn push(&mut self, damage_event: DamageEvent) {
        self.events.push(damage_event);
    }
}

#[system]
#[write_component(PlayerControlled)]
#[write_component(Vitality)]
// #[write_component(AIControlled)]
pub fn apply_damage(#[resource] damage_queue: &mut DamageQueue) {
    // get input's damage
    // get output's vitality
    for DamageEvent { input, output } in &damage_queue.events {
        log::debug!(
            "A DamageEvent arrived succesfully from {:?} and hit {:?}",
            input,
            output
        )
    }

    damage_queue.events.clear();
}
// TODO: Scrap below in favor of persistent toggleable hitbox in physics engine

#[derive(Debug, Clone)]
pub struct HurtInfo {
    pub attacker: Entity,
    // TODO: Right now the position is not accurate in regards to how much the body moved during the frame
    // perhaps should use Collider/Body handle + offset?
    // this requires further testing to see which way (current pos vs future pos) feels more natural
    pub position: Vec2,
    pub half_exts: Vec2,
    pub mask: u32,
    // extend by adding info like:
    // enum intent (damage...)
    // report success to attacker - tricky, not necessary right now
}

// TODO: Keep copy_msgs for feature devmode only
pub struct HurtQueue {
    msgs: Vec<HurtInfo>,
    pub copy_msgs: Vec<HurtInfo>,
}

impl HurtQueue {
    pub fn new() -> Self {
        Self {
            msgs: Vec::with_capacity(128),
            copy_msgs: Vec::with_capacity(128),
        }
    }
    pub fn push(&mut self, hurt_info: HurtInfo) {
        self.msgs.push(hurt_info);
    }
}

/// This system is responsible for checking checking which entities are supposed to get hurt and informing them.
#[system]
pub fn spread_pain(
    #[resource] hurt_queue: &mut HurtQueue,
    #[resource] damage_queue: &mut DamageQueue,
    #[resource] phys_world: &PhysicsWorld,
    #[resource] bodies: &BodySet,
    #[resource] colliders: &ColliderSet,
    #[resource] body_entity_map: &crate::phx::BodyEntityMap,
) {
    hurt_queue.copy_msgs.clear();

    for hurt_info in &hurt_queue.msgs {
        let hits = phys_world.overlap_test(
            hurt_info.position,
            hurt_info.half_exts,
            hurt_info.mask,
            bodies,
            colliders,
        );
        for chandle in hits {
            if let Some(defender) = body_entity_map.get(&colliders[chandle].owner) {
                damage_queue.push(DamageEvent {
                    input: hurt_info.attacker,
                    output: *defender,
                });
            }
        }
    }
    std::mem::swap(&mut hurt_queue.copy_msgs, &mut hurt_queue.msgs);
}
