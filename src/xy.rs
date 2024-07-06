#[derive(Clone)]
pub struct XY {
    pub x: f64,
    pub y: f64
}

pub fn xy(x: f64, y: f64) -> XY {
    XY { x, y }
}
