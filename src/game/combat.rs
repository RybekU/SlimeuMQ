use super::ai::HitMemory;
use crate::effect::{tint::TintChange, EffectData};
use crate::phx::{BodySet, ColliderSet, PhysicsWorld, Velocity};
use glam::Vec2;
use hecs::{CommandBuffer, Entity, World};
use macroquad::color::Color;

// Treat things that just react on getting hit (change direction, disappear) in a different way to things that actually have some sort of HP
// ^ the above statement is not a decision set in stone yet

/// Every entity that partakes in combat has this
#[derive(Debug, Clone)]
pub struct CombatStats {
    /// knockback force
    pub kb_force: Vec2,
    /// knockback resistance
    pub kb_res: f32,
}

impl CombatStats {
    pub fn new() -> Self {
        Self { kb_force: Vec2::new(64., -64.), kb_res: 0.5 }
    }
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
        Self { events: Vec::with_capacity(128) }
    }
    pub fn push(&mut self, damage_event: DamageEvent) {
        self.events.push(damage_event);
    }
}

pub fn apply_damage_system(
    world: &mut World,
    damage_queue: &mut DamageQueue,
    command_buffer: &mut CommandBuffer,
) {
    for DamageEvent { input, output } in damage_queue.events.drain(..) {
        log::debug!("A DamageEvent arrived succesfully from {:?} and hit {:?}", input, output);

        let maybe_off_combat = world.get_mut::<CombatStats>(input).ok().map(|x| x.clone());

        if let Ok((maybe_hit_memory, maybe_velocity, def_combat)) =
            world.query_one_mut::<(Option<&mut HitMemory>, Option<&mut Velocity>, &CombatStats)>(
                output,
            )
        {
            if let Some(HitMemory(hit_state)) = maybe_hit_memory {
                *hit_state = true;
            }

            let knockback = if let Some(off_combat) = maybe_off_combat {
                off_combat.kb_force * (1. - def_combat.kb_res)
            } else {
                Vec2::ZERO
            };

            if let Some(velocity) = maybe_velocity {
                // TODO: take into account direction
                velocity.src = knockback;
            }

            command_buffer.spawn((
                EffectData { parent: output, duration: 0.15 },
                TintChange::new(macroquad::color_u8!(255, 36, 0, 192)),
            ));
        }
    }
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
        Self { msgs: Vec::with_capacity(128), copy_msgs: Vec::with_capacity(128) }
    }
    pub fn push(&mut self, hurt_info: HurtInfo) {
        self.msgs.push(hurt_info);
    }
}

/// This system is responsible for checking checking which entities are supposed to get hurt and informing them.
pub fn spread_pain_system(
    hurt_queue: &mut HurtQueue,
    damage_queue: &mut DamageQueue,
    phys_world: &PhysicsWorld,
    bodies: &BodySet,
    colliders: &ColliderSet,
    body_entity_map: &crate::phx::BodyEntityMap,
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
                damage_queue.push(DamageEvent { input: hurt_info.attacker, output: *defender });
            }
        }
    }
    std::mem::swap(&mut hurt_queue.copy_msgs, &mut hurt_queue.msgs);
}
