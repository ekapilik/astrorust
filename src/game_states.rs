use crate::space_ship::*;
use macroquad::prelude::*;

const FONT_COLOR: Color = WHITE;

const LINEAR_ACCELERATION: f32 = 300.0; // pixels per second squared
const ROTATIONAL_ACCELERATION: f32 = 0.10; // radians per second squared

pub struct PlayingInfo {
    score: u32,
    level: u32,
    space_ship: SpaceShip,
}

pub enum GameState {
    MainMenu,
    Playing { playing_info: PlayingInfo },
    GameOver,
}

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

fn get_center_x() -> f32 {
    return (screen_width() / 2.0) - 60.0;
}

fn get_center_y() -> f32 {
    return screen_height() / 2.0;
}

pub fn update_game_state(game_state: &mut GameState) {
    match game_state {
        GameState::MainMenu => {
            let start = is_key_released(KeyCode::Enter);
            let quit = is_key_released(KeyCode::Q);

            if start {
                *game_state = GameState::Playing {
                    playing_info: PlayingInfo {
                        score: 0,
                        level: 1,
                        space_ship: SpaceShip::new(
                            20.0,
                            20.0,
                            vec2(get_center_x(), get_center_y()),
                        ),
                    },
                };
            }
            if quit {
                std::process::exit(0);
            }
        }
        GameState::Playing { playing_info } => {
            let dt = get_frame_time();
            let rotate_left = is_key_down(KeyCode::Left) || is_key_down(KeyCode::A);
            let rotate_right = is_key_down(KeyCode::Right) || is_key_down(KeyCode::D);
            let thrust_forward = is_key_down(KeyCode::Up) || is_key_down(KeyCode::W);

            let mut rotation = 0.0;
            let mut thrust = 0.0;

            if rotate_left {
                rotation += ROTATIONAL_ACCELERATION;
            }
            if rotate_right {
                rotation -= ROTATIONAL_ACCELERATION;
            }
            if thrust_forward {
                thrust += LINEAR_ACCELERATION;
            }

            playing_info.space_ship.body.rotate(rotation);
            playing_info.space_ship.apply_thrust(thrust);
            playing_info.space_ship.body.update(dt);

            if is_key_released(KeyCode::Escape) {
                *game_state = GameState::MainMenu;
            }
        }
        GameState::GameOver => {
            if is_key_released(KeyCode::Enter) {
                *game_state = GameState::MainMenu;
            }
        }
    }
}
