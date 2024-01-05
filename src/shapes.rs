use std::f64::consts::PI;

pub struct Rect {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}
pub struct Circle {
    pub x: f64,
    pub y: f64,
    pub radius: f64,
}

pub trait Area {
    fn area(&self) -> f64;
}

impl Area for Rect {
    fn area(&self) -> f64 {
        return self.width * self.height;
    }
}

impl Area for Circle {
    fn area(&self) -> f64 {
        return self.radius * self.radius * PI;
    }
}

impl Area for f64 {
    fn area(&self) -> f64 {
        return self * self;
    }
}
