use crate::{
    defaults,
    options::{Intersections, Lines, Triangle, Color},
    pattern_utils::Angle,
};

use super::{defaults::constants, CollisionOption, Point};

#[derive(Clone, Debug, PartialEq, PartialOrd)]
///Main struct for all pattern rendering options
pub struct GridOptions {
    ///Thickness of line in relation to distance between points
    /// eg. if the line_thickness = 0.1, and the distance between points is 10 pixels,
    /// then the line_thickness would be 1 pixel
    pub line_thickness: f32,
    ///Further options for how to render each pattern
    pub pattern_options: GridPatternOptions,
    ///Optional point to place in the center of each pattern (helps with determining pattern size at a glance)
    pub center_dot: Point,
}

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
///Struct that holds the different variations of GridPatterns
pub enum GridPatternOptions {
    ///Uniform means that all patterns will be rendered in the same way
    /// (This excludes the difference with PatternVariant)
    Uniform(Intersections, Lines),
    ///Changes what pattern renderer to use when finding an introspect or retrospect pattern
    /// That way you can change colors/renderers for embedded patterns
    Changing {
        ///Variations to use, starts at the first and goes up when it reaches an intro, goes down when reaching a retro
        variations: Vec<(Intersections, Lines)>,
        ///Vec of the angle_sigs of intro patterns
        intros: Vec<Vec<Angle>>,
        ///Vec of angle_sigs of retro patterns
        retros: Vec<Vec<Angle>>,
    },
}
impl GridOptions {
    ///Helper function that creates a new [GridOptions] using the default line_thickness: [constants::LINE_THICKNESS]
    pub fn generate(pattern_options: GridPatternOptions, center_dot: Point) -> Self {
        Self {
            line_thickness: constants::LINE_THICKNESS,
            pattern_options,
            center_dot,
        }
    }
}
impl GridPatternOptions {
    ///Generates a changing [GridPatternOptions] where the [Intersections] options are copied among all variations
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

    ///Same as generate_changing except it uses the angle_sigs for Introspection and Retrospection
    pub fn generate_default_changing(intersection: Intersections, lines: Vec<Lines>) -> Self {
        Self::generate_changing(
            intersection,
            lines,
            defaults::INTRO_ANGLES.to_vec(),
            defaults::RETRO_ANGLES.to_vec(),
        )
    }

    ///Generates a changing [GridPatternOptions] where all the variations are a Monocolor renderer
    pub fn gen_changing_monocolor(
        intersection: Intersections,
        colors: Vec<Color>,
        bent: bool,
    ) -> Self {
        GridPatternOptions::generate_default_changing(
            intersection,
            colors
                .into_iter()
                .map(|color| Lines::Monocolor { color, bent })
                .collect(),
        )
    }

    ///Generates a changing [GridPatternOptions] where all the variations are a Gradient renderer
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

    ///Generates a changing [GridPatternOptions] where all the variations are a Segment renderer
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
