use raylib::prelude::*;

use crate::{rail::Rail, rail_edge::RailEdge, test_cases::{test_case_square_1, test_case_square_2, test_case_square_3, test_case_square_4, test_case_square_5, test_case_square_6}, xy::{xy, XY}};

fn draw_line(d: &mut RaylibDrawHandle, screen_w: i32, screen_h: i32, 
    relative_screen_x1: f64, relative_screen_y1: f64, relative_screen_x2: f64, relative_screen_y2: f64,
    color: Color
) -> () {
    let x1 = ((screen_w as f64) * relative_screen_x1) as i32;
    let x2 = ((screen_w as f64) * relative_screen_x2) as i32;
    let y1 = ((screen_h as f64) * relative_screen_y1) as i32;
    let y2 = ((screen_h as f64) * relative_screen_y2) as i32;
    d.draw_line(x1, y1, x2, y2, color);
}

pub fn world_to_relative_screen(world_xy:&XY, screen_w: i32, screen_h: i32, min_world_x: f64, min_world_y: f64, max_world_x: f64, max_world_y: f64) -> XY {
    let relative_world_x = (world_xy.x - min_world_x) / (max_world_x - min_world_x);
    let relative_world_y = (world_xy.y - min_world_y) / (max_world_y - min_world_y);
    let relative_screen_x = relative_world_x; // draw from left to right
    let relative_screen_y = 1.0 - relative_world_y; // draws from top to bottom
    xy(relative_screen_x, relative_screen_y)
}

fn draw_rail_edge(d: &mut RaylibDrawHandle, screen_w: i32, screen_h: i32, edge: &RailEdge, min_world_x: f64, min_world_y: f64, max_world_x: f64, max_world_y: f64, color: Color) -> () {
    let a = &edge.a;
    let b = &edge.b;
    let a_screen  = world_to_relative_screen(a, screen_w, screen_h, min_world_x, min_world_y, max_world_x, max_world_y);
    let b_screen  = world_to_relative_screen(b, screen_w, screen_h, min_world_x, min_world_y, max_world_x, max_world_y);
    draw_line(d, screen_w, screen_h, a_screen.x, a_screen.y, b_screen.x, b_screen.y, color);
}

fn recursive_draw_rail(d: &mut RaylibDrawHandle, screen_w: i32, screen_h: i32, min_world_x: f64, min_world_y: f64, max_world_x: f64, max_world_y: f64, rail: &Rail, depth: i32) -> () {
    let color = {
        if (depth % 2 == 0) {
            Color::RED
        } else {
            Color::GREEN
        }
    };
    rail.edges.iter().for_each(|edge| {
        draw_rail_edge(d, screen_w, screen_h, edge, min_world_x, min_world_y, max_world_x, max_world_y, color);
    });
    rail.child_rails.iter().for_each(|child_rail| {
        recursive_draw_rail(d, screen_w, screen_h, min_world_x, min_world_y, max_world_x, max_world_y, child_rail, depth+1)
    })
}

pub fn raylib_main() {
    let test_data = test_case_square_1();
    let min_world_x = -0.2;
    let min_world_y = -0.2;
    let max_world_x = 1.2;
    let max_world_y = 1.2;

    // Initialize Raylib
    let (mut rl, thread) = raylib::init()
        .size(3840, 2160)
        .title("Render stuff")
        .build();
    rl.set_target_fps(60);

    while !rl.window_should_close() {
        let screen_w = rl.get_screen_width();
        let screen_h = rl.get_screen_height();
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::WHITE); 

        // draw screen calibration lines
        // draw_line(&mut d, screen_w, screen_h, 0.1, 0.1, 0.9, 0.9, Color::BLACK);
        // draw_line(&mut d, screen_w, screen_h, 0.1, 0.9, 0.9, 0.1, Color::RED);

        // draw rails
        recursive_draw_rail(&mut d, screen_w, screen_h, min_world_x, min_world_y, max_world_x, max_world_y, &test_data, 0);
    }
}