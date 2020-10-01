use macroquad::*;
mod game;
mod gfx;
mod phx;
mod util;

pub const GAME_SCALE: i32 = 4;
pub const UPDATE_RATE: f32 = 60.;
pub const FRAMETIME: f32 = 1. / UPDATE_RATE;

fn window_conf() -> Conf {
    Conf {
        window_title: "Slimeu PreGreekAlphabet".to_owned(),
        window_width: 320 * GAME_SCALE,
        window_height: 180 * GAME_SCALE,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut game = game::Game::new();
    game.init().await;

    let mut update_timer = util::Timer::with_fps(UPDATE_RATE);
    loop {
        // handle events here

        while update_timer.tick() {
            // execute schedule here
            game.update()
        }

        {
            // unrestrained drawing
            crate::gfx::render(&game);
        }
        next_frame().await
    }
}
