use super::EffectData;
use crate::gfx::Sprite;
use legion::{system, world::SubWorld, IntoQuery};
use macroquad::{Color, WHITE};

pub struct TintChange {
    /// Color set when effect begins
    pub on_creation: Color,
    /// Color set after effect ends
    pub on_deletion: Color,
    pub new: bool,
}

impl TintChange {
    pub fn new(on_creation: Color) -> Self {
        Self { on_creation, on_deletion: WHITE, new: true }
    }
}

#[system(for_each)]
#[write_component(Sprite)]
pub fn tint(world: &mut SubWorld, tint: &mut TintChange, effect: &mut EffectData) {
    if tint.new {
        if let Ok(sprite) = <&mut Sprite>::query().get_mut(world, effect.parent) {
            sprite.color = tint.on_creation;
        }
        tint.new = false;
    }

    if effect.duration.is_sign_negative() {
        if let Ok(sprite) = <&mut Sprite>::query().get_mut(world, effect.parent) {
            sprite.color = tint.on_deletion;
        }
    }
}
