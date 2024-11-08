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

struct SpaceShip {
    body: Body,
    top_center: Vec2,
    bottom_right: Vec2,
    bottom_center: Vec2,
    bottom_left: Vec2,
    flames_left: Vec2,
    flames_right: Vec2,
    flames_center: Vec2,
    ship_color: Color,
    flame_color: Color,
    thickness: f32,
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
    pub fn new(x: f32, y: f32) -> SpaceShip {
        let width = 20.0;
        let height = 20.0;

        // triangle with indent
        let top_center = vec2(x, y - height / 2.0);
        let bottom_right = vec2(x + width / 2.0, y + height / 2.0);
        let bottom_center = vec2(x, y + height / 4.0);
        let bottom_left = vec2(x - width / 2.0, y + height / 2.0);

        SpaceShip {
            body: Body {
                rotation: 0.0,
                point: vec2(x, y),
                velocity: vec2(0.0, 0.0),
                acceleration: vec2(0.0, 0.0),
            },
            top_center,
            bottom_right,
            bottom_center,
            bottom_left,
            ship_color: WHITE,
            flame_color: RED,
            thickness: 2.0,
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
        self.body.acceleration *= 0.0;
        self.body.velocity *= DRAG_COEFFICIENT;
    }

    fn build_space_ship(&mut self) {
        let width = 20.0;
        let height = 20.0;

        // triangle with indent
        let top_center = vec2(self.body.point.x, self.body.point.y - height / 2.0);
        let bottom_right = vec2(
            self.body.point.x + width / 2.0,
            self.body.point.y + height / 2.0,
        );
        let bottom_center = vec2(self.body.point.x, self.body.point.y + height / 4.0);
        let bottom_left = vec2(
            self.body.point.x - width / 2.0,
            self.body.point.y + height / 2.0,
        );

        // flames
        let flames_left = vec2(self.body.point.x - width / 2.0, self.body.point.y + height);
        let flames_right = vec2(self.body.point.x + width / 2.0, self.body.point.y + height);
        let flames_center = vec2(self.body.point.x, self.body.point.y + height + 10.0);

        let rotation = self.body.rotation + std::f32::consts::PI / 2.0;
        self.top_center = rotate_point(self.body.point, top_center, rotation);
        self.bottom_right = rotate_point(self.body.point, bottom_right, rotation);
        self.bottom_center = rotate_point(self.body.point, bottom_center, rotation);
        self.bottom_left = rotate_point(self.body.point, bottom_left, rotation);
        self.flames_left = rotate_point(self.body.point, flames_left, rotation);
        self.flames_right = rotate_point(self.body.point, flames_right, rotation);
        self.flames_center = rotate_point(self.body.point, flames_center, rotation);
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
        self.build_space_ship();
    }

    pub fn draw(&self) {
        draw_line(
            self.top_center.x,
            self.top_center.y,
            self.bottom_right.x,
            self.bottom_right.y,
            self.thickness,
            self.ship_color,
        );
        draw_line(
            self.bottom_right.x,
            self.bottom_right.y,
            self.bottom_center.x,
            self.bottom_center.y,
            self.thickness,
            self.ship_color,
        );
        draw_line(
            self.bottom_center.x,
            self.bottom_center.y,
            self.bottom_left.x,
            self.bottom_left.y,
            self.thickness,
            self.ship_color,
        );
        draw_line(
            self.bottom_left.x,
            self.bottom_left.y,
            self.top_center.x,
            self.top_center.y,
            self.thickness,
            self.ship_color,
        );
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
            playing_info.space_ship.draw();
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
                        space_ship: SpaceShip::new(get_center_x(), get_center_y()),
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
