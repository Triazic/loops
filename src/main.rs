mod xy;
mod vector_basics;
mod test_cases;
mod rail;
mod rail_edge;
use xy::XY;
fn main() {
    let xy = XY { x: 10.0, y: 10.0 };
    let res = do_something(xy);
    println!("Hello, world! {}", res);
}
fn do_something(xy: XY) -> f64 { xy.x + xy.y }
fn do_another_thing(xy: XY) -> f64 { xy.x * xy.y }
