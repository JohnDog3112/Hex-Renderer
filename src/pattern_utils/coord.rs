use super::Direction;
use std::ops::Add;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coord(pub i32, pub i32);

impl Add<Direction> for Coord {
    type Output = Self;

    fn add(self, rhs: Direction) -> Self::Output {
        self + match rhs {
            Direction::East => (1, 0),
            Direction::West => (-1, 0),

            Direction::NorthEast => (1, -1),
            Direction::SouthWest => (-1, 1),

            Direction::SouthEast => (0, 1),
            Direction::NorthWest => (0, -1),
        }
    }
}

impl Add<(i32, i32)> for Coord {
    type Output = Self;

    fn add(self, rhs: (i32, i32)) -> Self::Output {
        Coord(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Add for Coord {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Coord(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl From<(i32, i32)> for Coord {
    fn from(value: (i32, i32)) -> Self {
        Coord(value.0, value.1)
    }
}

impl Coord {
    pub fn min_components(self, rhs: Self) -> Self {
        Coord(self.0.min(rhs.0), self.1.min(rhs.1))
    }
    pub fn max_components(self, rhs: Self) -> Self {
        Coord(self.0.max(rhs.0), self.1.max(rhs.1))
    }

    pub fn order_by_x(self, rhs: Self) -> (Self, Self) {
        if self.0 < rhs.0 || (self.0 == rhs.0 && self.1 < rhs.1) {
            (self, rhs)
        } else {
            (rhs, self)
        }
    }
}
