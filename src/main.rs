pub mod body;
pub mod game_render;
pub mod game_state_machine;
pub mod game_states;
pub mod screen_util;
pub mod shape;
pub mod space_ship;

use macroquad::prelude::*;

const DEV_MODE: bool = true;
const TARGET_FPS: f32 = 60.0;
const BACKGROUND_COLOR: Color = BLACK;

#[macroquad::main("Asteroids")]
async fn main() {
    let mut game_state = game_states::GameState::MainMenu;
    let frame_sleep_duration = std::time::Duration::from_secs_f32(1.0 / TARGET_FPS);

    loop {
        clear_background(BACKGROUND_COLOR);
        draw_fps();

        game_state_machine::update_game_state(&mut game_state);
        game_render::render(&game_state);

        std::thread::sleep(frame_sleep_duration);
        next_frame().await // Blocks until the next frame, used to control the game loop.
    }
}

fn draw_fps() {
    if DEV_MODE {
        draw_text(&format!("FPS: {:.2}", get_fps()), 10.0, 20.0, 20.0, WHITE);
    }
}
