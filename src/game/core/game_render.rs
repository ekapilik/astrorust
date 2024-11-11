use crate::game::core::game_states::*;
use crate::utils::screen_util::*;
use macroquad::prelude::*;

const FONT_COLOR: Color = WHITE;
const BACKGROUND_COLOR: Color = BLACK;
const TARGET_FPS: f32 = 60.0;
const DEV_MODE: bool = true;

fn draw_fps() {
    if DEV_MODE {
        draw_text(&format!("FPS: {:.2}", get_fps()), 10.0, 20.0, 20.0, WHITE);
    }
}

pub fn render(game_state: &GameState) {
    clear_background(BACKGROUND_COLOR);
    draw_fps();
    match game_state {
        GameState::MainMenu => {
            render_main_menu();
        }
        GameState::Playing { playing_info } => {
            render_playing_info(playing_info);
            playing_info.space_ship.render();
            playing_info.bullets.iter().for_each(|bullet| {
                bullet.render();
            });
            playing_info.asteroids.iter().for_each(|asteroid| {
                asteroid.render();
            });
        }
        GameState::GameOver => {
            draw_text("Game Over", 10.0, 10.0, 30.0, FONT_COLOR);
        }
    }

    let frame_sleep_duration = std::time::Duration::from_secs_f32(1.0 / TARGET_FPS);
    std::thread::sleep(frame_sleep_duration);
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
