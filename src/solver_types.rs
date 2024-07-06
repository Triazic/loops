use crate::{rail::Rail, rail_edge::RailEdge, xy::XY};

#[derive(Clone, PartialEq)]
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
    pub dest_edge_id: i32,
    pub dest_direction: Direction,
}

pub struct SolverState {
    pub root_rail: Rail,
    pub seed_point: XY,
    pub seed_direction: Direction,
    pub pipe_spacing: f64,
}

impl SolverState {
    pub fn get_rail_by_id(&self, rail_id: i32) -> &Rail {
        todo!("get_rail_by_id");
    }
    pub fn get_edge_by_id(&self, edge_id: i32) -> &RailEdge {
        todo!("get_edge_by_id");
    }
    pub fn get_edge_by_parent_edge_id(&self, parent_edge_id: i32) -> &RailEdge {
        todo!("get_edge_by_id");
    }
}