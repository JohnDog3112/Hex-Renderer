#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
///Color struct, using RGBA
pub struct Color(pub u8, pub u8, pub u8, pub u8);

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
///Options for drawing the lines of the patterns
pub enum Lines {

    ///Monocolor draws the lines in a single color
    /// if bent = true, the corners will bend on the intersections
    Monocolor {
        ///Color to draw the lines with
        color: Color,
        ///Whether or not it bends at intersection points
        bent: bool,
    },
    ///Gradient slowly switches between colors (gradient)
    Gradient {
        ///Vec of colors to draw gradients between
        /// If the vec is empty, it's treated as Monocolor
        colors: Vec<Color>,
        ///Minimum number of segments before adding another color to switch between
        /// Eg. if segments_per_color = 10,
        /// 1-9 segments - maximum of 2 colors
        /// 10-19 segments - maximum of 3 colors, 
        segments_per_color: usize,
        ///Whether or not to have the segments bend around corners
        bent: bool,
    },
    ///Changes colors whenever it reaches an intersection that's already had the current color
    SegmentColors {
        ///Colors to use
        colors: Vec<Color>,
        ///Arrows/Triangles to draw at the start and when switching between colors
        triangles: Triangle,
        ///Options for impossible patterns (when you get overlapping segments)
        collisions: CollisionOption,
    },
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
///Options for drawing the triangle/arrow between color changes on the Segment Renderer
pub enum Triangle {
    ///None, simply don't draw them
    None,
    ///Match the color of the line
    Match { 
        ///radius is how big it is (as a percentage of segment length)
        radius: f32 
    },
    ///Same as [Triangle::Match] except with an extra border around it
    BorderMatch { 
        ///radius of how big the match triangle is (as a percentage of segment length)
        match_radius: f32, 
        ///a [Marker] for the border
        border: Marker 
    },
    ///Same as [Triangle::BorderMatch] except with an extra triangle right after the start point
    BorderStartMatch { 
        ///radius of how big the match triangle is (as a percentage of segment length)
        match_radius: f32, 
        ///a [Marker] for the border
        border: Marker 
    },
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
///Options for drawing overlapping segments (impossible patterns)
pub enum CollisionOption {
    ///Draws the first segment and then dashes of the given color for the rest
    Dashes(Color),
    ///Draws the line as a set of dashes where the dash marks match the colors of the overlapping lines
    MatchedDashes,
    ///Draws each of the segments as smaller, parallel lines all next to eachother
    ParallelLines,
    ///Same as [CollisionOption::ParallelLines] except with an escape when you get too many overlaps
    OverloadedParallel {
        ///number of overlapping segments/lines before using the overload option
        max_line: usize,
        ///Rendering option for when reaching too many parallel lines
        overload: OverloadOptions,
    },
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
///Options for what to do when you get too many parallel lines
pub enum OverloadOptions {
    ///same as [CollisionOption::Dashes] (just draws dashes of the given color over the first line)
    Dashes(Color),
    ///Similar to [OverloadOptions::Dashes] except it includes a label with the number of overlapping lines
    LabeledDashes { color: Color, label: Marker },
    ///same as [CollisionOption::MatchedDashes] (represents them as dashes that represet each color of overlapping lines)
    MatchedDashes,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
///Options for drawing points at the grid points/intersections
pub enum Point {
    ///Doesn't draw any points
    None,
    ///Draws a single dot
    Single(Marker),
    ///Draws an inner dot dotand outer dot (or a point with a border)
    Double { inner: Marker, outer: Marker },
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
///Specifier for how to draw a shape (not necessarily a circle)
pub struct Marker {
    ///The color to draw it with
    pub color: Color,
    ///The radius of the shape
    pub radius: f32,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
///Specifier for how to draw the start and end points on a pattern
pub enum EndPoint {
    ///Draw a normal point
    Point(Point),
    ///Draw a point that matches the starting/ending color
    Match { radius: f32 },
    ///Draw a point that matches the starting/ending color with a border
    BorderedMatch { match_radius: f32, border: Marker },
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
///How to draw all the points in a pattern, including start, end, and middle points
pub enum Intersections {
    ///Doesn't draw any points
    Nothing,
    ///Draws the same point for everything, including start and end
    UniformPoints(Point),
    ///Draws a different point for the start, end, and middle
    EndsAndMiddle {
        start: EndPoint,
        end: EndPoint,
        middle: Point,
    },
}

impl Color {
    pub const WHITE: Self = Self(255, 255, 255, 255);
    pub const BLACK: Self = Self(0, 0, 0, 255);
}

impl From<Color> for tiny_skia::Color {
    fn from(value: Color) -> Self {
        Self::from_rgba8(value.0, value.1, value.2, value.3)
    }
}
impl From<tiny_skia::Color> for Color {
    fn from(value: tiny_skia::Color) -> Self {
        value.to_color_u8().into()
    }
}
impl From<tiny_skia::ColorU8> for Color {
    fn from(value: tiny_skia::ColorU8) -> Self {
        Self(
            value.red(),
            value.green(),
            value.blue(),
            value.alpha()
        )
    }
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
                let marker = if *match_radius > border.radius {
                    Point::Double {
                        inner: *border,
                        outer: match_marker,
                    }
                } else {
                    Point::Double {
                        inner: match_marker,
                        outer: *border,
                    }
                };
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
                let match_marker = Marker {
                    color: start_color,
                    radius: *match_radius,
                };
                let marker = if *match_radius > border.radius {
                    Point::Double {
                        inner: *border,
                        outer: match_marker,
                    }
                } else {
                    Point::Double {
                        outer: *border,
                        inner: match_marker,
                    }
                };
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
