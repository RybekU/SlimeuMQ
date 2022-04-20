use glam::Vec2;
use hecs::{Entity, World};
use macroquad::camera::Camera2D;

use crate::{gfx::align2subpixels, phx::Position, GAME_DIMENSIONS, GAME_SCALE};

pub struct Camera {
    src: Camera2D,
    pub target: Option<Entity>,

    cam_halfdim: Vec2,
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

        Self {
            src,
            target: None,
            cam_halfdim: Vec2::new(GAME_DIMENSIONS.0 as f32, GAME_DIMENSIONS.1 as f32) / 2.,
        }
    }

    pub fn update(&mut self, world: &World, current_room: &crate::game::stage::Room) {
        // get target location
        // set that to camera center
        if let Some(position) = self.target.and_then(|entity| world.get::<Position>(entity).ok()) {
            // respect room borders
            let loc_x = position
                .src
                .x
                .max(current_room.left + self.cam_halfdim.x)
                .min(current_room.right - self.cam_halfdim.x);
            let loc_y = position
                .src
                .y
                .max(current_room.top + self.cam_halfdim.y)
                .min(current_room.bottom - self.cam_halfdim.y);
            self.src.target = macroquad::prelude::Vec2::new(
                align2subpixels(loc_x, GAME_SCALE as f32),
                align2subpixels(loc_y, GAME_SCALE as f32),
            );
        }

        // TODO: interpolate current location with previous
        // TODO: read about the "box" <- camera moves only when player does a "significant" movement
    }

    pub fn src(&self) -> &Camera2D {
        &self.src
    }
}
