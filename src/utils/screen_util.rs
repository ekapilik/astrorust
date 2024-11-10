use macroquad::prelude::*;

pub fn get_center_x() -> f32 {
    return (screen_width() / 2.0) - 60.0;
}

pub fn get_center_y() -> f32 {
    return screen_height() / 2.0;
}
