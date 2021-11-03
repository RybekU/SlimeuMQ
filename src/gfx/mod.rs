pub mod animation;

#[cfg(feature = "devtools")]
pub mod debug_info;

pub use self::animation::*;

pub type AnimationStorage = fxhash::FxHashMap<String, AnimationTemplate>;
pub type TextureStorage = fxhash::FxHashMap<String, macroquad::texture::Texture2D>;

use crate::game::Game;
use crate::phx::Position;
use crate::GAME_SCALE;

use glam::Vec2;
use legion::IntoQuery;
use macroquad::camera::set_camera;
use macroquad::color::{Color, GRAY, WHITE};
use macroquad::math::Rect;
use macroquad::texture::{draw_texture_ex, DrawTextureParams};
use macroquad::window::clear_background;

pub struct Sprite {
    pub texture: String,
    /// area of the texture to be drawn
    pub rect: Rect,
    /// offset from the location given by `Position` component, by default the center
    pub offset: Vec2,
    /// white for default
    pub color: Color,
    /// if true sprite faces left
    pub flip: bool,
}

impl Sprite {
    /// Offset is centered by default
    pub fn new(name: String, x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            texture: name,
            rect: Rect::new(x, y, width, height),
            offset: -Vec2::new(width, height) / 2.,
            color: WHITE,
            flip: false,
        }
    }
}

/// aligns position with the pixel grid of given game scale
pub fn align2subpixels(num: f32, game_scale: f32) -> f32 {
    let frac = num.fract();
    let num = num.trunc();
    num + (frac * game_scale).trunc() / game_scale
}

pub fn render(game: &Game) {
    clear_background(GRAY);
    set_camera(&game.camera);

    let mut query = <(&Position, &Sprite)>::query();
    for (position, sprite) in query.iter(&game.world) {
        let texture = game.textures.get(&sprite.texture).unwrap();
        let rect = sprite.rect;

        draw_texture_ex(
            *texture,
            align2subpixels(position.src.x() + sprite.offset.x(), GAME_SCALE as f32),
            align2subpixels(position.src.y() + sprite.offset.y(), GAME_SCALE as f32),
            sprite.color,
            DrawTextureParams { source: Some(rect), flip_x: sprite.flip, ..Default::default() },
        );
    }

    #[cfg(feature = "devtools")]
    {
        debug_info::visualize_colliders(&game.resources);
        debug_info::visualize_boxes(&game.resources);
    }
}
