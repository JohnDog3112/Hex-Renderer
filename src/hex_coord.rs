use std::ops::{Mul, Add, Sub, Div};

use super::Coord;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HexCoord(pub f32, pub f32);


const Y_FACTOR: f32 = 0.866025403784;

impl HexCoord {
    pub fn get_y(y: i32) -> f32 {
        y as f32 * Y_FACTOR
    }
}
impl From<Coord> for HexCoord {
    fn from(value: Coord) -> Self {
        HexCoord(
            value.0 as f32 + 0.5 * value.1 as f32,
            value.1 as f32 * Y_FACTOR
        )
    }
}

impl Add for HexCoord {
    type Output = HexCoord;

    fn add(self, rhs: Self) -> Self::Output {
        HexCoord(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Sub for HexCoord {
    type Output = HexCoord;

    fn sub(self, rhs: Self) -> Self::Output {
        HexCoord(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl Mul<f32> for HexCoord {
    type Output = HexCoord;

    fn mul(self, rhs: f32) -> Self::Output {
        HexCoord(self.0 * rhs, self.1 * rhs)
    }
}

impl Div<f32> for HexCoord {
    type Output = HexCoord;

    fn div(self, rhs: f32) -> Self::Output {
        HexCoord(self.0/rhs, self.1/rhs)
    }
}