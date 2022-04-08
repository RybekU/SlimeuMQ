use hecs::{Entity, World};
use macroquad::camera::Camera2D;

use crate::{phx::Position, GAME_DIMENSIONS};

pub struct Camera {
    src: Camera2D,
    pub target: Option<Entity>,
    // rectangle
    // some other chase/interpolate parameters...?
}

impl Camera {
    pub fn new() -> Self {
        let src = Camera2D::from_display_rect(macroquad::math::Rect::new(
            0.0,
            0.0,
            GAME_DIMENSIONS.0 as f32,
            GAME_DIMENSIONS.1 as f32,
        ));

        Self { src, target: None }
    }

    pub fn update(&mut self, world: &World) {
        // get target location
        // set that to camera center
        if let Some(position) = self.target.and_then(|entity| world.get::<Position>(entity).ok()) {
            self.src.target = macroquad::prelude::Vec2::new(position.src.x, position.src.y);
        }

        // TODO: interpolate with current location with previous
        // TODO: read about the "box" <- camera moves only when player does a "significant" movement
        // TODO: pay respect to world borders
    }

    pub fn src(&self) -> &Camera2D {
        &self.src
    }
}
