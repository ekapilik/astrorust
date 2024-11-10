use macroquad::prelude::Vec2;

#[derive(Debug, Clone, Copy)]
pub struct Body {
    pub point: Vec2,
    pub velocity: Vec2,
    pub acceleration: Vec2,
    pub rotation: f32,
}
