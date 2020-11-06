pub mod tint;

use crate::FRAMETIME;
use legion::{system, systems::CommandBuffer, Entity};

pub struct EffectData {
    pub parent: legion::Entity,
    pub duration: f32,
}

#[system(for_each)]
pub fn effect_update(
    entity: &Entity,
    command_buffer: &mut CommandBuffer,
    effect_data: &mut EffectData,
) {
    effect_data.duration -= FRAMETIME;

    if effect_data.duration.is_sign_negative() {
        command_buffer.remove(*entity);
    }
}
