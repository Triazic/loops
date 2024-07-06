use crate::{line::Line, xy::XY};

pub fn magnitude(v: &XY) -> f64 {
    (v.x * v.x + v.y * v.y).sqrt()
}

pub fn normalise(v: &XY) -> XY {
    let mag = magnitude(v);
    XY {
        x: v.x / mag,
        y: v.y / mag,
    }
}

pub fn dot(v1: &XY, v2: &XY) -> f64 {
    v1.x * v2.x + v1.y * v2.y
}

pub fn get_angle_between_two_vectors(v1: &XY, v2: &XY) -> f64 {
    let _v1 = normalise(v1);
    let _v2 = normalise(v2);
    (dot(&_v1, &_v2)).acos()
}

pub fn add(a:&XY, b:&XY) -> XY {
    XY {x: a.x+b.x, y: a.y+b.y}
}

pub fn subtract(a:&XY, b:&XY) -> XY {
    XY {x: a.x-b.x, y: a.y-b.y}
}

pub fn divide_scalar(a:&XY, scalar: f64) -> XY {
    XY {x: a.x/scalar, y: a.y/scalar}
}

pub fn multiply_scalar(a:&XY, scalar: f64) -> XY {
    XY {x: a.x*scalar, y: a.y*scalar}
}

pub fn midpoint(ps:&Vec<&XY>) -> XY {
    let len = ps.len();
    if (len == 0) {
        panic!("No points supplied to 'midpoint'");
    }
    let sum = ps.iter().fold(XY { x: 0., y: 0. }, |acc, item| add(&acc, &item));
    divide_scalar(&sum, len as f64)
}


pub fn project(project_this: &XY, onto_this: &XY) -> f64 {
    // the dot product gives the projection, but too large by a factor of magnitude(vector to project onto)
    dot(project_this, onto_this) / magnitude(onto_this)
}

pub fn project_point_onto_line(point: &XY, line: &Line) -> XY {
    let oa = &line.point_on_line;
    let op = point;
    let ab = &line.tangent;
    let ap = subtract(op, oa);
    let ap_projected_onto_ab = project(&ap, ab);
    let ax = multiply_scalar(&normalise(ab), ap_projected_onto_ab);
    let ox = add(oa, &ax);
    ox
}
