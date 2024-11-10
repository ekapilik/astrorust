use crate::game_states::*;
use crate::screen_util::*;
use macroquad::prelude::*;

const FONT_COLOR: Color = WHITE;

pub fn render(game_state: &GameState) {
    match game_state {
        GameState::MainMenu => {
            render_main_menu();
        }
        GameState::Playing { playing_info } => {
            render_playing_info(playing_info);
            playing_info.space_ship.render();
        }
        GameState::GameOver => {
            draw_text("Game Over", 10.0, 10.0, 30.0, FONT_COLOR);
        }
    }
}

fn render_main_menu() {
    let center_x = get_center_x();
    let center_y = get_center_y();
    draw_text("Main Menu", center_x, center_y - 80.0, 30.0, FONT_COLOR);
    draw_text(
        "Press Enter to start",
        center_x - 30.0,
        center_y + 0.0,
        20.0,
        WHITE,
    );
    draw_text(
        "Arrow keys or WASD to move",
        center_x - 60.0,
        center_y + 30.0,
        20.0,
        WHITE,
    );
    draw_text(
        "Press q to quit",
        center_x - 20.0,
        center_y + 60.0,
        20.0,
        WHITE,
    );
}

fn render_playing_info(playing_info: &PlayingInfo) {
    draw_text(
        &format!("Score: {}", playing_info.score),
        get_center_x(),
        20.0,
        30.0,
        FONT_COLOR,
    );

    draw_text(
        &format!("Level: {}", playing_info.level),
        screen_width() - 150.0,
        20.0,
        30.0,
        FONT_COLOR,
    );
}
