use super::Angle;
use std::ops::Add;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    East = 0,
    SouthEast = 1,
    SouthWest = 2,
    West = 3,
    NorthWest = 4,
    NorthEast = 5,
}

impl Add<Angle> for Direction {
    type Output = Self;

    fn add(self, rhs: Angle) -> Self::Output {
        ((self as u8 + rhs as u8) % 6).try_into().unwrap()
    }
}

impl TryFrom<(i32, i32)> for Direction {
    type Error = ();

    fn try_from(value: (i32, i32)) -> Result<Self, Self::Error> {
        match value {
            (1, 0) => Ok(Self::East),
            (-1, 0) => Ok(Self::West),

            (1, -1) => Ok(Direction::NorthEast),
            (-1, 1) => Ok(Direction::SouthWest),

            (0, 1) => Ok(Direction::SouthEast),
            (0, -1) => Ok(Direction::NorthWest),

            _ => Err(()),
        }
    }
}

impl Direction {
    #[allow(dead_code)]
    pub fn get_reverse(self) -> Self {
        match self {
            Self::East => Self::West,
            Self::West => Self::East,

            Self::NorthEast => Self::SouthWest,
            Self::SouthWest => Self::NorthEast,

            Self::NorthWest => Self::SouthEast,
            Self::SouthEast => Self::NorthWest,
        }
    }
}
#[derive(Debug)]
pub enum DirectionParseError {
    InvalidNumber(u8),
    InvalidStr(String),
}
impl TryFrom<u8> for Direction {
    type Error = DirectionParseError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Direction::East),
            1 => Ok(Direction::SouthEast),
            2 => Ok(Direction::SouthWest),
            3 => Ok(Direction::West),
            4 => Ok(Direction::NorthWest),
            5 => Ok(Direction::NorthEast),
            _ => Err(Self::Error::InvalidNumber(value)),
        }
    }
}

impl TryFrom<&str> for Direction {
    type Error = DirectionParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match &value.to_lowercase()[..] {
            "east" | "e" => Ok(Direction::East),
            "southeast" | "south_east" | "se" => Ok(Direction::SouthEast),
            "southwest" | "south_west" | "sw" => Ok(Direction::SouthWest),
            "west" | "w" => Ok(Direction::West),
            "northwest" | "north_west" | "nw" => Ok(Direction::NorthWest),
            "northeast" | "north_east" | "ne" => Ok(Direction::NorthEast),
            _ => Err(Self::Error::InvalidStr(value.to_string())),
        }
    }
}
