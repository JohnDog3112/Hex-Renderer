use tiny_skia::Color;

use crate::draw_options::{Intersections, Lines, Marker, EndPoint, Triangle};

#[derive(Clone)]
pub struct GridOptions {
    pub line_thickness: f32,
    pub scale: f32,
    pub draw_options: GridDrawOptions,
}
#[allow(dead_code)]
#[derive(Clone)]
pub enum GridDrawOptions {
    Uniform(Intersections, Lines),
    Changing(Vec<(Intersections, Lines)>)
}

#[allow(dead_code)]
impl GridOptions {
    const DEFUALT_LINE_THICKNESS: f32 = 0.12;
    const DEFAULT_SCALE: f32 = 50.0;
    const DEFAULT_INNER_RADIUS: f32 = 0.1;
    const DEFAULT_OUTER_RADIUS: f32 = 0.14;

    fn generate_uniform(intersections: Intersections, lines: Lines) -> Self {
        GridOptions {
            line_thickness: Self::DEFUALT_LINE_THICKNESS,
            scale: Self::DEFAULT_SCALE,
            draw_options: GridDrawOptions::Uniform(intersections, lines)
        }
    }

    pub fn monocolor() -> Self {
        let intersections = Intersections::UniformPoints(Marker::SinglePoint(Color::WHITE, Self::DEFAULT_INNER_RADIUS));
        let lines = Lines::Monocolor(Color::from_rgba8(108, 25, 140, 255));

        Self::generate_uniform(intersections, lines)
    }

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
            EndPoint::BorderedMatch(Self::DEFAULT_INNER_RADIUS, Color::WHITE, Self::DEFAULT_OUTER_RADIUS), 
            EndPoint::Marker(Marker::None), 
            Marker::SinglePoint(Color::WHITE, Self::DEFAULT_INNER_RADIUS)
        );

        let mut colors = vec![];
        Self::default_colors()[0..2].clone_into(&mut colors);

        let lines = Lines::Gradient(colors,15,true);

        Self::generate_uniform(intersections, lines)
    }

    pub fn multi_gradient() -> Self {
        let intersections = Intersections::EndsAndMiddle(
            EndPoint::BorderedMatch(Self::DEFAULT_INNER_RADIUS, Color::WHITE, Self::DEFAULT_OUTER_RADIUS), 
            EndPoint::Marker(Marker::None), 
            Marker::SinglePoint(Color::WHITE, Self::DEFAULT_INNER_RADIUS)
        );

        let lines = Lines::Gradient(Self::default_colors(), 15, true);

        Self::generate_uniform(intersections, lines)
    }

    pub fn segments() -> Self {
        let end_point = EndPoint::BorderedMatch(Self::DEFAULT_INNER_RADIUS, Color::WHITE, Self::DEFAULT_OUTER_RADIUS);
        let intersections = Intersections::EndsAndMiddle(
            end_point.clone(), 
            end_point, 
            Marker::SinglePoint(Color::WHITE, Self::DEFAULT_INNER_RADIUS)
        );
        let lines = Lines::SegmentColors(
            Self::default_colors(),
            Triangle::BorderStartMatch(0.16, Color::WHITE, 0.24)
        );

        Self::generate_uniform(intersections, lines)
    }

    
} 