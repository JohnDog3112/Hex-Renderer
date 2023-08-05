use tiny_skia::{Color, LineCap, LineJoin, Pixmap, Stroke};

use crate::{
    options::{Intersections, Lines},
    pattern_utils::{Angle, Coord, Direction, DynamicList, HexCoord},
};

use super::{
    draw_gradient::draw_gradient_lines, draw_monocolor::draw_monocolor_lines,
    draw_segments::draw_segment_lines, point::draw_points,
};

#[derive(Debug, Clone)]
pub struct Pattern {
    pub path: Vec<Coord>,
    pub top_left: Coord,
    pub bottom_right: Coord,

    pub left_perimiter: Vec<Coord>,
    pub right_perimiter: Vec<Coord>,

    pub points: Vec<Coord>,
    pub angles: Vec<Angle>,
}

impl Pattern {
    pub fn new(rotation: Direction, links: Vec<Angle>) -> Self {
        let mut path = vec![Coord(0, 0), Coord(0, 0) + rotation];
        let mut top_left = path[0].min_components(path[1]);
        let mut bottom_right = path[1].max_components(path[1]);

        let mut rotation = rotation;

        let mut left_perimiter = DynamicList::new();
        let mut right_perimiter = DynamicList::new();

        Self::add_to_perimiter(&mut left_perimiter, &mut right_perimiter, path[0]);
        Self::add_to_perimiter(&mut left_perimiter, &mut right_perimiter, path[1]);

        for link in &links {
            rotation = rotation + *link;

            let next_point = *path.last().unwrap() + rotation;

            top_left = top_left.min_components(next_point);
            bottom_right = bottom_right.max_components(next_point);
            Self::add_to_perimiter(&mut left_perimiter, &mut right_perimiter, next_point);
            path.push(next_point);
        }

        let mut points = path.clone();
        points.dedup();
        Pattern {
            path,
            top_left,
            bottom_right,
            left_perimiter: left_perimiter.to_vector(),
            right_perimiter: right_perimiter.to_vector(),
            points,
            angles: links,
        }
    }
    fn add_to_perimiter(
        left_perimiter: &mut DynamicList<Coord>,
        right_perimiter: &mut DynamicList<Coord>,
        point: Coord,
    ) {
        if let Some(val) = left_perimiter.get(point.1) {
            if point.0 < val.0 {
                left_perimiter.set(point.1, point);
            }
        } else {
            left_perimiter.set(point.1, point);
        }

        if let Some(val) = right_perimiter.get(point.1) {
            if point.0 > val.0 {
                right_perimiter.set(point.1, point);
            }
        } else {
            right_perimiter.set(point.1, point);
        }
    }

    pub fn draw_pattern(
        &self,
        pixmap: &mut Pixmap,
        origin: HexCoord,
        scale: f32,
        line_thickness: f32,
        line_options: &Lines,
        point_options: &Intersections,
    ) {
        let mut stroke = Stroke::default();
        stroke.width = line_thickness * scale;
        stroke.line_cap = LineCap::Round;
        stroke.line_join = LineJoin::Round;

        let end_colors;

        match line_options {
            Lines::Monocolor(color) => {
                draw_monocolor_lines(&self, pixmap, &stroke, origin, scale, *color);
                end_colors = (*color, *color);
            }
            Lines::Gradient {
                colors,
                segments_per_color,
                bent,
            } => {
                if colors.len() < 2 {
                    let col = *colors.get(0).unwrap_or(&Color::WHITE);
                    end_colors = (col, col);
                    draw_monocolor_lines(&self, pixmap, &stroke, origin, scale, col);
                } else {
                    end_colors = (
                        colors[0],
                        draw_gradient_lines(
                            &self,
                            pixmap,
                            &stroke,
                            origin,
                            scale,
                            colors,
                            *segments_per_color,
                            *bent,
                        ),
                    );
                }
            }
            Lines::SegmentColors(colors, triangle) => {
                end_colors = (
                    colors[0],
                    draw_segment_lines(&self, pixmap, &stroke, origin, scale, colors, triangle),
                );
            }
        }

        match point_options {
            Intersections::Nothing => (),
            Intersections::UniformPoints(point) => {
                draw_points(&self.points, pixmap, origin, scale, &point);
            }
            Intersections::EndsAndMiddle { start, end, middle } => {
                let start_point = self.path[0];
                let end_point = self.path[self.path.len() - 1];

                let start = start.clone().into_point(end_colors.0);
                let end = end.clone().into_point(end_colors.1);

                draw_points(&vec![start_point], pixmap, origin, scale, &start);
                if start_point != end_point {
                    draw_points(&vec![end_point], pixmap, origin, scale, &end);
                }
                let middle_points: Vec<Coord> = self
                    .points
                    .clone()
                    .into_iter()
                    .filter(|&point| point != start_point && point != end_point)
                    .collect();

                draw_points(&middle_points, pixmap, origin, scale, &middle);
            }
        }
    }
}

impl TryFrom<&str> for Pattern {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut parts: Vec<&str> = value.split(' ').collect();

        if parts.len() != 2 {
            return Err(());
        }

        if parts[0].to_lowercase().starts_with("hexpattern(") {
            if parts[1].ends_with(')') {
                parts[0] = &parts[0]["hexpattern(".len()..];
                parts[1] = &parts[1][..parts[1].len() - 1];
            } else {
                return Err(());
            }
        }

        let direction: Direction = parts[0].try_into()?;

        let angles: Vec<Angle> = parts[1]
            .chars()
            .map(|a| Angle::try_from(a))
            .collect::<Result<Vec<Angle>, _>>()?;

        return Ok(Pattern::new(direction, angles));
    }
}
