use raylib::prelude::*;

pub fn raylib_main() {
    // Initialize Raylib
    let (mut rl, thread) = raylib::init()
        .size(1000, 1000)
        .title("Draw 2D Lines")
        .build();

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::WHITE);

        // Draw lines
        d.draw_line(10, 10, 990, 990, Color::BLACK);
        d.draw_line(10, 990, 990, 10, Color::BLACK);
    }
}