use macroquad::prelude::*;

#[derive(Debug, Clone)]
pub enum ScreenEdgeBehavior {
    Wrap,
    Bounce,
    Destroy,
}

#[derive(Debug, Clone)]
pub struct Body {
    pub point: Vec2,
    pub velocity: Vec2,
    pub acceleration: Vec2,
    pub rotation: f32,
    pub drag_coefficient: f32,
    pub screen_edge_behavior: ScreenEdgeBehavior,
    pub destroyed: bool,
}

impl Body {
    pub fn apply_thrust(&mut self, thrust: f32) {
        self.acceleration.x += thrust * self.rotation.cos();
        self.acceleration.y += thrust * self.rotation.sin();
    }

    pub fn rotate(&mut self, rotation: f32) {
        self.rotation = (self.rotation + rotation) % (2.0 * std::f32::consts::PI);
    }

    pub fn drag(&mut self) {
        self.acceleration *= 0.0;
        self.velocity *= self.drag_coefficient;
    }

    pub fn update(&mut self, dt: f32) {
        debug!("--------------");
        debug!("point: {:?}", self.point);
        debug!("velocity: {:?}", self.velocity);
        debug!("acceleration: {:?}", self.acceleration);
        debug!("rotation: {:?}", self.rotation);
        self.velocity += self.acceleration * dt;
        self.point += self.velocity * dt;
        self.drag();

        match self.screen_edge_behavior {
            ScreenEdgeBehavior::Wrap => self.warp_around(),
            ScreenEdgeBehavior::Bounce => self.bounce(),
            ScreenEdgeBehavior::Destroy => self.destroy(),
        }
    }

    fn warp_around(&mut self) {
        let screen_width = screen_width();
        let screen_height = screen_height();
        if self.point.x > screen_width {
            self.point.x = 0.0;
        } else if self.point.x < 0.0 {
            self.point.x = screen_width;
        }
        if self.point.y > screen_height {
            self.point.y = 0.0;
        } else if self.point.y < 0.0 {
            self.point.y = screen_height;
        }
    }

    fn bounce(&mut self) {
        let screen_width = screen_width();
        let screen_height = screen_height();
        if self.point.x > screen_width {
            self.point.x = screen_width;
            self.velocity.x *= -1.0;
        } else if self.point.x < 0.0 {
            self.point.x = 0.0;
            self.velocity.x *= -1.0;
        }
        if self.point.y > screen_height {
            self.point.y = screen_height;
            self.velocity.y *= -1.0;
        } else if self.point.y < 0.0 {
            self.point.y = 0.0;
            self.velocity.y *= -1.0;
        }
    }

    fn destroy(&mut self) {
        let screen_width = screen_width();
        let screen_height = screen_height();
        if self.point.x > screen_width
            || self.point.x < 0.0
            || self.point.y > screen_height
            || self.point.y < 0.0
        {
            self.destroyed = true;
        }
    }
}
