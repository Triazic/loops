mod xy;
use num_traits::Num;

use xy::XY;

fn main() {
    let xy = XY::<f32> { x: 10.0, y: 10.0 };
    let res = do_something(xy);
    println!("Hello, world! {}", res);
}

fn do_something<T: Num>(xy: XY::<T>) -> T {
    xy.x + xy.y
}

fn do_another_thing<T: Num>(xy: XY::<T>) -> T {
    xy.x * xy.y
}
