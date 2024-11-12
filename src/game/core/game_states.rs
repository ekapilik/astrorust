use crate::game::components::asteroid::*;
use crate::game::components::bullet::*;
use crate::game::components::space_ship::*;
use macroquad::prelude::*;

pub struct PlayingInfo {
    pub score: u32,
    pub level: u32,
    pub space_ship: SpaceShip,
    pub bullets: Vec<Bullet>,
    pub asteroids: Vec<Asteroid>,
}

pub enum GameState {
    MainMenu,
    Playing { playing_info: PlayingInfo },
    NextLevel { level: u32, score: u32 },
    GameOver { level: u32, score: u32 },
}
