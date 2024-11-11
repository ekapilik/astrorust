use crate::game::components::bullet::*;
use crate::game::components::space_ship::*;
use crate::game::core::game_states::*;
use crate::utils::screen_util::*;
use macroquad::prelude::*;

const LINEAR_ACCELERATION: f32 = 300.0; // pixels per second squared
const ROTATIONAL_ACCELERATION: f32 = 0.10; // radians per second squared
const BULLET_VEL: f32 = 500.0;

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
                        bullets: vec![],
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
            let fire = is_key_released(KeyCode::Space);

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
            if fire {
                playing_info.bullets.push(Bullet::new(
                    playing_info.space_ship.body.point,
                    playing_info.space_ship.body.rotation,
                    BULLET_VEL,
                ));
            }

            playing_info.space_ship.body.rotate(rotation);
            playing_info.space_ship.apply_thrust(thrust);
            playing_info.space_ship.body.update(dt);

            playing_info
                .bullets
                .iter_mut()
                .for_each(|b| b.body.update(dt));
            playing_info.bullets.retain(|b| !b.body.destroyed);

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
