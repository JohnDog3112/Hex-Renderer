use tiny_skia::Color;

use crate::options::{Intersections, Lines, Triangle};

use super::defaults::constants;

#[derive(Clone)]
pub struct GridOptions {
    pub line_thickness: f32,
    pub scale: f32,
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
            scale: constants::SCALE,
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
        triangle: Triangle,
    ) -> Self {
        Self::generate_changing(
            intersection,
            colors
                .into_iter()
                .map(|colors| Lines::SegmentColors(colors, triangle))
                .collect(),
        )
    }
}
/*
impl GridOptions {
    fn generate_uniform(inp: (Intersections, Lines)) -> Self {
        GridOptions {
            line_thickness: defaults::LINE_THICKNESS,
            scale: DEFAULT_SCALE,
            draw_options: GridDrawOptions::Uniform(inp.0, inp.1),
        }
    }
    pub fn monocolor(color: Color) -> Self {
        Self::generate_uniform(monocolor_parts(color))
    }
    pub fn monocolor_palette(palette: Vec<Color>) -> Self {
        Self::monocolor(palette[0])
    }
}

fn monocolor_parts(color: Color) -> (Intersections, Lines) {

}

impl GridOptions::Uniform {


    pub fn monocolor(color: Color) -> Self {
        Self::generate_uniform(monocolor_parts(color))
    }

    pub fn monocolor_palette(color: Vec<Color>) -> Self {
        Self::monocolor(color[0])
    }

    pub fn monocolor_default()

    pub fn gradient(palette: Vec<Color>) -> Self {}
}

#[allow(dead_code)]
impl GridOptions {
    fn default_colors() -> Vec<Color> {
        vec![
            Color::from_rgba8(214, 9, 177, 255),
            Color::from_rgba8(108, 25, 140, 255),
            Color::from_rgba8(50, 102, 207, 255),
            Color::from_rgba8(102, 110, 125, 255),
        ]
    }
    pub fn gradient() -> Self {
        let intersections = Intersections::EndsAndMiddle(
            EndPoint::BorderedMatch(
                Self::DEFAULT_INNER_RADIUS,
                Color::WHITE,
                Self::DEFAULT_OUTER_RADIUS,
            ),
            EndPoint::Marker(Point::None),
            Point::Single(Color::WHITE, Self::DEFAULT_INNER_RADIUS),
        );

        let mut colors = vec![];
        Self::default_colors()[0..2].clone_into(&mut colors);

        let lines = Lines::Gradient(colors, 15, true);

        Self::generate_uniform(intersections, lines)
    }

    pub fn multi_gradient() -> Self {
        let intersections = Intersections::EndsAndMiddle(
            EndPoint::BorderedMatch(
                Self::DEFAULT_INNER_RADIUS,
                Color::WHITE,
                Self::DEFAULT_OUTER_RADIUS,
            ),
            EndPoint::Marker(Point::None),
            Point::Single(Color::WHITE, Self::DEFAULT_INNER_RADIUS),
        );

        let lines = Lines::Gradient(Self::default_colors(), 15, true);

        Self::generate_uniform(intersections, lines)
    }

    pub fn segments() -> Self {
        let end_point = EndPoint::BorderedMatch(
            Self::DEFAULT_INNER_RADIUS,
            Color::WHITE,
            Self::DEFAULT_OUTER_RADIUS,
        );
        let intersections = Intersections::EndsAndMiddle(
            end_point.clone(),
            end_point,
            Point::Single(Color::WHITE, Self::DEFAULT_INNER_RADIUS),
        );
        let lines = Lines::SegmentColors(
            Self::default_colors(),
            Triangle::BorderStartMatch(0.16, Color::WHITE, 0.24),
        );

        Self::generate_uniform(intersections, lines)
    }
}
*/
