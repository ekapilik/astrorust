use crate::physics::body::*;
use crate::render::shape::*;
use macroquad::prelude::*;

pub struct Bullet {
    pub body: Body,
    pub shape: Shape,
}

impl Bullet {
    pub fn new(start_point: Vec2, rotation: f32, velocity: f32) -> Bullet {
        Bullet {
            body: Body {
                rotation,
                point: start_point,
                velocity: vec2(velocity * rotation.cos(), velocity * rotation.sin()),
                acceleration: vec2(0.0, 0.0),
                drag_coefficient: 1.0,
                screen_edge_behavior: ScreenEdgeBehavior::Destroy,
                destroyed: false,
            },
            shape: Shape {
                points: vec![vec2(0.0, 0.0), vec2(5.0, 0.0)],
                color: Color::new(29.0 / 255.0, 240.0 / 255.0, 233.0 / 255.0, 1.0),
                thickness: 4.0,
            },
        }
    }

    pub fn render(&self) {
        self.shape.draw(self.body.point, self.body.rotation);
    }
}
