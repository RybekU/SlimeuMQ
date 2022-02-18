use super::EffectData;
use crate::gfx::Sprite;
use hecs::World;
use macroquad::color::{Color, WHITE};

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

pub fn tint_system(world: &mut World) {
    for (_eid, (tint, effect)) in world.query::<(&mut TintChange, &EffectData)>().iter() {
        if tint.new {
            // world.get::<&mut Sprite>(effect.parent).unwrap();
            if let Ok(mut sprite) = world.get_mut::<Sprite>(effect.parent) {
                sprite.color = tint.on_creation;
            }
            tint.new = false;
        }

        if effect.duration.is_sign_negative() {
            if let Ok(mut sprite) = world.get_mut::<Sprite>(effect.parent) {
                sprite.color = tint.on_deletion;
            }
        }
    }
}
