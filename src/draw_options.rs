use tiny_skia::Color;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum Lines {
    Monocolor(Color),
    Gradient(GradientOptions),
    SegmentColors(Vec<Color>, Triangle),
}

#[derive(Debug, Clone)]
pub struct GradientOptions {
    pub colors: Vec<Color>,
    pub segs_per_color: usize,
    pub bent_corners: bool,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum Triangle {
    None,
    Match(f32),
    BorderMatch(f32, Color, f32),
    BorderStartMatch(f32, Color, f32),
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum Marker {
    None,
    SinglePoint(Color, f32),
    DoublePoint(Color, f32, Color, f32),
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum EndPoint {
    Marker(Marker),
    Match(f32),
    BorderedMatch(f32, Color, f32),
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum Intersections {
    Nothing,
    UniformPoints(Marker),
    EndsAndMiddle(EndPoint, EndPoint, Marker),
}

impl From<Marker> for EndPoint {
    fn from(value: Marker) -> Self {
        EndPoint::Marker(value)
    }
}

impl EndPoint {
    pub fn into_point(self, end_color: Color) -> Marker{
        match self {
            Self::Marker(point) => point,
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

impl Marker {
    pub fn get_max_radius(&self) -> f32 {
        match self {
            Marker::None => 0.0,
            Marker::SinglePoint(_, r) => *r,
            Marker::DoublePoint(_, r1, _, r2) => r1.max(*r2),
        }
    }
}
impl EndPoint {
    pub fn get_max_radius(&self) -> f32 {
        match self {
            EndPoint::Marker(marker) => marker.get_max_radius(),
            EndPoint::Match(r) => *r,
            EndPoint::BorderedMatch(r1, _, r2) => r1.max(*r2),
        }
    }
}
impl Intersections {
    pub fn get_max_radius(&self) -> f32 {
        match self {
            Intersections::Nothing => 0.0,
            Intersections::UniformPoints(marker) => marker.get_max_radius(),
            Intersections::EndsAndMiddle(m1, m2, m3) => 
                m1.get_max_radius().max(m2.get_max_radius()).max(m3.get_max_radius()),
        }
    }
}
impl Triangle {
    pub fn get_max_radius(&self) -> f32 {
        match self {
            Triangle::None => 0.0,
            Triangle::Match(r) => *r,
            Triangle::BorderMatch(r1, _, r2) => r1.max(*r2),
            Triangle::BorderStartMatch(r1, _, r2) => r1.max(*r2),
        }
    }
}
impl Lines {
    pub fn get_max_radius(&self) -> f32 {
        match self {
            Lines::Monocolor(_) => 0.0,
            Lines::Gradient(_) => 0.0,
            Lines::SegmentColors(_, triangle) => triangle.get_max_radius(),
        }
    }
}