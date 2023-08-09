use super::Coord;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ConnectionPoint(Coord, Coord);

impl ConnectionPoint {
    pub fn new(a: Coord, b: Coord) -> Self {
        let (a, b) = a.order_by_x(b);
        Self(a, b)
    }
}
