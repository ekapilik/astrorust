use crate::game::components::space_ship::*;
use macroquad::prelude::*;

pub struct PlayingInfo {
    pub score: u32,
    pub level: u32,
    pub space_ship: SpaceShip,
}

pub enum GameState {
    MainMenu,
    Playing { playing_info: PlayingInfo },
    GameOver,
}
