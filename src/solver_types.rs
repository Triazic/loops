use crate::xy::XY;

enum Direction {
    Clockwise,
    AntiClockwise
}

struct Jump {
    from_rail_id: i32,
    to_rail_id: i32,
    source_point: XY,
    dest_point: XY,
    dest_direction: Direction,
}