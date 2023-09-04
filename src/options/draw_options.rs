use tiny_skia::Color;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum Lines {
    Monocolor {
        color: Color,
        bent: bool,
    },
    Gradient {
        colors: Vec<Color>,
        segments_per_color: usize,
        bent: bool,
    },
    SegmentColors {
        colors: Vec<Color>,
        triangles: Triangle,
        collisions: CollisionOption,
    },
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub enum Triangle {
    None,
    Match { radius: f32 },
    BorderMatch { match_radius: f32, border: Marker },
    BorderStartMatch { match_radius: f32, border: Marker },
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub enum CollisionOption {
    Dashes(Color),
    MatchedDashes,
    ParallelLines,
    OverloadedParallel {
        max_line: usize,
        overload: OverloadOptions,
    },
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub enum OverloadOptions {
    Dashes(Color),
    LabeledDashes { color: Color, label: Marker },
    MatchedDashes,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub enum Point {
    None,
    Single(Marker),
    Double { inner: Marker, outer: Marker },
}

#[derive(Debug, Clone, Copy)]
pub struct Marker {
    pub color: Color,
    pub radius: f32,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub enum EndPoint {
    Point(Point),
    Match { radius: f32 },
    BorderedMatch { match_radius: f32, border: Marker },
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub enum Intersections {
    Nothing,
    UniformPoints(Point),
    EndsAndMiddle {
        start: EndPoint,
        end: EndPoint,
        middle: Point,
    },
}

impl From<Point> for EndPoint {
    fn from(value: Point) -> Self {
        EndPoint::Point(value)
    }
}

impl EndPoint {
    pub fn into_point(self, end_color: Color) -> Point {
        match self {
            Self::Point(point) => point,
            Self::Match { radius } => Point::Single(Marker {
                color: end_color,
                radius,
            }),
            Self::BorderedMatch {
                match_radius,
                border,
            } => {
                let match_marker = Marker {
                    radius: match_radius,
                    color: end_color,
                };
                if match_radius > border.radius {
                    Point::Double {
                        inner: border,
                        outer: match_marker,
                    }
                } else {
                    Point::Double {
                        inner: match_marker,
                        outer: border,
                    }
                }
            }
        }
    }
}

impl Triangle {
    pub fn to_middle_point(&self, color: Color) -> Option<Point> {
        match self {
            Triangle::None => None,
            Triangle::Match { radius }
            | Triangle::BorderStartMatch {
                match_radius: radius,
                border: _,
            } => Some(Point::Single(Marker {
                color,
                radius: *radius,
            })),
            Triangle::BorderMatch {
                match_radius,
                border,
            } => {
                let match_marker = Marker {
                    radius: *match_radius,
                    color,
                };
                let marker;
                if *match_radius > border.radius {
                    marker = Point::Double {
                        inner: *border,
                        outer: match_marker,
                    };
                } else {
                    marker = Point::Double {
                        inner: match_marker,
                        outer: *border,
                    };
                }
                Some(marker)
            }
        }
    }
    pub fn to_start_point(&self, start_color: Color) -> Option<Point> {
        match self {
            Triangle::None => None,
            Triangle::Match { radius } => Some(Point::Single(Marker {
                color: start_color,
                radius: *radius,
            })),
            Triangle::BorderMatch {
                match_radius,
                border,
            }
            | Triangle::BorderStartMatch {
                match_radius,
                border,
            } => {
                let marker;
                let match_marker = Marker {
                    color: start_color,
                    radius: *match_radius,
                };
                if *match_radius > border.radius {
                    marker = Point::Double {
                        inner: *border,
                        outer: match_marker,
                    };
                } else {
                    marker = Point::Double {
                        outer: *border,
                        inner: match_marker,
                    };
                }
                Some(marker)
            }
        }
    }
}
impl Point {
    pub fn get_max_radius(&self) -> f32 {
        match self {
            Point::None => 0.0,
            Point::Single(marker) => marker.radius,
            Point::Double { inner, outer } => inner.radius.max(outer.radius),
        }
    }
}
impl EndPoint {
    pub fn get_max_radius(&self) -> f32 {
        match self {
            EndPoint::Point(point) => point.get_max_radius(),
            EndPoint::Match { radius } => *radius,
            EndPoint::BorderedMatch {
                match_radius,
                border,
            } => match_radius.max(border.radius),
        }
    }
}
impl Intersections {
    pub fn get_max_radius(&self) -> f32 {
        match self {
            Intersections::Nothing => 0.0,
            Intersections::UniformPoints(marker) => marker.get_max_radius(),
            Intersections::EndsAndMiddle {
                start: start_point,
                end: end_point,
                middle: middle_points,
            } => start_point
                .get_max_radius()
                .max(end_point.get_max_radius())
                .max(middle_points.get_max_radius()),
        }
    }
}
impl Triangle {
    pub fn get_max_radius(&self) -> f32 {
        match self {
            Triangle::None => 0.0,
            Triangle::Match { radius } => *radius,
            Triangle::BorderMatch {
                match_radius,
                border,
            } => match_radius.max(border.radius),
            Triangle::BorderStartMatch {
                match_radius,
                border,
            } => match_radius.max(border.radius),
        }
    }
}
impl Lines {
    pub fn get_max_radius(&self) -> f32 {
        match self {
            Lines::Monocolor { color: _, bent: _ }
            | Lines::Gradient {
                colors: _,
                segments_per_color: _,
                bent: _,
            } => 0.0,
            Lines::SegmentColors {
                colors: _,
                triangles: arrows,
                collisions: _,
            } => arrows.get_max_radius(),
        }
    }
}
