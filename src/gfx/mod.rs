pub mod debug_info;

use crate::game::Game;
use crate::phx::Position;
use glam::Vec2;
use legion::IntoQuery;
use macroquad::{
    clear_background, draw_texture_ex, set_camera, DrawTextureParams, Rect, Texture2D, GRAY, WHITE,
};

pub struct Sprite {
    pub src: String,
    pub offset: Vec2,
}

impl Sprite {
    pub fn new(name: String, texture: &Texture2D) -> Self {
        Self {
            src: name,
            offset: -Vec2::new(texture.width(), texture.height()) / 2.,
        }
    }
}

pub fn render(game: &Game) {
    clear_background(GRAY);
    set_camera(game.camera);

    let mut query = <(&Position, &Sprite)>::query();
    for (position, sprite) in query.iter(&game.world) {
        let texture = game.textures.get(&sprite.src).unwrap();
        // draw_texture(*texture, (position.src.x() + sprite.offset.x()), (position.src.y() + sprite.offset.y()), WHITE);
        draw_texture_ex(
            *texture,
            position.src.x() + sprite.offset.x(),
            position.src.y() + sprite.offset.y(),
            WHITE,
            DrawTextureParams {
                source: Some(Rect::new(0., 0., 18., 18.)),
                ..Default::default()
            },
        );
    }
    debug_info::visualize_hitboxes(&game.resources);
}
