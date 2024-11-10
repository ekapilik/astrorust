use macroquad::prelude::*;

const FONT_COLOR: Color = WHITE;

const LINEAR_ACCELERATION: f32 = 300.0; // pixels per second squared
const ROTATIONAL_ACCELERATION: f32 = 0.10; // radians per second squared
const DRAG_COEFFICIENT: f32 = 0.99;

#[derive(Debug, Clone, Copy)]
struct Body {
    point: Vec2,
    velocity: Vec2,
    acceleration: Vec2,
    rotation: f32,
}

struct Shape {
    points: Vec<Vec2>,
    color: Color,
    thickness: f32,
}

impl Shape {
    fn transform(&self, center: Vec2, rotation: f32) -> Vec<Vec2> {
        let translated_points: Vec<Vec2> =
            self.points.iter().map(|point| *point + center).collect();
        let rotated_points: Vec<Vec2> = translated_points
            .iter()
            .map(|point| rotate_point(center, *point, rotation))
            .collect();
        return rotated_points;
    }

    fn draw(&self, center: Vec2, rotation: f32) {
        let transformed_points = self.transform(center, rotation);
        let mut prev_point = transformed_points.first().unwrap();
        let remaining_points = &transformed_points[1..];
        for point in remaining_points {
            draw_line(
                prev_point.x,
                prev_point.y,
                point.x,
                point.y,
                self.thickness,
                self.color,
            );
            prev_point = point;
        }
    }
}

struct SpaceShip {
    body: Body,
    ship_shape: Shape,
    flames_shape_base: Shape,
    flames_shape_extended: Shape,
    is_thrusting: bool,
}

fn rotate_point(base: Vec2, end: Vec2, rotation: f32) -> Vec2 {
    // https://math.stackexchange.com/questions/270194/how-to-find-the-vertices-angle-after-rotation
    let dx = end.x - base.x;
    let dy = end.y - base.y;
    let x = base.x + dx * rotation.cos() - dy * rotation.sin();
    let y = base.y + dx * rotation.sin() + dy * rotation.cos();
    return vec2(x, y);
}

impl SpaceShip {
    // constructor
    pub fn new(width: f32, height: f32, start_point: Vec2) -> SpaceShip {
        // triangle with indent
        let top_center = vec2(height / 2.0, 0.0);
        let bottom_right = vec2(-width / 2.0, height / 2.0);
        let bottom_center = vec2(-width / 4.0, 0.0);
        let bottom_left = vec2(-width / 2.0, -height / 2.0);
        // flames base
        let flames_base_left = vec2(-width * 1.2, -height / 2.0);
        let flames_base_center = vec2(-width * 0.5, 0.0);
        let flames_base_right = vec2(-width * 1.2, height / 2.0);
        // flames extended
        let flames_extended_left = vec2(-width * 1.5, -height / 4.0);
        let flames_extended_center = vec2(-width * 0.5, 0.0);
        let flames_extended_right = vec2(-width * 1.5, height / 4.0);

        SpaceShip {
            body: Body {
                rotation: 0.0,
                point: start_point,
                velocity: vec2(0.0, 0.0),
                acceleration: vec2(0.0, 0.0),
            },
            ship_shape: Shape {
                points: vec![
                    top_center,
                    bottom_right,
                    bottom_center,
                    bottom_left,
                    top_center,
                ],
                color: WHITE,
                thickness: 2.0,
            },
            flames_shape_base: Shape {
                points: vec![flames_base_left, flames_base_center, flames_base_right],
                color: RED,
                thickness: 2.0,
            },
            flames_shape_extended: Shape {
                points: vec![
                    flames_extended_left,
                    flames_extended_center,
                    flames_extended_right,
                ],
                color: YELLOW,
                thickness: 2.0,
            },
            is_thrusting: false,
        }
    }

    pub fn apply_thrust(&mut self, thrust: f32) {
        self.body.acceleration.x += thrust * self.body.rotation.cos();
        self.body.acceleration.y += thrust * self.body.rotation.sin();
    }

    pub fn rotate(&mut self, rotation: f32) {
        self.body.rotation = (self.body.rotation + rotation) % (2.0 * std::f32::consts::PI);
    }

    fn drag(&mut self) {
        self.is_thrusting = self.body.acceleration.length() > 0.0;

        self.body.acceleration *= 0.0;
        self.body.velocity *= DRAG_COEFFICIENT;
    }

    fn warp_around(&mut self) {
        let screen_width = screen_width();
        let screen_height = screen_height();
        if self.body.point.x > screen_width {
            self.body.point.x = 0.0;
        } else if self.body.point.x < 0.0 {
            self.body.point.x = screen_width;
        }
        if self.body.point.y > screen_height {
            self.body.point.y = 0.0;
        } else if self.body.point.y < 0.0 {
            self.body.point.y = screen_height;
        }
    }

    pub fn update(&mut self, dt: f32) {
        println!("--------------");
        println!("point: {:?}", self.body.point);
        println!("velocity: {:?}", self.body.velocity);
        println!("acceleration: {:?}", self.body.acceleration);
        println!("rotation: {:?}", self.body.rotation);

        self.body.velocity += self.body.acceleration * dt;
        self.body.point += self.body.velocity * dt;
        self.drag();
        self.warp_around();
    }

    pub fn render(&self) {
        self.ship_shape.draw(self.body.point, self.body.rotation);
        if self.is_thrusting {
            self.flames_shape_base
                .draw(self.body.point, self.body.rotation);
            if rand::gen_range(0, 100) < 66 {
                self.flames_shape_extended
                    .draw(self.body.point, self.body.rotation);
            }
        }
    }
}

struct PlayingInfo {
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
    draw_text("Main Menu", center_x, center_y, 30.0, FONT_COLOR);
    draw_text(
        "Press Enter to start",
        center_x - 30.0,
        center_y + 50.0,
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
            if is_key_released(KeyCode::Enter) {
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

            playing_info.space_ship.rotate(rotation);
            playing_info.space_ship.apply_thrust(thrust);
            // update space ship
            playing_info.space_ship.update(dt);

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
