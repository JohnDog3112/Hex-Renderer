use std::collections::{HashMap, HashSet};

use tiny_skia::{Color, LineCap, LineJoin, Pixmap, Stroke};

use crate::{
    options::{Intersections, Lines, Marker, Point},
    pattern_utils::{
        Angle, AngleParseError, ConnectionPoint, Coord, Direction, DirectionParseError,
        DynamicList, HexCoord,
    },
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

    top_left_bound: HexCoord,
    bottom_right_bound: HexCoord,

    pub left_perimiter: Vec<Coord>,
    pub right_perimiter: Vec<Coord>,

    pub points: Vec<Coord>,
    pub angles: Vec<Angle>,

    pub collisions: HashMap<ConnectionPoint, i32>,
}

impl Pattern {
    pub fn new(rotation: Direction, links: Vec<Angle>) -> Self {
        let mut path = vec![Coord(0, 0), Coord(0, 0) + rotation];
        let mut top_left = path[0].min_components(path[1]);
        let mut bottom_right = path[0].max_components(path[1]);

        let mut top_left_bound = HexCoord::from(path[0]).min_components(path[1].into());
        let mut bottom_right_bound = HexCoord::from(path[0]).max_components(path[1].into());

        let mut rotation = rotation;

        let mut left_perimiter = DynamicList::new();
        let mut right_perimiter = DynamicList::new();

        Self::add_to_perimiter(&mut left_perimiter, &mut right_perimiter, path[0]);
        Self::add_to_perimiter(&mut left_perimiter, &mut right_perimiter, path[1]);

        let mut collisions = HashMap::new();
        let mut connections = HashSet::new();

        connections.insert(ConnectionPoint::new(path[0], path[1]));

        for link in &links {
            rotation = rotation + *link;

            let last_point = *path.last().unwrap();
            let next_point = last_point + rotation;

            top_left = top_left.min_components(next_point);
            bottom_right = bottom_right.max_components(next_point);

            top_left_bound = top_left_bound.min_components(next_point.into());
            bottom_right_bound = bottom_right_bound.max_components(next_point.into());

            Self::add_to_perimiter(&mut left_perimiter, &mut right_perimiter, next_point);
            path.push(next_point);

            let connection_point = ConnectionPoint::new(next_point, last_point);

            if let Some(collisions) = collisions.get_mut(&connection_point) {
                *collisions += 1;
            } else if connections.contains(&connection_point) {
                collisions.insert(connection_point, 1);
            } else {
                connections.insert(connection_point);
            }
        }

        let mut points = path.clone();
        points.dedup();
        Pattern {
            path,
            top_left,
            bottom_right,
            top_left_bound,
            bottom_right_bound,
            left_perimiter: left_perimiter.to_vector(),
            right_perimiter: right_perimiter.to_vector(),
            points,
            angles: links,
            collisions,
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

        let center = (self.bottom_right_bound + self.top_left_bound) / 2.0;
        let y_factor = 0.866025403784;

        let y_coord = (center.1 / y_factor).round() as i32;
        let x_coord = (center.0 - 0.5 * y_coord as f32) as i32;

        println!(
            "{:?}, {:?}, {:?}, {:?}, {:?}, {:?}",
            self.bottom_right,
            self.top_left,
            center,
            x_coord,
            y_coord,
            HexCoord::from(Coord(x_coord, y_coord))
        );
        let coord = Coord(x_coord, y_coord);
        if !self.points.contains(&coord) {
            draw_points(
                &vec![Coord(x_coord, y_coord)],
                pixmap,
                origin,
                scale,
                &Point::Single(Marker {
                    color: Color::WHITE,
                    radius: 0.07,
                }),
            );
        }
    }
}

#[derive(Debug, Clone)]
pub enum PatternParseError {
    InvalidParts(String),
    HangingHexPattern(String),
    InvalidStartDirection { input: String, direction: String },
    InvalidAngle { input: String, angle: char },
}
impl TryFrom<&str> for Pattern {
    type Error = PatternParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut parts: Vec<&str> = value.trim().split(' ').collect();

        if parts.len() != 2 {
            return Err(Self::Error::InvalidParts(value.to_string()));
        }

        if parts[0].to_lowercase().starts_with("hexpattern(") {
            if parts[1].ends_with(')') {
                parts[0] = &parts[0]["hexpattern(".len()..];
                parts[1] = &parts[1][..parts[1].len() - 1];
            } else {
                return Err(Self::Error::HangingHexPattern(value.to_string()));
            }
        }

        let direction: Direction =
            parts[0]
                .try_into()
                .map_err(|err| PatternParseError::InvalidStartDirection {
                    input: value.to_string(),
                    direction: match err {
                        DirectionParseError::InvalidNumber(_) => unreachable!(),
                        DirectionParseError::InvalidStr(str) => str,
                    },
                })?;

        let angles: Vec<Angle> = parts[1]
            .chars()
            .map(|a| Angle::try_from(a))
            .collect::<Result<Vec<Angle>, AngleParseError>>()
            .map_err(|err| PatternParseError::InvalidAngle {
                input: value.to_string(),
                angle: err.0,
            })?;

        return Ok(Pattern::new(direction, angles));
    }
}
