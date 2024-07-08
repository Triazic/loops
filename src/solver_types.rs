use itertools::Itertools;

use crate::{rail::Rail, rail_edge::RailEdge, xy::XY};

#[derive(Clone, PartialEq, Debug)]
pub enum Direction {
    Clockwise,
    AntiClockwise
}

#[derive(Debug, Clone)]
pub struct Jump {
    /** if rail_id is -1, this counts as 'termination' */
    pub from_rail_id: i32,
    /** if rail_id is -1, this counts as 'termination' */
    pub to_rail_id: i32,
    pub source_point: XY,
    pub source_edge_id: i32,
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
        fn rec(rail:&Rail, rail_id: i32) -> &Rail {
            if (rail.id == rail_id) {
                return rail;
            }
            return rec(&rail.child_rails[0], rail_id); // dogshit
        }
        rec(&self.root_rail, rail_id)
    }
    pub fn get_edge_by_id(&self, edge_id: i32) -> &RailEdge {
        fn rec(rail:&Rail, edge_id: i32) -> &RailEdge {
            let res = rail.edges.iter().find(|edge| edge.id == edge_id);
            match res {
                Some(edge) => edge,
                None => rec(&rail.child_rails[0], edge_id) // dogshit
            }
        }
        rec(&self.root_rail, edge_id)
    }
    pub fn get_edge_index_by_id(&self, edge_id: i32) -> usize {
        fn rec(rail:&Rail, edge_id: i32) -> usize {
            let res = rail.edges.iter().find_position(|edge| edge.id == edge_id);
            match res {
                Some((index, _)) => index,
                None => rec(&rail.child_rails[0], edge_id) // dogshit
            }
        }
        rec(&self.root_rail, edge_id)
    }
    pub fn get_edge_by_parent_edge_id(&self, parent_edge_id: i32) -> &RailEdge {
        fn rec(rail:&Rail, parent_edge_id: i32) -> &RailEdge {
            let res = rail.edges.iter().find(|edge| edge.parent_edge_id.is_some() && edge.parent_edge_id.unwrap() == parent_edge_id);
            match res {
                Some(edge) => edge,
                None => rec(&rail.child_rails[0], parent_edge_id) // dogshit
            }
        }
        rec(&self.root_rail, parent_edge_id)
    }
}