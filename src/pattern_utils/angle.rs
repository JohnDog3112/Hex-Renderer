#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Angle {
    Forward = 0,
    Right = 1,
    BackRight = 2,
    Back = 3,
    BackLeft = 4,
    Left = 5,
}

pub struct AngleParseError(pub char);

impl TryFrom<char> for Angle {
    type Error = AngleParseError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'w' => Ok(Angle::Forward),
            'e' => Ok(Angle::Right),
            'd' => Ok(Angle::BackRight),
            's' => Ok(Angle::Back),
            'a' => Ok(Angle::BackLeft),
            'q' => Ok(Angle::Left),
            _ => Err(AngleParseError(value)),
        }
    }
}
