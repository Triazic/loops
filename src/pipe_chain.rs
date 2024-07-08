use crate::{bounded_line::{bounded_line, BoundedLine}, solver_types::{Direction, Jump, SolverState}, xy::XY};

pub fn get_pipe_chain(state: &SolverState, jumps: &Vec<Jump>) -> Vec<BoundedLine> {
    let mut returner = Vec::new();
    // seed forward jump
    let seed_forward_jump = jumps.iter().find(|jump| jump.to_rail_id == 0 && jump.from_rail_id == -1).expect("no seed_forward_jump?");
    let seed_forward_line = bounded_line(seed_forward_jump.source_point.clone(), seed_forward_jump.dest_point.clone());
    returner.push(seed_forward_line);

    fn rec(pipe_chain: &mut Vec<BoundedLine>, jumps: &Vec<Jump>, state: &SolverState, point: &XY, edge_id: i32, rail_id: i32, direction: &Direction, depth: i32) {
        let rail = state.get_rail_by_id(rail_id);
        let edge = state.get_edge_by_id(edge_id);

        // // exit condition can be that an edge has an escape jump on it
        let next_escape_jump = jumps.iter().find(|jump| jump.to_rail_id < jump.from_rail_id && jump.from_rail_id == rail_id);
        let next_forward_jump = jumps.iter().find(|jump| jump.to_rail_id > jump.from_rail_id && jump.from_rail_id == rail_id);

        if (next_escape_jump.is_none() && next_forward_jump.is_none()) {
            panic!("How is there neither an escape jump or a forward jump?");
        }
        // // true exit
        // if (next_forward_jump.is_none() && next_escape_jump.is_some() && next_escape_jump.unwrap().to_rail_id == -1) {
        //     return;
        // }
        match next_escape_jump {
            None => {

            },
            Some(next_escape_jump) => {
                if (depth > 0) {
                    let next_escape_jump_edge = next_escape_jump.source_edge_id;
                    if (edge_id == next_escape_jump_edge) {
                        // segment to the jump point
                        let a = point;
                        let b = &next_escape_jump.source_point;
                        let segment = bounded_line(a.clone(), b.clone());
                        pipe_chain.push(segment);
        
                        // the actual jump segment
                        let a = &next_escape_jump.source_point;
                        let b = &next_escape_jump.dest_point;
                        let segment = bounded_line(a.clone(), b.clone());
                        pipe_chain.push(segment);
                        
                        // true exit if we escape jumped onto rail -1
                        if (next_escape_jump.to_rail_id == -1) {
                            return;
                        }
                        rec(pipe_chain, jumps, state, &next_escape_jump.dest_point, next_escape_jump.dest_edge_id, next_escape_jump.to_rail_id, &next_escape_jump.dest_direction, 0);
                        return;
                    }
                }
            },
        }

        // exit condition can be that an edge has an forward jump on it
        match next_forward_jump {
            None => {

            },
            Some(next_forward_jump) => {
                if (depth > 0) {
                    let next_forward_jump_edge = next_forward_jump.source_edge_id;
                    if (edge_id == next_forward_jump_edge) {
                        // segment to the jump point
                        let a = point;
                        let b = &next_forward_jump.source_point;
                        let segment = bounded_line(a.clone(), b.clone());
                        pipe_chain.push(segment);
        
                        // the actual jump segment
                        let a = &next_forward_jump.source_point;
                        let b = &next_forward_jump.dest_point;
                        let segment = bounded_line(a.clone(), b.clone());
                        pipe_chain.push(segment);
        
                        rec(pipe_chain, jumps, state, &next_forward_jump.dest_point, next_forward_jump.dest_edge_id, next_forward_jump.to_rail_id, &next_forward_jump.dest_direction, 0);
                        return;
                    }
                }
            },
        }



        let a = point;
        let b = match direction {
            Direction::Clockwise => &edge.b,
            Direction::AntiClockwise => &edge.a,
        };
        let segment = bounded_line(a.clone(), b.clone());

        let edge_index = state.get_edge_index_by_id(edge_id);

        let n_edges = rail.edges.len();
        let next_edge_index = {
            match direction {
                Direction::Clockwise => {
                    if (edge_index+1 > n_edges - 1) {
                        0
                    }
                    else {
                        edge_index+1
                    }
                },
                Direction::AntiClockwise => {
                    if (edge_index==0) {
                        n_edges-1
                    }
                    else {
                        edge_index-1
                    }
                },
            }

        };
        let next_edge_id = rail.edges[next_edge_index].id;
        let next_point = b.clone();

        pipe_chain.push(segment);
        rec(pipe_chain, jumps, state, &next_point, next_edge_id, rail.id, direction, depth+1);
    }
    rec(&mut returner, &jumps, &state, &seed_forward_jump.dest_point, seed_forward_jump.dest_edge_id, seed_forward_jump.to_rail_id, &seed_forward_jump.dest_direction, 0);

    returner
}