use tiny_skia::Color;

#[derive(Debug, Clone)]
pub enum Lines {
    Monocolor(Color),
    Gradient(Color, Color),
    MultiGradient(Vec<Color>),
    BoundGradient(Vec<Color>, usize),
    SegmentColors(Vec<Color>, Triangle),
}

#[derive(Debug, Clone)]
pub enum Triangle {
    None,
    Match(f32),
    BorderMatch(f32, Color, f32),
    BorderStartMatch(f32, Color, f32),
}

#[derive(Debug, Clone)]
pub enum Marker {
    None,
    SinglePoint(Color, f32),
    DoublePoint(Color, f32, Color, f32),
}
#[derive(Debug, Clone)]
pub enum EndPoint {
    EndMarker(Marker),
    Match(f32),
    BorderedMatch(f32, Color, f32),
}

#[derive(Debug, Clone)]
pub enum Intersections {
    Nothing,
    UniformPoints(Marker),
    EndsAndMiddle(EndPoint, EndPoint, Marker),
}

impl From<Marker> for EndPoint {
    fn from(value: Marker) -> Self {
        EndPoint::EndMarker(value)
    }
}

impl EndPoint {
    pub fn into_point(self, end_color: Color) -> Marker{
        match self {
            Self::EndMarker(point) => point,
            Self::Match(radius) => Marker::SinglePoint(end_color, radius),
            Self::BorderedMatch(r1, color, r2) => {
                if r1 > r2 {
                    Marker::DoublePoint(end_color, r1, color, r2)
                } else {
                    Marker::DoublePoint(color, r2, end_color, r1)
                }
            },
        }
    }
}