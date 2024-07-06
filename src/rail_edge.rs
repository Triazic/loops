use std::iter::Enumerate;

use crate::xy::XY;

pub struct RailEdge {
    pub id: i32,
    pub parent_edge_id: Option<i32>,
    pub a: XY,
    pub b: XY,
}

pub fn rail_edge(a:XY, b: XY, id: i32, parent_edge_id: Option<i32>) -> RailEdge {
    RailEdge { id, parent_edge_id, a, b }
}