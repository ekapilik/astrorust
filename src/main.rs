use macroquad::prelude::*;

const DEV_MODE: bool = true;

#[macroquad::main("Asteroids")]
async fn main() {
    loop {
        clear_background(BLACK);
        if DEV_MODE {
            draw_fps();
        }
        next_frame().await // Blocks until the next frame, used to control the game loop.
    }
}

fn draw_fps() {
    draw_text(&format!("FPS: {:.2}", get_fps()), 10.0, 20.0, 20.0, WHITE);
}
