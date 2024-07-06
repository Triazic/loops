use crate::xy::XY;

fn magnitude(v: &XY) -> f64 {
    (v.x * v.x + v.y * v.y).sqrt()
}

fn normalize(v: &XY) -> XY {
    let mag = magnitude(v);
    XY {
        x: v.x / mag,
        y: v.y / mag,
    }
}

fn dot(v1: &XY, v2: &XY) -> f64 {
    v1.x * v2.x + v1.y * v2.y
}

fn get_angle_between_two_vectors(v1: &XY, v2: &XY) -> f64 {
    let _v1 = normalize(v1);
    let _v2 = normalize(v2);
    (dot(&_v1, &_v2)).acos()
}

pub fn add(a:&XY, b:&XY) -> XY {
    XY {x: a.x+b.x, y: a.y+b.y}
}

pub fn divide_scalar(a:&XY, scalar: f64) -> XY {
    XY {x: a.x/scalar, y: a.y/scalar}
}

pub fn midpoint(ps:&Vec<&XY>) -> XY {
    let len = ps.len();
    if (len == 0) {
        panic!("No points supplied to 'midpoint'");
    }
    let sum = ps.iter().fold(XY { x: 0., y: 0. }, |acc, item| add(&acc, &item));
    divide_scalar(&sum, len as f64)
}
