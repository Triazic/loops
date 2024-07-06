use crate::solver_types::{Jump, SolverState};

pub fn get_jumps(state: &SolverState) -> Vec<Jump> {
    // need to jump from seed point on to first rail
    let jump = Jump {
        from_rail_id: -1,
        to_rail_id: 0,
        source_point: state.seed_point.clone(),
        dest_point: todo!(),
        dest_direction: state.seed_direction,
    };

    Vec::from([jump])
}