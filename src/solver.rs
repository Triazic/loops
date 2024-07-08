use itertools::Itertools;

use crate::{line::line, rail::Rail, rail_edge::RailEdge, solver_types::{Direction, Jump, SolverState}, vector_basics::{add, multiply_scalar, normalise, project_point_onto_line, subtract}, xy::{xy, XY}};

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
        dest_edge_id: 3, // TODO hack
        dest_direction: state.seed_direction.clone(),
        source_edge_id: -1, // doesn't matter
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
                    dest_edge_id: 3,
                    dest_direction: Direction::Clockwise,
                    source_edge_id: edge.id, // doesn't matter
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
                    dest_edge_id: 3,
                    dest_direction: Direction::Clockwise,
                    source_edge_id: target_edge.id, // doesn't matter
                }
            }
    };

    Vec::from([
        seed_jump, 
        seed_escape_jump
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
    let seed_edge_id = seed_jumps[0].dest_edge_id; // TODO dogshit
    let iter_1 = get_jumps_atomic(&state, state.root_rail.id, &seed_point, seed_edge_id, &seed_direction, state.pipe_spacing);
    let forward_jump = iter_1.iter().find(|jump| jump.to_rail_id > jump.from_rail_id).expect("Ooops no forward jump"); // TODO dogshit
    let next_point = &forward_jump.dest_point;
    let next_rail_id = forward_jump.to_rail_id;
    let next_rail = state.get_rail_by_id(next_rail_id);
    let iter_2 = get_jumps_atomic(&state, next_rail.id, next_point, forward_jump.dest_edge_id, &forward_jump.dest_direction, state.pipe_spacing);
    let forward_jump = iter_2.iter().find(|jump| jump.to_rail_id > jump.from_rail_id);
    match forward_jump {
        None => {},
        Some(forward_jump) => {
            let next_point = &forward_jump.dest_point;
            let next_rail_id = forward_jump.to_rail_id;
            let next_rail = state.get_rail_by_id(next_rail_id);
            let iter_3 = get_jumps_atomic(&state, next_rail.id, next_point, forward_jump.dest_edge_id, &forward_jump.dest_direction, state.pipe_spacing);
            returner.extend(iter_3);
        },
    }

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

pub fn get_edge_by_id(rail:&Rail, edge_id:i32) -> &RailEdge {
    rail.edges.iter().find(|edge| edge.id == edge_id).expect(&format!("oops.. no edge found with id {}", edge_id))
}

pub fn get_edge_by_parent_edge_id(rail:&Rail, parent_edge_id:i32) -> &RailEdge {
    rail.edges.iter().find(|edge| edge.parent_edge_id.is_some() && edge.parent_edge_id.unwrap() == parent_edge_id).expect(&format!("oops.. no edge found with id {}", parent_edge_id))
}

pub fn reverse_direction(direction:&Direction) -> Direction {
    match direction {
        Direction::Clockwise => Direction::AntiClockwise,
        Direction::AntiClockwise => Direction::Clockwise,
    }
}

fn get_jumps_atomic(state: &SolverState, rail_id: i32, point:&XY, edge_id: i32, direction:&Direction, pipe_spacing: f64) -> Vec<Jump> {
    // let exit_jump = {
    //     let direction = 
    //     match(direction) {
    //         Direction::Clockwise => normalise(&subtract(&edge.a, &edge.b)), // anti-clockwise
    //         Direction::AntiClockwise => normalise(&subtract(&edge.b, &edge.a)), // clockwise
    //     };
    // }

    let rail = state.get_rail_by_id(rail_id);
    let edge = state.get_edge_by_id(edge_id);

    let no_rails = rail.child_rails.is_empty();
    if (no_rails) {
        // we only need to return an escape jump
        let parent_rail_id = rail.parent_rail_id.expect("how no parent_rail_id?");
        let escape_jump = match rail.parent_rail_id {
            None => None, // not sure what to do here... 
            Some(_) => 
                {
                    let rail_to_escape_from = rail;
                    let edge_to_escape_from = edge;
    
                    let rail_to_escape_to = 
                        match rail.parent_rail_id {
                            Some(parent_rail_id) => {
                                let parent_rail = state.get_rail_by_id(parent_rail_id);
                                Some(parent_rail)
                            },
                            None => None,
                        };
                    let edge_to_escape_to = 
                        match edge.parent_edge_id {
                            Some(parent_edge_id) => {
                                let parent_edge = state.get_edge_by_id(parent_edge_id);
                                Some(parent_edge)
                            },
                            None => None,
                        };
                    
                    let a = project_point_onto_edge(&edge_to_escape_from, point);
                    let b = project_point_onto_edge(edge_to_escape_to.unwrap_or(&edge), point);
        
                    // assume maintain current loop direction. therefore projection to find exit point is the opposite?
                    let direction_vec = 
                        match(direction) {
                            Direction::Clockwise => normalise(&subtract(&edge_to_escape_from.a, &edge_to_escape_from.b)), // anti-clockwise
                            Direction::AntiClockwise => normalise(&subtract(&edge_to_escape_from.b, &edge_to_escape_from.a)), // clockwise
                        };
        
                    let source_point = add(&a, &multiply_scalar(&direction_vec, pipe_spacing));
                    let dest_point = add(&b, &multiply_scalar(&direction_vec, pipe_spacing));
        
                    Some(Jump {
                        from_rail_id: rail_to_escape_from.id.clone(),
                        to_rail_id: rail_to_escape_to.unwrap().id,
                        source_point: source_point.clone(),
                        dest_point: dest_point.clone(), 
                        dest_edge_id: {
                            match edge_to_escape_to {
                                Some(edge) => edge.id,
                                None => -1,
                            }
                        },
                        dest_direction: reverse_direction(&direction),
                        source_edge_id: edge_to_escape_from.id,
                    })
                }
        };
        return [escape_jump].into_iter().filter_map(|x| x).collect_vec();
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
            let edge_id = edge_id;
            let edge = state.get_edge_by_id(edge_id);
            let next_rail = &rail.child_rails[0];

            let escape_jump = match rail.parent_rail_id {
                None => None, // not sure what to do here... 
                Some(_) => 
                    {
                        let rail_to_escape_from = next_rail;
                        let edge_to_escape_from = state.get_edge_by_parent_edge_id(edge_id);
        
                        let rail_to_escape_to = 
                            match rail.parent_rail_id {
                                Some(rail_id) => Some(state.get_rail_by_id(rail_id)),
                                None => None,
                            };
                        let edge_to_escape_to = 
                            match edge.parent_edge_id {
                                Some(parent_edge) => Some(state.get_edge_by_id(parent_edge)),
                                None => None,
                            };
                        
                        let a = project_point_onto_edge(&edge_to_escape_from, point);
                        let b = project_point_onto_edge(edge_to_escape_to.unwrap_or(&edge), point);
            
                        // assume maintain current loop direction. therefore projection to find exit point is the opposite?
                        let direction_vec = 
                            match(direction) {
                                Direction::Clockwise => normalise(&subtract(&edge_to_escape_from.a, &edge_to_escape_from.b)), // anti-clockwise
                                Direction::AntiClockwise => normalise(&subtract(&edge_to_escape_from.b, &edge_to_escape_from.a)), // clockwise
                            };
            
                        let source_point = add(&a, &multiply_scalar(&direction_vec, pipe_spacing));
                        let dest_point = add(&b, &multiply_scalar(&direction_vec, pipe_spacing));
            
                        Some(Jump {
                            from_rail_id: rail_to_escape_from.id.clone(),
                            to_rail_id: rail_to_escape_to.unwrap().id,
                            source_point: source_point.clone(),
                            dest_point: dest_point.clone(), 
                            dest_edge_id: {
                                match edge_to_escape_to {
                                    Some(edge) => edge.id,
                                    None => -1,
                                }
                            },
                            dest_direction: reverse_direction(direction),
                            source_edge_id: edge_to_escape_from.id,
                        })
                    }
            };

            let forward_jump = {
                let proposed_rail = next_rail;
                let next_edge = state.get_edge_by_parent_edge_id(edge_id);
                let proposed_edge = next_edge;
                
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
    
                Some(Jump {
                    from_rail_id: rail.id.clone(),
                    to_rail_id: proposed_rail.id.clone(),
                    source_point: e.clone(),
                    dest_point: d.clone(),
                    dest_edge_id: proposed_edge.id,
                    dest_direction: reverse_direction(direction),
                    source_edge_id: edge_id,
                })
            };

            return [
                forward_jump, 
                escape_jump
            ].into_iter().filter_map(|x| x).collect_vec();
        }
        if (depth > 1) {
            let edge_id = edge_id;
            let edge = state.get_edge_by_id(edge_id);
            let next_rail = &rail.child_rails[0];
            let next_next_rail = &next_rail.child_rails[0];

            let escape_jump = match rail.parent_rail_id {
                None => None, // not sure what to do here... 
                Some(_) => 
                    {
                        let rail_to_escape_from = next_rail;
                        let edge_to_escape_from = state.get_edge_by_parent_edge_id(edge_id);
        
                        let rail_to_escape_to = 
                            match rail.parent_rail_id {
                                Some(rail_id) => Some(state.get_rail_by_id(rail_id)),
                                None => None,
                            };
                        let edge_to_escape_to = 
                            match edge.parent_edge_id {
                                Some(parent_edge) => Some(state.get_edge_by_id(parent_edge)),
                                None => None,
                            };
                        
                        let a = project_point_onto_edge(&edge_to_escape_from, point);
                        let b = project_point_onto_edge(edge_to_escape_to.unwrap_or(&edge), point);
            
                        // assume maintain current loop direction. therefore projection to find exit point is the opposite?
                        let direction_vec = 
                            match(direction) {
                                Direction::Clockwise => normalise(&subtract(&edge_to_escape_from.a, &edge_to_escape_from.b)), // anti-clockwise
                                Direction::AntiClockwise => normalise(&subtract(&edge_to_escape_from.b, &edge_to_escape_from.a)), // clockwise
                            };
            
                        let source_point = add(&a, &multiply_scalar(&direction_vec, pipe_spacing));
                        let dest_point = add(&b, &multiply_scalar(&direction_vec, pipe_spacing));
            
                        Some(Jump {
                            from_rail_id: rail_to_escape_from.id.clone(),
                            to_rail_id: rail_to_escape_to.unwrap().id,
                            source_point: source_point.clone(),
                            dest_point: dest_point.clone(), 
                            dest_edge_id: {
                                match edge_to_escape_to {
                                    Some(edge) => edge.id,
                                    None => -1,
                                }
                            },
                            dest_direction: reverse_direction(&direction),
                            source_edge_id: edge_to_escape_from.id,
                        })
                    }
            };

            let forward_jump = {
                let proposed_rail = next_next_rail;
                let next_edge_id = state.get_edge_by_parent_edge_id(edge_id).id;
                let next_next_edge= state.get_edge_by_parent_edge_id(next_edge_id);
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
    
                Some(Jump {
                    from_rail_id: rail.id.clone(),
                    to_rail_id: proposed_rail.id.clone(),
                    source_point: e.clone(),
                    dest_point: d.clone(),
                    dest_edge_id: proposed_edge.id,
                    dest_direction: direction.clone(),
                    source_edge_id: edge_id,
                })
            };

            return [forward_jump, escape_jump].into_iter().filter_map(|x| x).collect_vec();
        }
        panic!("unhandled logical state");
    }

    panic!("unhandled logical state");
}