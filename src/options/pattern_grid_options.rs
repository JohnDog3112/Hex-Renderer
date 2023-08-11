use tiny_skia::Color;

use crate::{
    defaults,
    options::{Intersections, Lines, Triangle},
    pattern_utils::Angle,
};

use super::{defaults::constants, CollisionOption, Point};

#[derive(Clone)]
pub struct GridOptions {
    pub line_thickness: f32,
    pub pattern_options: GridPatternOptions,
    pub center_dot: Point,
}

#[allow(dead_code)]
#[derive(Clone)]
pub enum GridPatternOptions {
    Uniform(Intersections, Lines),
    Changing {
        variations: Vec<(Intersections, Lines)>,
        intros: Vec<Vec<Angle>>,
        retros: Vec<Vec<Angle>>,
    },
}
impl GridOptions {
    pub fn generate(pattern_options: GridPatternOptions, center_dot: Point) -> Self {
        Self {
            line_thickness: constants::LINE_THICKNESS,
            pattern_options,
            center_dot,
        }
    }
}
impl GridPatternOptions {
    pub fn generate_changing(
        intersection: Intersections,
        lines: Vec<Lines>,
        intros: Vec<Vec<Angle>>,
        retros: Vec<Vec<Angle>>,
    ) -> Self {
        let mut parts = Vec::new();

        for line in lines {
            parts.push((intersection, line));
        }
        Self::Changing {
            variations: parts,
            intros,
            retros,
        }
    }
    pub fn generate_default_changing(intersection: Intersections, lines: Vec<Lines>) -> Self {
        Self::generate_changing(
            intersection,
            lines,
            defaults::INTRO_ANGLES.to_vec(),
            defaults::RETRO_ANGLES.to_vec(),
        )
    }
    pub fn gen_changing_monocolor(intersection: Intersections, colors: Vec<Color>) -> Self {
        GridPatternOptions::generate_default_changing(
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
        GridPatternOptions::generate_default_changing(
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
        Self::generate_default_changing(
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

impl GridOptions {
    pub fn get_max_radius(&self) -> f32 {
        self.line_thickness
            .max(self.center_dot.get_max_radius())
            .max(self.pattern_options.get_max_radius())
    }
}
impl GridPatternOptions {
    pub fn get_max_radius(&self) -> f32 {
        match self {
            GridPatternOptions::Uniform(intersection, line) => {
                intersection.get_max_radius().max(line.get_max_radius())
            }
            GridPatternOptions::Changing {
                variations,
                intros: _,
                retros: _,
            } => variations
                .iter()
                .map(|part| part.0.get_max_radius().max(part.1.get_max_radius()))
                .fold(0.0, |a, b| a.max(b)),
        }
    }
}
