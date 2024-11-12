use std::vec;

use crate::physics::body::*;
use crate::render::shape::*;
use macroquad::prelude::*;
use macroquad::rand::gen_range as random_range;

pub struct Asteroid {
    pub body: Body,
    pub shape: Shape,
}

impl Asteroid {
    pub fn new(start_point: Vec2, rotation: f32, velocity: f32, scale: f32) -> Asteroid {
        let points = vec![
            vec2(-3.0, -2.0) * scale,
            vec2(-1.26, -2.76) * scale,
            vec2(-0.28, -1.88) * scale,
            vec2(2.28, -2.48) * scale,
            vec2(3.66, 0.0) * scale,
            vec2(0.44, 1.24) * scale,
            vec2(1.78, 1.56) * scale,
            vec2(0.58, 2.9) * scale,
            vec2(-1.18, 2.22) * scale,
            vec2(-2.26, 3.04) * scale,
            vec2(-3.54, 1.66) * scale,
            vec2(-2.68, 0.34) * scale,
            vec2(-3.0, -2.0) * scale,
        ];

        Asteroid {
            body: Body {
                rotation,
                point: start_point,
                velocity: vec2(velocity * rotation.cos(), velocity * rotation.sin()),
                acceleration: vec2(0.0, 0.0),
                drag_coefficient: 1.0,
                screen_edge_behavior: ScreenEdgeBehavior::Wrap,
                destroyed: false,
            },
            shape: Shape {
                points,
                color: WHITE,
                thickness: 2.0,
            },
        }
    }

    pub fn render(&self) {
        self.shape.draw(self.body.point, self.body.rotation);
    }
}

pub fn create_asteroids(level: u32) -> Vec<Asteroid> {
    let mut asteroids: Vec<Asteroid> = vec![];
    for _ in 0..level {
        let start_point = vec2(
            random_range(0.0, screen_width()),
            random_range(0.0, screen_height()),
        );
        let rotation = random_range(0.0, 2.0 * std::f32::consts::PI);
        let velocity = 50.0;
        let scale = 20.0;
        asteroids.push(Asteroid::new(start_point, rotation, velocity, scale));
    }
    asteroids
}
