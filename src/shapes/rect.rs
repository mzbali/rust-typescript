use std::fmt::Display;

use super::{
    area::Area,
    collisons::{Contains, Points},
};

pub struct Rect {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

impl Rect {
    pub fn contains_point(&self, (x, y): (f64, f64)) -> bool {
        return x >= self.x && x <= self.x + self.width && y >= self.y && y <= self.y + self.height;
    }
}

impl Contains for Rect {
    fn contains_points(&self, point: (f64, f64)) -> bool {
        return self.contains_point(point);
    }
}

impl Points for Rect {
    fn points(&self) -> super::collisons::PointIter {
        return vec![
            (self.x, self.y),
            (self.x + self.width, self.y),
            (self.x, self.y + self.height),
            (self.x + self.width, self.y + self.height),
        ]
        .into();
    }
}

impl Area for Rect {
    fn area(&self) -> f64 {
        return self.width * self.height;
    }
}

impl Default for Rect {
    fn default() -> Self {
        return Rect {
            x: 0.0,
            y: 0.0,
            width: 10.0,
            height: 10.0,
        };
    }
}

impl Display for Rect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(
            f,
            "Rectangle({},{}): {}x{}",
            self.x, self.y, self.width, self.height
        );
    }
}
