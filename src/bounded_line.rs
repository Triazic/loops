use crate::xy::XY;

#[derive(Clone)]
pub struct BoundedLine {
    pub a: XY,
    pub b: XY,
}

pub fn bounded_line(a:XY, b:XY) -> BoundedLine {
    BoundedLine { a, b }
}
