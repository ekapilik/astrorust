use crate::physics::body::*;
use crate::render::shape::*;
use macroquad::prelude::*;

pub struct SpaceShip {
    pub body: Body,
    pub ship_shape: Shape,
    pub flames_shape_base: Shape,
    pub flames_shape_extended: Shape,
    pub is_thrusting: bool,
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
                drag_coefficient: 0.99,
                screen_edge_behavior: ScreenEdgeBehavior::Wrap,
                destoryed: false,
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
        self.body.apply_thrust(thrust);
        self.is_thrusting = thrust > 0.0;
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
