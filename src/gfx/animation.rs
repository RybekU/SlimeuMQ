use super::Sprite;
use crate::FRAMETIME;
use legion::system;
use macroquad::math::Rect;

pub struct AnimationTemplate {
    pub repeat: bool,
    pub texture_name: String,
    /// rect to be set on `Sprite` component
    pub rect: Rect,
    pub move_by: f32,
    pub frames: Vec<Frame>,
}

// Start with default, most robust frame implementation and optimize for simple repeated offset later
#[derive(Debug, Clone)]
pub struct Frame {
    pub duration: f32,
    // ...TODO: Hitbox/Hurtbox
}

/// Component
pub struct Animation {
    pub played: String,
    /// current frame
    pub frame: usize,
    /// cached from `Frame`
    frame_duration: f32,
    /// accumulated time
    pub acc: f32,
    /// overrides `AnimationTemplate` repeat setting
    pub repeat: bool,
    state: State,
}

enum State {
    New,
    Playing,
    Finished,
}

impl Animation {
    pub fn new(animation_storage: &super::AnimationStorage, animation_name: &str) -> Self {
        let animation_template = animation_storage.get(animation_name).expect(&animation_name);
        Self {
            played: animation_name.to_owned(),
            frame: 0,
            frame_duration: animation_template.frames[0].duration,
            acc: 0.,
            repeat: animation_template.repeat,
            state: State::New,
        }
    }
    pub fn new_with_sprite(
        animation_storage: &super::AnimationStorage,
        animation_name: &str,
    ) -> (Self, Sprite) {
        let animation_template = animation_storage.get(animation_name).expect(&animation_name);
        (
            Self {
                played: animation_name.to_owned(),
                frame: 0,
                frame_duration: animation_template.frames[0].duration,
                acc: 0.,
                repeat: animation_template.repeat,
                state: State::New,
            },
            Sprite::new(
                animation_template.texture_name.clone(),
                animation_template.rect.x,
                animation_template.rect.y,
                animation_template.rect.w,
                animation_template.rect.h,
            ),
        )
    }

    pub fn change(&mut self, animation_name: &str) {
        self.played = animation_name.to_owned();
        self.state = State::New;
    }
}

/// TODO: Add Option<Hitbox>
#[system(for_each)]
pub fn animate(
    #[resource] animation_storage: &super::AnimationStorage,
    sprite: &mut Sprite,
    animation: &mut Animation,
) {
    match animation.state {
        State::New => {
            let animation_template =
                animation_storage.get(&animation.played).expect(&animation.played);
            sprite.texture = animation_template.texture_name.clone();
            sprite.rect = animation_template.rect;

            animation.frame = 0;
            animation.frame_duration = animation_template.frames[0].duration;
            animation.acc = FRAMETIME;
            animation.repeat = animation_template.repeat;

            animation.state = State::Playing;
        }
        State::Playing => {
            animation.acc += FRAMETIME;
            if animation.acc >= animation.frame_duration {
                animation.acc -= animation.frame_duration;
                animation.frame += 1;

                let animation_template =
                    animation_storage.get(&animation.played).expect(&animation.played);
                if let Some(frame) = animation_template.frames.get(animation.frame) {
                    sprite.rect.x += animation_template.move_by;
                    animation.frame_duration = frame.duration;
                } else if animation_template.repeat {
                    let frame = &animation_template.frames[0];
                    sprite.rect = animation_template.rect;
                    animation.frame = 0;
                    animation.frame_duration = frame.duration;
                } else {
                    animation.state = State::Finished;
                }
            }
        }
        State::Finished => {}
    }
}
