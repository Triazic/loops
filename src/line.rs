use crate::xy::XY;

#[derive(Clone)]
pub struct Line {
    pub point_on_line: XY,
    pub tangent: XY,
}

pub fn line(point_on_line:XY, tangent:XY) -> Line {
    Line { point_on_line, tangent }
}
