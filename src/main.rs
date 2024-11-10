mod game;
mod physics;
mod render;
mod utils;

use macroquad::prelude::*;

#[macroquad::main("Asteroids")]
async fn main() {
    let mut game_state = game::core::game_states::GameState::MainMenu;
    loop {
        game::core::game_state_machine::update_game_state(&mut game_state);
        game::core::game_render::render(&game_state);
        next_frame().await;
    }
}
