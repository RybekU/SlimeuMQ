use macroquad::*;
mod game;
mod gfx;
mod phx;
mod util;

pub const GAME_SCALE: i32 = 4;
pub const UPDATE_RATE: f32 = 60.;

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
    let texture: Texture2D = load_texture("media/slimeu_base.png").await;
    set_texture_filter(texture, drawing::FilterMode::Nearest);

    let mut game = game::Game::new();
    game.textures.insert("slimeu_base".into(), texture);

    game.world.push((
        phx::Position {
            src: Vec2::new(10.0, 10.0),
        },
        gfx::Sprite::new("slimeu_base".to_owned(), &texture),
    ));
    game.world.push((
        phx::Position {
            src: Vec2::new(30.0, 10.0),
        },
        gfx::Sprite::new("slimeu_base".to_owned(), &texture),
    ));
    game.world.push((
        phx::Position {
            src: Vec2::new(50.0, 10.0),
        },
        gfx::Sprite::new("slimeu_base".to_owned(), &texture),
    ));

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
