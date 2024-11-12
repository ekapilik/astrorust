use crate::game::components::asteroid::*;
use crate::game::components::bullet::*;
use crate::game::components::space_ship::*;
use crate::game::core::game_states::*;
use crate::physics::collision::point_in_polygon;
use crate::utils::screen_util::*;
use macroquad::prelude::*;

const LINEAR_ACCELERATION: f32 = 300.0; // pixels per second squared
const ROTATIONAL_ACCELERATION: f32 = 0.10; // radians per second squared
const BULLET_VEL: f32 = 500.0;
const DEV_MODE: bool = false;

pub fn update_game_state(game_state: &mut GameState) {
    let mut next_game_state: Option<GameState> = None;

    match game_state {
        GameState::MainMenu => {
            let start = is_key_released(KeyCode::Enter);
            let quit = is_key_released(KeyCode::Q);

            if start {
                next_game_state = Some(GameState::Playing {
                    playing_info: PlayingInfo {
                        score: 0,
                        level: 1,
                        space_ship: SpaceShip::new(
                            20.0,
                            20.0,
                            vec2(get_center_x(), get_center_y()),
                        ),
                        bullets: vec![],
                        asteroids: create_asteroids(1),
                    },
                });
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
            let escape = is_key_released(KeyCode::Escape);

            let mut next_level: bool = false;
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

            playing_info
                .asteroids
                .iter_mut()
                .for_each(|a| a.body.update(dt));

            let mut new_asteroids: Vec<Asteroid> = vec![];
            playing_info.asteroids.iter_mut().for_each(|a| {
                playing_info.bullets.iter_mut().for_each(|b| {
                    if a.shape.collides_with(
                        a.body.point,
                        a.body.rotation,
                        &b.shape,
                        b.body.point,
                        b.body.rotation,
                    ) {
                        a.body.destroyed = true;
                        b.body.destroyed = true;
                        playing_info.score += get_asteroid_score(&a.size);
                        new_asteroids.append(&mut split_asteroid(a));
                    }
                });
                if a.shape.collides_with(
                    a.body.point,
                    a.body.rotation,
                    &playing_info.space_ship.ship_shape,
                    playing_info.space_ship.body.point,
                    playing_info.space_ship.body.rotation,
                ) {
                    playing_info.space_ship.body.destroyed = true;
                }
            });

            if new_asteroids.len() > 0 {
                playing_info.asteroids.append(&mut new_asteroids);
            }
            playing_info.bullets.retain(|b| !b.body.destroyed); // remove destroyed bullets
            playing_info.asteroids.retain(|a| !a.body.destroyed); // remove destroyed asteroids

            if playing_info.asteroids.is_empty() {
                next_level = true;
            }

            if DEV_MODE {
                render_grid_points(&playing_info.asteroids);
            }

            if playing_info.space_ship.body.destroyed {
                next_game_state = Some(GameState::GameOver {
                    level: playing_info.level,
                    score: playing_info.score,
                });
            }
            if escape {
                next_game_state = Some(GameState::MainMenu);
            }
            if next_level {
                next_game_state = Some(GameState::NextLevel {
                    level: playing_info.level,
                    score: playing_info.score,
                });
            }
        }
        GameState::NextLevel { level, score } => {
            if is_key_released(KeyCode::Enter) {
                let next_level = *level + 1;
                next_game_state = Some(GameState::Playing {
                    playing_info: PlayingInfo {
                        score: *score,
                        level: next_level,
                        space_ship: SpaceShip::new(
                            20.0,
                            20.0,
                            vec2(get_center_x(), get_center_y()),
                        ),
                        bullets: vec![],
                        asteroids: create_asteroids(next_level),
                    },
                });
            }
        }
        GameState::GameOver { .. } => {
            if is_key_released(KeyCode::Enter) {
                next_game_state = Some(GameState::MainMenu);
            }
        }
    }
    if let Some(next_state) = next_game_state {
        *game_state = next_state;
    }
}

fn render_grid_points(asteroids: &Vec<Asteroid>) {
    let step = 5;
    let x_points = (0..(screen_width() as i32)).step_by(step);
    let y_points = (0..(screen_height() as i32)).step_by(step);

    for x in x_points {
        for y in y_points.clone() {
            let point = vec2(x as f32, y as f32);
            let mut point_in_asteroid = false;

            for asteroid in asteroids.iter() {
                let transformed_points = asteroid
                    .shape
                    .transform(asteroid.body.point, asteroid.body.rotation);
                if point_in_polygon(&point, &transformed_points) {
                    point_in_asteroid = true;
                    break;
                }
            }

            if point_in_asteroid {
                draw_circle(point.x, point.y, 2.0, RED);
            } else {
                // draw_circle(point.x, point.y, 1.0, WHITE);
            }
        }
    }
}
