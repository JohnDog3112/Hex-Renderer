use lazy_static::lazy_static;
use tiny_skia::Color;

use crate::options::{palettes, GridPatternOptions, Marker, Triangle};

use super::{EndPoint, GridOptions, Intersections, Lines, Point};

#[allow(dead_code)]
pub mod constants {
    pub const LINE_THICKNESS: f32 = 0.12;
    pub const SCALE: f32 = 50.0;
    pub const INNER_RADIUS: f32 = 0.07;
    pub const OUTER_RADIUS: f32 = 0.1;

    pub const SEGS_PER_COLOR: usize = 15;
    pub const GRADIENT_INNER_RADIUS: f32 = 0.05;
    pub const GRADIENT_OUTER_RADIUS: f32 = 0.07;

    pub const TRIANGLE_INNER_RADIUS: f32 = 0.16;
    pub const TRIANGLE_OUTER_RADIUS: f32 = 0.25;
}
use constants::*;
pub mod components {
    use super::*;

    lazy_static! {
        pub static ref MARKER: Marker = Marker {
            radius: OUTER_RADIUS,
            color: Color::WHITE
        };
        pub static ref POINT: Point = Point::Single(*MARKER);
    }
    lazy_static! {
        pub static ref MONOCOLOR_INTERSECTION: Intersections = Intersections::UniformPoints(*POINT);
        pub static ref UNIFORM_MONOCOLOR_LINE: Lines = Lines::Monocolor(palettes::DEFAULT[0]);
    }
    lazy_static! {
        pub static ref GRADIENT_INTERSECTION: Intersections = Intersections::Nothing;
        pub static ref GRADIENT_MARKER: Marker = Marker {
            color: Color::WHITE,
            radius: GRADIENT_OUTER_RADIUS,
        };
        pub static ref GRADIENT_START_POINT: EndPoint = EndPoint::BorderedMatch {
            match_radius: GRADIENT_INNER_RADIUS,
            border: *GRADIENT_MARKER
        };
        pub static ref GRADIENT_POINT_INTERSECTION: Intersections = Intersections::EndsAndMiddle {
            start: *GRADIENT_START_POINT,
            end: Point::Single(*GRADIENT_MARKER).into(),
            middle: Point::Single(*GRADIENT_MARKER)
        };
        pub static ref UNIFORM_GRADIENT_LINE: Lines = Lines::Gradient {
            colors: palettes::DEFAULT.to_vec(),
            segments_per_color: SEGS_PER_COLOR,
            bent: true,
        };
    }
    lazy_static! {
        pub static ref SEGMENT_END_POINT: EndPoint = EndPoint::BorderedMatch {
            match_radius: INNER_RADIUS,
            border: *MARKER
        };
        pub static ref SEGMENT_INTERSECTION: Intersections = Intersections::EndsAndMiddle {
            start: *SEGMENT_END_POINT,
            end: *SEGMENT_END_POINT,
            middle: *POINT,
        };
        pub static ref TRIANGLE_MARKER: Marker = Marker {
            color: Color::WHITE,
            radius: TRIANGLE_OUTER_RADIUS,
        };
        pub static ref TRIANGLE: Triangle = Triangle::BorderStartMatch {
            match_radius: TRIANGLE_INNER_RADIUS,
            border: *TRIANGLE_MARKER
        };
        pub static ref SEGMENT_LINE: Lines =
            Lines::SegmentColors(palettes::DEFAULT.to_vec(), *TRIANGLE);
    }
}
use components::*;

mod grids {
    use super::*;
    lazy_static! {
        pub static ref UNIFORM_MONOCOLOR: GridOptions = GridOptions::generate(
            GridPatternOptions::Uniform(*MONOCOLOR_INTERSECTION, UNIFORM_MONOCOLOR_LINE.clone())
        );
        pub static ref MONOCOLOR: GridOptions =
            GridOptions::generate(GridPatternOptions::gen_changing_monocolor(
                *MONOCOLOR_INTERSECTION,
                palettes::DEFAULT.to_vec()
            ));
    }

    lazy_static! {
        pub static ref UNIFORM_GRADIENT: GridOptions = GridOptions::generate(
            GridPatternOptions::Uniform(*GRADIENT_INTERSECTION, UNIFORM_GRADIENT_LINE.clone())
        );
        pub static ref UNIFORM_POINT_GRADIENT: GridOptions =
            GridOptions::generate(GridPatternOptions::Uniform(
                *GRADIENT_POINT_INTERSECTION,
                UNIFORM_GRADIENT_LINE.clone()
            ));
        pub static ref GRADIENT: GridOptions =
            GridOptions::generate(GridPatternOptions::gen_changing_gradient(
                *GRADIENT_INTERSECTION,
                palettes::ALL.to_vec(),
                true
            ));
        pub static ref POINT_GRADIENT: GridOptions =
            GridOptions::generate(GridPatternOptions::gen_changing_gradient(
                *GRADIENT_POINT_INTERSECTION,
                palettes::ALL.to_vec(),
                true
            ));
    }
    lazy_static! {
        pub static ref UNIFORM_SEGMENT: GridOptions = GridOptions::generate(
            GridPatternOptions::Uniform(*SEGMENT_INTERSECTION, SEGMENT_LINE.clone())
        );
        pub static ref SEGMENT: GridOptions =
            GridOptions::generate(GridPatternOptions::gen_changing_segment(
                *SEGMENT_INTERSECTION,
                palettes::ALL.to_vec(),
                *TRIANGLE
            ));
    }
}
pub use grids::*;
