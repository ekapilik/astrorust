use crate::game_states::*;
use crate::screen_util::*;
use crate::space_ship::*;
use macroquad::prelude::*;

const LINEAR_ACCELERATION: f32 = 300.0; // pixels per second squared
const ROTATIONAL_ACCELERATION: f32 = 0.10; // radians per second squared

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
