mod game;
mod physics;
mod render;
mod utils;

use std::thread::sleep;

use macroquad::prelude::*;

const TARGET_FPS: f64 = 60.0;
const DEV_MODE: bool = false;

#[macroquad::main("Asteroids")]
async fn main() {
    let mut game_state = game::core::game_states::GameState::MainMenu;
    loop {
        let start_time = get_time();
        game::core::game_state_machine::update_game_state(&mut game_state);
        let game_state_duration = get_time() - start_time;
        let render_start_time = get_time();
        game::core::game_render::render(&game_state);
        let render_duration = get_time() - render_start_time;

        let elapsed_time = get_time() - start_time;
        let target_frame_time = 1.0 / TARGET_FPS;
        let sleep_duration = target_frame_time - elapsed_time;

        if DEV_MODE {
            println!(
            "Game state duration: {:.2} us, Render duration: {:.2} us, Elapsed time: {:.2} us, Target frame time: {:.2} us",
            game_state_duration * 1000000.0,
            render_duration * 1000000.0,
            elapsed_time * 1000000.0,
            target_frame_time * 1000000.0,
        );
        }

        if sleep_duration > 0.0 {
            sleep(std::time::Duration::from_secs_f64(sleep_duration));
        }
        next_frame().await;
    }
}
