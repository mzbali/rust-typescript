mod shapes;
use crate::shapes::rect::Rect;

fn main() {
    let rect = Rect::default();

    for point in &rect {
        println!("{:?}", point);
    }

    println!("{}", rect);
}
