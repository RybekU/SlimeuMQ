use macroquad::*;
mod simplecam;

use simplecam::SimpleCam2D;

const GAME_SCALE: i32 = 4;

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

    let camera = SimpleCam2D::with_zoom(GAME_SCALE as f32);

    let mut middle_rotation = 0.;
    loop {
        clear_background(GRAY);
        set_camera(camera);

        middle_rotation += std::f32::consts::PI * get_frame_time();

        draw_texture(texture, 10., 10., WHITE);
        draw_texture_ex(
            texture,
            screen_width() / GAME_SCALE as f32 / 2. - texture.width() / 2.,
            screen_height() / GAME_SCALE as f32 / 2. - texture.height() / 2.,
            WHITE,
            DrawTextureParams {
                rotation: middle_rotation,
                dest_size: Some(Vec2::splat(texture.width() * 2.)),
                ..Default::default()
            },
        );
        next_frame().await
    }
}
