use crate::{rail::Rail, xy::XY};

pub enum Direction {
    Clockwise,
    AntiClockwise
}

pub struct Jump {
    /** if rail_id is -1, this counts as 'termination' */
    pub from_rail_id: i32,
    /** if rail_id is -1, this counts as 'termination' */
    pub to_rail_id: i32,
    pub source_point: XY,
    pub dest_point: XY,
    pub dest_direction: Direction,
}

pub struct SolverState {
    pub root_rail: Rail,
    pub seed_point: XY,
    pub seed_direction: Direction,
    pub pipe_spacing: f64,
}