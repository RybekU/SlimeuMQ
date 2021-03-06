use macroquad::prelude::Conf;
use macroquad::window::next_frame;

mod effect;
mod game;
mod gfx;
mod phx;
mod util;

pub const GAME_SCALE: i32 = 4;
pub const UPDATE_RATE: f32 = 60.;
pub const FRAMETIME: f32 = 1. / UPDATE_RATE;

fn window_conf() -> Conf {
    Conf {
        window_title: "Slimeu Early".to_owned(),
        window_width: 320 * GAME_SCALE,
        window_height: 180 * GAME_SCALE,
        sample_count: 0,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    if cfg!(not(target_arch = "wasm32")) {
        simple_logger::SimpleLogger::new().init().expect("Logger failed");
    }
    let mut game = game::Game::new();
    game.init().await;

    let mut update_timer = util::Timer::with_fps(UPDATE_RATE);
    let mut current_time;
    loop {
        // read time only once per frame
        current_time = std::time::Instant::now();
        while update_timer.tick(&current_time) {
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
