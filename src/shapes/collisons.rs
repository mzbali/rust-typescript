pub trait Collidable<T> {
    fn collide(&self, other: &T) -> bool;

    fn collides(&self, others: &[T]) -> bool {
        for other in others {
            if self.collide(other) {
                return true;
            }
        }
        return false;
    }
}

pub struct PointIter {
    points: Vec<(f64, f64)>,
    idx: usize,
}

impl Iterator for PointIter {
    type Item = (f64, f64);
    fn next(&mut self) -> Option<Self::Item> {
        let idx = self.idx;
        self.idx += 1;

        return self.points.get(idx).map(|x| *x);
    }
}

impl From<Vec<(f64, f64)>> for PointIter {
    fn from(points: Vec<(f64, f64)>) -> Self {
        return PointIter { points, idx: 0 };
    }
}

trait Points {
    fn points(&self) -> PointIter;
}

trait Contains {
    fn contains_points(&self, point: (f64, f64)) -> bool;
}

impl<T> Collidable<T> for T
where
    T: Points + Contains,
{
    fn collide(&self, other: &T) -> bool {
        for point in other.points() {
            if self.contains_points(point) {
                return true;
            }
        }
        return false;
    }
}
