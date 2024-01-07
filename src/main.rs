mod shapes;
use crate::shapes::{circle::Circle, collisons::Collidable, rect::Rect};

fn main() {
    let rect = Rect::default();
    let rect2 = Rect::default();
    let circle = Circle {
        x: 0.0,
        y: 0.0,
        radius: 1.0,
    };
    let circle2 = Circle {
        x: 1.0,
        y: 1.0,
        radius: 4.0,
    };

    println!("{}", rect.collide(&rect2));
    println!("{}", circle.collide(&circle2));
    rect.collide(&circle);

    println!("{}", rect);
}
