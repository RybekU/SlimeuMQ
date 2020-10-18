use crate::phx::{BodySet, ColliderSet, PhysicsWorld};
use glam::Vec2;
use legion::{system, Entity};

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
    #[resource] phys_world: &PhysicsWorld,
    #[resource] bodies: &BodySet,
    #[resource] colliders: &ColliderSet,
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
        let hit_count = hits.count();
        if hit_count > 0 {
            println!("{:?} hit {}", hurt_info.attacker, hit_count);
        }
    }
    std::mem::swap(&mut hurt_queue.copy_msgs, &mut hurt_queue.msgs);
}
