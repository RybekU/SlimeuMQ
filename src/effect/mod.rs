pub mod tint;

use hecs::{CommandBuffer, Entity, World};

use crate::FRAMETIME;

pub struct EffectData {
    pub parent: hecs::Entity,
    pub duration: f32,
}

pub fn effect_update_system(world: &mut World, cmd: &mut CommandBuffer) {
    for (eid, effect_data) in world.query_mut::<&mut EffectData>() {
        effect_update(eid, cmd, effect_data);
    }
}

fn effect_update(entity: Entity, command_buffer: &mut CommandBuffer, effect_data: &mut EffectData) {
    effect_data.duration -= FRAMETIME;

    if effect_data.duration.is_sign_negative() {
        command_buffer.despawn(entity);
    }
}
