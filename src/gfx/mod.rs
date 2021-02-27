pub mod debug_info;

use crate::game::Game;
use crate::phx::Position;
use glam::Vec2;
use legion::IntoQuery;
use macroquad::camera::set_camera;
use macroquad::color::{Color, GRAY, WHITE};
use macroquad::math::Rect;
use macroquad::texture::{draw_texture_ex, DrawTextureParams};
use macroquad::window::clear_background;

pub struct Sprite {
    pub src: String,
    /// area of the texture to be drawn
    pub rect: Rect,
    /// offset from the location given by Position component, by default the center
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
            src: name,
            rect: Rect::new(x, y, width, height),
            offset: -Vec2::new(width, height) / 2.,
            color: WHITE,
            flip: false,
        }
    }
}

pub fn render(game: &Game) {
    clear_background(GRAY);
    set_camera(game.camera);

    let mut query = <(&Position, &Sprite)>::query();
    for (position, sprite) in query.iter(&game.world) {
        let texture = game.textures.get(&sprite.src).unwrap();
        let rect = sprite.rect;

        draw_texture_ex(
            *texture,
            position.src.x() + sprite.offset.x(),
            position.src.y() + sprite.offset.y(),
            sprite.color,
            DrawTextureParams { source: Some(rect), flip_x: sprite.flip, ..Default::default() },
        );
    }
    debug_info::visualize_hitboxes(&game.resources);
    debug_info::visualize_hurtboxes(&game.resources);
}
