use crate::{line::line, rail::Rail, rail_edge::RailEdge, solver_types::{Direction, Jump, SolverState}, vector_basics::{add, multiply_scalar, normalise, project_point_onto_line, subtract}, xy::XY};

/** dogshit code */
pub fn get_seed_jumps(state: &SolverState) -> Vec<Jump> {
    // need to jump from seed point on to first rail
    let seed_edge_index = 3;  // TODO: hack
    let dest_point = {
        // seed edge
        let edge = &state.root_rail.edges[seed_edge_index]; // TODO: hack
        let edge_as_line = line(edge.a.clone(), subtract(&edge.a, &edge.b));
        project_point_onto_line(&state.seed_point, &edge_as_line)
    };

    let seed_jump = Jump {
        from_rail_id: -1,
        to_rail_id: 0,
        source_point: state.seed_point.clone(),
        dest_point: dest_point,
        dest_edge_index: seed_edge_index,
        dest_direction: state.seed_direction.clone(),
    };

    let seed_escape_jump = {
        let exactly_one_rail = state.root_rail.child_rails.is_empty();
            // exit point will be from the root rail, at a position equal to pipe spacing, in the opposite direction of seed direction
            // rail edges are ordered in clockwise direction, so we know that to go anti-clockwise we can go from seed index, backwards one
            let edge = &state.root_rail.edges[seed_edge_index];
            let direction = 
                match(state.seed_direction) {
                    Direction::Clockwise => normalise(&subtract(&edge.a, &edge.b)), // anti-clockwise
                    Direction::AntiClockwise => normalise(&subtract(&edge.b, &edge.a)), // clockwise
                };
            let a = &seed_jump.dest_point;
            let b = add(&a, &multiply_scalar(&direction, state.pipe_spacing));
            let c = add(&state.seed_point, &multiply_scalar(&direction, state.pipe_spacing));
            if (exactly_one_rail) {
                Jump {
                    from_rail_id: 0,
                    to_rail_id: -1,
                    source_point: b,
                    dest_point: c,
                    dest_edge_index: 3,
                    dest_direction: Direction::Clockwise, // doesn't matter
                }
            }
            else {
                // project onto the second rail... 
                let target_edge = &state.root_rail.child_rails[0].edges[3]; // TODO: hack
                let edge_as_line = line(target_edge.a.clone(), subtract(&target_edge.a, &target_edge.b));
                let d = project_point_onto_line(&b, &edge_as_line);
                Jump {
                    from_rail_id: 1,
                    to_rail_id: -1,
                    source_point: d,
                    dest_point: c,
                    dest_edge_index: 3,
                    dest_direction: Direction::Clockwise, // doesn't matter
                }
            }
    };

    Vec::from([
        seed_jump, 
        // seed_escape_jump
    ])
}

pub fn project_point_onto_edge(edge:&RailEdge, point:&XY) -> XY {
    let edge_as_line = line(edge.a.clone(), subtract(&edge.b, &edge.a));
    project_point_onto_line(point, &edge_as_line)
}

pub fn get_all_jumps(state: &SolverState) -> Vec<Jump> {
    let mut returner: Vec<Jump> = Vec::new();
    let seed_jumps = get_seed_jumps(state);

    let seed_point = &seed_jumps[0].dest_point; // TODO dogshit
    let seed_direction = &seed_jumps[0].dest_direction; // TODO dogshit
    let seed_edge_index = seed_jumps[0].dest_edge_index; // TODO dogshit
    let iter_1 = get_jumps_atomic(&state.root_rail, &seed_point, seed_edge_index, &seed_direction, state.pipe_spacing);
    let forward_jump = iter_1.iter().find(|jump| jump.to_rail_id > jump.from_rail_id).expect("Ooops no forward jump"); // TODO dogshit
    let next_point = &forward_jump.dest_point;
    let next_rail = &state.root_rail.child_rails[0]; // todo dogshit
    let iter_2 = get_jumps_atomic(next_rail, next_point, forward_jump.dest_edge_index, &forward_jump.dest_direction, state.pipe_spacing);

    returner.extend(seed_jumps);
    returner.extend(iter_1);
    returner.extend(iter_2);
    returner
}

pub fn get_rail_depth(rail:&Rail) -> i32 {
    pub fn get_rail_depth_rec(rail:&Rail, depth:i32) -> i32 {
        let no_rails = rail.child_rails.is_empty();
        if (no_rails) {
            return depth;
        }
        let more_than_one_rail = rail.child_rails.len() > 1;
        if (more_than_one_rail) {
            panic!("haven't handled multiple rails");
        }
        let one_rail = rail.child_rails.len() == 1;
        if (one_rail) {
            return get_rail_depth_rec(&rail.child_rails[0], depth+1);
        }
        panic!("unhandled logical state");
    }
    get_rail_depth_rec(rail, 0)
}

