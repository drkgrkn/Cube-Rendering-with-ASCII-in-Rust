use crate::point::Point;

pub struct Edge {
    p1: Point,
    p2: Point,
}

impl Edge {
    pub fn new(p1: Point, p2: Point) -> Edge {
        Edge { p1: p1, p2: p2 }
    }
}
