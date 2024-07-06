use crate::xy::XY;

pub fn magnitude(xy: XY) -> f64 {
    f64::sqrt(xy.x * xy.x + xy.y * xy.y)
}

pub fn divide_scalar(xy: XY, scalar: f64) -> XY {
    XY { x: xy.x / scalar, y: xy.y / scalar }
}

pub fn blah(xy: XY) -> f64 {
    2.0
}