pub fn get_edge_by_id(rail:&Rail, edge_id:u16) -> &RailEdge {
    rail.edges.iter().find(|edge| edge.id == edge_id).expect(&format!("oops.. no edge found with id {}", edge_id))
}

pub fn get_edge_by_parent_edge_id(rail:&Rail, parent_edge_id:u16) -> &RailEdge {
    rail.edges.iter().find(|edge| edge.parent_edge_id.is_some() && edge.parent_edge_id.unwrap() == parent_edge_id).expect(&format!("oops.. no edge found with id {}", parent_edge_id))
}

fn get_jumps_atomic(rail:&Rail, point:&XY, edge_index: usize, direction:&Direction, pipe_spacing: f64) -> Vec<Jump> {
    // let exit_jump = {
    //     let direction = 
    //     match(direction) {
    //         Direction::Clockwise => normalise(&subtract(&edge.a, &edge.b)), // anti-clockwise
    //         Direction::AntiClockwise => normalise(&subtract(&edge.b, &edge.a)), // clockwise
    //     };
    // }

    let no_rails = rail.child_rails.is_empty();
    if (no_rails) {
        panic!("haven't handled no rails");
    }
    let more_than_one_rail = rail.child_rails.len() > 1;
    if (more_than_one_rail) {
        panic!("haven't handled multiple rails");
    }
    let one_rail = rail.child_rails.len() == 1;
    if (one_rail) {
        let depth = get_rail_depth(rail);
        if (depth == 0) {
            todo!("havent handled depth of 0");
        }
        if (depth == 1) {
            todo!("havent handled depth of 1");
        }
        if (depth > 1) {
            let edge = &rail.edges[edge_index];
            let edge_id = edge.id;
            let next_rail = &rail.child_rails[0];
            let next_next_rail = &next_rail.child_rails[0];

            let escape_jump = {
                let proposed_rail = next_rail;
                let next_edge_id = get_edge_by_parent_edge_id(next_rail, edge_id);
                let proposed_edge = next_edge_id;
                
                // project our current point onto the proposed edge
                let a = project_point_onto_edge(&proposed_edge, point);
    
                // assume maintain current loop direction. therefore projection to find exit point is the opposite?
                let direction_vec = 
                    match(direction) {
                        Direction::Clockwise => normalise(&subtract(&proposed_edge.a, &proposed_edge.b)), // anti-clockwise
                        Direction::AntiClockwise => normalise(&subtract(&proposed_edge.b, &proposed_edge.a)), // clockwise
                    };
    
                let b = add(&a, &multiply_scalar(&direction_vec, pipe_spacing));
                let c = add(&point, &multiply_scalar(&direction_vec, pipe_spacing));
    
                Jump {
                    from_rail_id: proposed_rail.id.clone(),
                    to_rail_id: rail.id.clone(),
                    source_point: c.clone(), 
                    dest_point: b.clone(),
                    dest_edge_index: 0, // this is wrong
                    dest_direction: direction.clone(),
                }
            };

            let forward_jump = {
                let proposed_rail = next_next_rail;
                let next_edge_id = get_edge_by_parent_edge_id(next_rail, edge_id).id;
                let next_next_edge= get_edge_by_parent_edge_id(next_next_rail, next_edge_id);
                let proposed_edge = next_next_edge;
                
                // project our current point onto the proposed edge
                let a = project_point_onto_edge(&proposed_edge, point);
    
                // assume maintain current loop direction. therefore projection to find exit point is the opposite?
                let direction_vec = 
                    match(direction) {
                        Direction::Clockwise => normalise(&subtract(&proposed_edge.a, &proposed_edge.b)), // anti-clockwise
                        Direction::AntiClockwise => normalise(&subtract(&proposed_edge.b, &proposed_edge.a)), // clockwise
                    };

                let d = add(&a, &multiply_scalar(&direction_vec, pipe_spacing*2.));
                let e = add(&point, &multiply_scalar(&direction_vec, pipe_spacing*2.));
    
                Jump {
                    from_rail_id: rail.id.clone(),
                    to_rail_id: proposed_rail.id.clone(),
                    source_point: e.clone(),
                    dest_point: d.clone(),
                    dest_edge_index: 0, // this is wrong
                    dest_direction: direction.clone(),
                }
            };

            return Vec::from([forward_jump, escape_jump]);
        }
        panic!("unhandled logical state");
    }

    panic!("unhandled logical state");
}