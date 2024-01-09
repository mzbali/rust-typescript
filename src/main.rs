mod shapes;
use std::{fmt::Display, str::FromStr};

use shapes::collisons::{Contains, PointIter, Points};

use crate::shapes::{circle::Circle, collisons::Collidable, rect::Rect};

enum Shape {
    Rect(Rect),
    Circ(Circle),
}

impl Display for Shape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Shape::Rect(rect) => write!(f, "{}", rect),
            Shape::Circ(circ) => write!(f, "{}", circ),
        }
    }
}

impl FromStr for Shape {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (shape, data) = s.split_once(" ").ok_or(anyhow::anyhow!("Invalid shape"))?;

        match shape {
            "rect" => return Ok(Shape::Rect(data.parse()?)),
            "circ" => return Ok(Shape::Circ(data.parse()?)),
            _ => Err(anyhow::anyhow!("Invalid Shape")),
        }
    }
}

impl Contains for Shape {
    fn contains_points(&self, point: (f64, f64)) -> bool {
        match self {
            Shape::Rect(rect) => return rect.contains_points(point),
            Shape::Circ(circ) => return circ.contains_points(point),
        }
    }
}

impl Points for &Shape {
    fn points(&self) -> PointIter {
        match self {
            Shape::Rect(rect) => return rect.points(),
            Shape::Circ(circ) => return circ.points(),
        }
    }
}

fn main() -> anyhow::Result<()> {
    let file = std::fs::read_to_string("shapes")?;
    let shapes = file
        .lines()
        .filter_map(|line| line.parse().ok())
        .collect::<Vec<Shape>>();
    shapes
        .iter()
        .skip(1)
        .zip(shapes.iter().take(shapes.len() - 1))
        .filter(|(a, b)| a.collide(b))
        .for_each(|(a, b)| println!("{} collides with {}", a, b));

    return Ok(());
}
