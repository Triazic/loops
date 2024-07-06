use raylib::prelude::*;

use crate::test_cases::test_case_square_6;

fn draw_line(d: &mut RaylibDrawHandle, screen_w: i32, screen_h: i32, relative_x1: f64, relative_y1: f64, relative_x2: f64, relative_y2: f64) -> () {
    let x1 = ((screen_w as f64) * relative_x1) as i32;
    let x2 = ((screen_w as f64) * relative_x2) as i32;
    let y1 = ((screen_h as f64) * relative_y1) as i32;
    let y2 = ((screen_h as f64) * relative_y2) as i32;
    d.draw_line(x1, y1, x2, y2, Color::BLACK);
}

pub fn raylib_main() {
    let test_data = test_case_square_6();

    // Initialize Raylib
    let (mut rl, thread) = raylib::init()
        .size(3840, 2160)
        .title("Render stuff")
        .build();

    while !rl.window_should_close() {
        let screen_w = rl.get_screen_width();
        let screen_h = rl.get_screen_height();
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::WHITE);

        // Draw lines
        draw_line(&mut d, screen_w, screen_h, 0.1, 0.1, 0.9, 0.9);
        draw_line(&mut d, screen_w, screen_h, 0.1, 0.9, 0.9, 0.1);
    }
}