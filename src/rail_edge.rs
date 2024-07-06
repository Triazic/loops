use std::iter::Enumerate;

use crate::xy::XY;

pub struct RailEdge {
    pub id: u16,
    pub parent_edge_id: Option<u16>,
    pub a: XY,
    pub b: XY,
}

pub fn rail_edge(a:XY, b: XY, id: u16, parent_edge_id: Option<u16>) -> RailEdge {
    RailEdge { id, parent_edge_id, a, b }
}