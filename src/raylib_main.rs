use raylib::prelude::*;

use crate::{rail::Rail, rail_edge::RailEdge, raylib_structs::{ScreenDims, WorldBounds}, solver::{get_all_jumps, get_seed_jumps}, solver_types::{Direction, Jump, SolverState}, test_cases::{test_case_square_6}, vector_basics::midpoint, xy::{xy, XY}};

struct DrawContext<'a> {
    pub d: RaylibDrawHandle<'a>,
    pub screen_bounds: &'a ScreenDims,
    pub world_bounds: &'a WorldBounds,
}

fn draw_line(d: &mut RaylibDrawHandle, screen_bounds: &ScreenDims, 
    relative_screen_x1: f64, relative_screen_y1: f64, relative_screen_x2: f64, relative_screen_y2: f64,
    color: Color
) -> () {
    let x1 = ((screen_bounds.width as f64) * relative_screen_x1) as i32;
    let x2 = ((screen_bounds.width as f64) * relative_screen_x2) as i32;
    let y1 = ((screen_bounds.height as f64) * relative_screen_y1) as i32;
    let y2 = ((screen_bounds.height as f64) * relative_screen_y2) as i32;
    d.draw_line(x1, y1, x2, y2, color);
}

fn draw_circle(d: &mut RaylibDrawHandle, screen_bounds: &ScreenDims, 
    relative_screen_x1: f64, relative_screen_y1: f64,
    color: Color
) -> () {
    let x = ((screen_bounds.width as f64) * relative_screen_x1) as i32;
    let y = ((screen_bounds.height as f64) * relative_screen_y1) as i32;
    let radius = ((screen_bounds.width as f64) * 0.0025) as f32;
    d.draw_circle(x, y, radius, color);
}

fn draw_circle_world_co_ords(ctx: &mut DrawContext, a: &XY, color: Color) -> () {
    let a_screen  = world_to_relative_screen(ctx.screen_bounds, ctx.world_bounds, a);
    draw_circle(&mut ctx.d, ctx.screen_bounds, a_screen.x, a_screen.y, color);
}

fn draw_line_world_co_ords(ctx: &mut DrawContext, a: &XY, b: &XY, color: Color) -> () {
    let a_screen  = world_to_relative_screen(ctx.screen_bounds, ctx.world_bounds, a);
    let b_screen  = world_to_relative_screen(ctx.screen_bounds, ctx.world_bounds, b);
    draw_line(&mut ctx.d, ctx.screen_bounds, a_screen.x, a_screen.y, b_screen.x, b_screen.y, color);
}

fn draw_text(d: &mut RaylibDrawHandle, screen_bounds: &ScreenDims, 
    relative_screen_x: f64, relative_screen_y: f64, text: &str,
    color: Color
) -> () {
    let x = ((screen_bounds.width as f64) * relative_screen_x) as i32;
    let y = ((screen_bounds.height as f64) * relative_screen_y) as i32;
    let font_size = ((screen_bounds.height as f64) * 0.017) as i32;
    let text_width = d.measure_text(text, font_size);
    let text_height = d.measure_text("M", font_size);
    let text_x = (x - text_width/2);
    let text_y = (y - text_height/2);
    d.draw_text(&text, text_x, text_y, font_size, color);
}

pub fn world_to_relative_screen(screen_bounds: &ScreenDims, world_bounds: &WorldBounds, world_xy:&XY) -> XY {
    let relative_world_x = (world_xy.x - world_bounds.min_x) / (world_bounds.max_x - world_bounds.min_x);
    let relative_world_y = (world_xy.y - world_bounds.min_y) / (world_bounds.max_y - world_bounds.min_y);
    let relative_screen_x = relative_world_x; // draw from left to right
    let relative_screen_y = 1.0 - relative_world_y; // draws from top to bottom
    xy(relative_screen_x, relative_screen_y)
}

fn draw_rail_edge(ctx: &mut DrawContext, edge: &RailEdge, color: Color) -> () {
    let a = &edge.a;
    let b = &edge.b;
    draw_line_world_co_ords(ctx, a, b, color);
}

fn draw_rail_edge_id(ctx: &mut DrawContext, edge: &RailEdge, color: Color) -> () {
    let a = &edge.a;
    let b = &edge.b;
    let mid_point = midpoint(&Vec::from([a, b]));
    let screen  = world_to_relative_screen(ctx.screen_bounds, ctx.world_bounds, &mid_point);
    let text = 
        match &edge.parent_edge_id {
            None => format!("{:?}", &edge.id),
            Some(id) => format!("{:?} ({})", &edge.id, &id),
        };
    draw_text(&mut ctx.d, ctx.screen_bounds, screen.x, screen.y, &text, color)
}

fn recursive_draw_rail_edges(ctx: &mut DrawContext, rail: &Rail, depth: i32) -> () {
    let color = {
        if (depth % 2 == 0) {
            Color::RED
        } else {
            Color::GREEN
        }
    };
    rail.edges.iter().for_each(|edge| {
        draw_rail_edge(ctx, edge, color);
    });
    rail.child_rails.iter().for_each(|child_rail| {
        recursive_draw_rail_edges(ctx, child_rail, depth+1)
    })
}

fn recursive_draw_rail_edge_ids(ctx: &mut DrawContext, rail: &Rail, depth: i32) -> () {
    let color = {
        if (depth % 2 == 0) {
            Color::RED
        } else {
            Color::GREEN
        }
    };
    rail.edges.iter().for_each(|edge| {
        draw_rail_edge_id(ctx, edge, color);
    });
    rail.child_rails.iter().for_each(|child_rail| {
        recursive_draw_rail_edge_ids(ctx, child_rail, depth+1)
    })
}

fn draw_jumps(ctx: &mut DrawContext, jumps: &Vec<Jump>) -> () {
    jumps.iter().for_each(|jump| {
        let a = &jump.source_point;
        let b = &jump.dest_point;
        draw_line_world_co_ords(ctx, &a, &b, Color::BLUE);
        draw_circle_world_co_ords(ctx, &b, Color::BLUE);
    })
}

pub fn raylib_main() {
    let test_data = test_case_square_6();
    let world_bounds = WorldBounds {
        min_x: -0.2,
        min_y: -0.2,
        max_x: 1.2,
        max_y: 1.2,
    };
    let jumps = get_all_jumps(&test_data);

    // Initialize Raylib
    let (mut rl, thread) = raylib::init()
        .size(3840, 2160)
        .title("Render stuff")
        .build();
    rl.set_target_fps(60);
    rl.set_trace_log(TraceLogLevel::LOG_ERROR);

    while !rl.window_should_close() {
        let screen_bounds = ScreenDims {
            width: rl.get_screen_width(),
            height: rl.get_screen_height(),
        };
        let mut ctx = DrawContext {
            d: rl.begin_drawing(&thread),
            screen_bounds: &screen_bounds,
            world_bounds: &world_bounds,
        };

        ctx.d.clear_background(Color::WHITE); 

        // draw screen calibration lines
        // draw_line(&mut d, screen_w, screen_h, 0.1, 0.1, 0.9, 0.9, Color::BLACK);
        // draw_line(&mut d, screen_w, screen_h, 0.1, 0.9, 0.9, 0.1, Color::RED);

        // draw rail edges
        recursive_draw_rail_edges(&mut ctx, &test_data.root_rail, 0);
        recursive_draw_rail_edge_ids(&mut ctx, &test_data.root_rail, 0);

        // draw jumps
        draw_jumps(&mut ctx, &jumps);
    }
}