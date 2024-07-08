#[cfg(test)]
mod tests {
    use loops::{pipe_chain::get_pipe_chain, raylib_structs::WorldBounds, solver::get_all_jumps, solver_types::Direction, test_cases::{test_case_square_4, test_case_square_5, test_case_square_6}};

    #[test]
    fn test() {
        let test_data = test_case_square_6();
        let jumps = get_all_jumps(&test_data);

        // make assertions about our jumps
        let forward_one = jumps.iter().find(|jump| {
            jump.source_edge_id == 3 && jump.dest_edge_id == 11
            && jump.from_rail_id == 0 && jump.to_rail_id == 2
            && jump.dest_direction == Direction::Clockwise
        });
        let forward_two = jumps.iter().find(|jump| {
            jump.source_edge_id == 11 && jump.dest_edge_id == 19
            && jump.from_rail_id == 2 && jump.to_rail_id == 4
            && jump.dest_direction == Direction::Clockwise
        });
        let forward_three = jumps.iter().find(|jump| {
            jump.source_edge_id == 19 && jump.dest_edge_id == 23
            && jump.from_rail_id == 4 && jump.to_rail_id == 5
            && jump.dest_direction == Direction::AntiClockwise
        });
        let escape_one = jumps.iter().find(|jump| {
            jump.source_edge_id == 23 && jump.dest_edge_id == 15
            && jump.from_rail_id == 5 && jump.to_rail_id == 3
            && jump.dest_direction == Direction::AntiClockwise
        });
        let escape_two = jumps.iter().find(|jump| {
            jump.source_edge_id == 15 && jump.dest_edge_id == 7
            && jump.from_rail_id == 3 && jump.to_rail_id == 1
            && jump.dest_direction == Direction::AntiClockwise
        });
        assert!(forward_one.is_some(), "no expected forward_one jump");
        assert!(forward_two.is_some(), "no expected forward_two jump");
        assert!(forward_three.is_some(), "no expected forward_three jump");
        assert!(escape_one.is_some(), "no expected escape_one jump");
        assert!(escape_two.is_some(), "no expected escape_two jump");
    }

    #[test]
    fn test_2() {
        let test_data = test_case_square_5();
        let jumps = get_all_jumps(&test_data);

        // make assertions about our jumps
        let forward_one = jumps.iter().find(|jump| {
            jump.source_edge_id == 3 && jump.dest_edge_id == 11
            && jump.from_rail_id == 0 && jump.to_rail_id == 2
            && jump.dest_direction == Direction::Clockwise
        });
        let forward_two = jumps.iter().find(|jump| {
            jump.source_edge_id == 11 && jump.dest_edge_id == 19
            && jump.from_rail_id == 2 && jump.to_rail_id == 4
            && jump.dest_direction == Direction::Clockwise
        });
        let escape_one = jumps.iter().find(|jump| {
            jump.from_rail_id == 4 && jump.to_rail_id == 3
            && jump.dest_direction == Direction::AntiClockwise
        });
        let escape_two = jumps.iter().find(|jump| {
            jump.from_rail_id == 3 && jump.to_rail_id == 1
            && jump.dest_direction == Direction::AntiClockwise
        });
        assert!(forward_one.is_some(), "no expected forward_one jump");
        assert!(forward_two.is_some(), "no expected forward_two jump");
        assert!(escape_one.is_some(), "no expected escape_one jump");
        assert!(escape_two.is_some(), "no expected escape_two jump");
    }

    #[test]
    fn test_3() {
        let test_data = test_case_square_4();
        let jumps = get_all_jumps(&test_data);

        // make assertions about our jumps
        let forward_one = jumps.iter().find(|jump| {
            jump.source_edge_id == 3 && jump.dest_edge_id == 11
            && jump.from_rail_id == 0 && jump.to_rail_id == 2
            && jump.dest_direction == Direction::Clockwise
        });
        let forward_two = jumps.iter().find(|jump| {
            jump.from_rail_id == 2 && jump.to_rail_id == 3
            && jump.dest_direction == Direction::AntiClockwise
        });
        let escape_one = jumps.iter().find(|jump| {
            jump.from_rail_id == 3 && jump.to_rail_id == 1
            && jump.dest_direction == Direction::AntiClockwise
        });
        let wrong_escape = jumps.iter().find(|jump| {
            jump.from_rail_id == 3 && jump.to_rail_id == 2
        });
        assert!(forward_one.is_some(), "no expected forward_one jump");
        assert!(forward_two.is_some(), "no expected forward_two jump");
        assert!(escape_one.is_some(), "no expected escape_one jump");
        assert!(escape_one.is_none(), "has wrong_escape");
    }
}