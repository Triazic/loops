use crate::{rail_edge::RailEdge, xy::XY};

/** 
 *  a rail is a metaphor for a grind-rail from Ratchet and Clank that the algorithm can jump between
 *  a polygon
 */
pub struct Rail {
    pub id: i32,
    pub parent_rail_id: Option<i32>,
    pub edges: Vec<RailEdge>,
    pub child_rails: Vec<Rail>,
}