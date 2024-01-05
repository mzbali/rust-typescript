mod shapes;
use shapes::{Circle, Rect, Area};


fn main() {
    let rect = Rect {
        x: 0.0,
        y: 0.0,
        width: 10.0,
        height: 10.0,
    };

    let circ = Circle {
        x: 0.0,
        y: 0.0,
        radius: 10.0,
    };

    println!("{}", rect.area());
    println!("{}", circ.area());
    println!("{}", 6.0.area());
}
