use tiny_skia::Color;

use crate::options::{Intersections, Lines, Triangle};

use super::{defaults::constants, CollisionOption};

#[derive(Clone)]
pub struct GridOptions {
    pub line_thickness: f32,
    pub pattern_options: GridPatternOptions,
}

#[allow(dead_code)]
#[derive(Clone)]
pub enum GridPatternOptions {
    Uniform(Intersections, Lines),
    Changing(Vec<(Intersections, Lines)>),
}
impl GridOptions {
    pub fn generate(pattern_options: GridPatternOptions) -> Self {
        Self {
            line_thickness: constants::LINE_THICKNESS,
            pattern_options,
        }
    }
}
impl GridPatternOptions {
    pub fn generate_changing(intersection: Intersections, lines: Vec<Lines>) -> Self {
        let mut parts = Vec::new();

        for line in lines {
            parts.push((intersection, line));
        }
        Self::Changing(parts)
    }
    pub fn gen_changing_monocolor(intersection: Intersections, colors: Vec<Color>) -> Self {
        GridPatternOptions::generate_changing(
            intersection,
            colors
                .into_iter()
                .map(|color| Lines::Monocolor(color))
                .collect(),
        )
    }
    pub fn gen_changing_gradient(
        intersection: Intersections,
        colors: Vec<Vec<Color>>,
        bent: bool,
    ) -> Self {
        GridPatternOptions::generate_changing(
            intersection,
            colors
                .into_iter()
                .map(|colors| Lines::Gradient {
                    colors,
                    segments_per_color: constants::SEGS_PER_COLOR,
                    bent,
                })
                .collect(),
        )
    }
    pub fn gen_changing_segment(
        intersection: Intersections,
        colors: Vec<Vec<Color>>,
        triangles: Triangle,
        collisions: CollisionOption,
    ) -> Self {
        Self::generate_changing(
            intersection,
            colors
                .into_iter()
                .map(|colors| Lines::SegmentColors {
                    colors,
                    triangles,
                    collisions,
                })
                .collect(),
        )
    }
}
