use macroquad::prelude::Conf;
use macroquad::window::next_frame;

mod effect;
mod game;
mod gfx;
mod map;
mod phx;
mod util;

pub const GAME_SCALE: i32 = 4;
pub const UPDATE_RATE: f32 = 60.;
pub const FRAMETIME: f32 = 1. / UPDATE_RATE;

pub const GAME_DIMENSIONS: (i32, i32) = (320, 180);
// 384x216

fn window_conf() -> Conf {
    Conf {
        window_title: "Slimeu Early".to_owned(),
        window_width: GAME_DIMENSIONS.0 * GAME_SCALE,
        window_height: GAME_DIMENSIONS.1 * GAME_SCALE,
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

    let mut update_timer = util::FrameTimer::with_fps(UPDATE_RATE as f64);

    let mut histogram = util::timer::UpdateHistogram::new();
    let use_histogram = false;

    update_timer.time_snapping = false;
    update_timer.time_averaging = true;
    let mut resync = true;
    loop {
        update_timer.get_time();

        // read time only once per frame
        update_timer.process_elapsed();

        if resync {
            update_timer.resync();
            resync = false;
        }

        while update_timer.fuzzy_tick() {
            // execute schedule here
            game.update();
            histogram.register_update();
        }

        {
            // unrestrained drawing
            crate::gfx::render(&mut game);
        }

        if use_histogram {
            histogram.tick();
        }

        next_frame().await
    }
}
