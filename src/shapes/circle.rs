use core::f64::consts::PI;

use super::{area::Area, collisons::Collidable, rect::Rect};

pub struct Circle {
    pub x: f64,
    pub y: f64,
    pub radius: f64,
}

impl Circle {
    pub fn contains_point(&self, (x, y): (f64, f64)) -> bool {
        let dx = self.x - x;
        let dy = self.y - y;

        return dx * dx - dy * dy <= self.radius * self.radius;
    }
}

impl Area for Circle {
    fn area(&self) -> f64 {
        return self.radius * self.radius * PI;
    }
}

impl Collidable<Rect> for Circle {
    fn collide(&self, other: &Rect) -> bool {
        return self.contains_point((other.x, other.y));
    }
}

impl Collidable<Circle> for Circle {
    fn collide(&self, other: &Circle) -> bool {
        return self.contains_point((other.x, other.y));
    }
}
