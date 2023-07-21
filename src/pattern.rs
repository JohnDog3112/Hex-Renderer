use std::collections::HashMap;

use tiny_skia::{Path, PathBuilder, Pixmap, Paint, Stroke, LineCap, Transform, FillRule, Color, LinearGradient, SpreadMode, GradientStop};

use crate::{coord::Coord, direction::Direction, angle::Angle, hex_coord::HexCoord, dynamic_list::DynamicList, draw_options::{Intersections, Lines, Marker, Triangle, GradientOptions}};

#[derive(Debug, Clone)]
pub struct Pattern {
    pub path: Vec<Coord>,
    pub top_left: Coord,
    pub bottom_right: Coord,
    
    pub left_perimiter: Vec<Coord>,
    pub right_perimiter: Vec<Coord>,

    pub points: Vec<Coord>,
}

impl Pattern {
    pub fn new(rotation: Direction, links: Vec<Angle>) -> Self {
        let mut path = vec![Coord(0,0), Coord(0,0) + rotation];
        let mut top_left = get_min_components(path[0], path[1]);
        let mut bottom_right = get_max_components(path[0], path[1]);

        let mut rotation = rotation;

        let mut left_perimiter = DynamicList::new();
        let mut right_perimiter = DynamicList::new();

        Self::add_to_perimiter(&mut left_perimiter, &mut right_perimiter, path[0]);
        Self::add_to_perimiter(&mut left_perimiter, &mut right_perimiter, path[1]);

        for link in links {
            rotation = rotation + link;

            let next_point = *path.last().unwrap() + rotation;

            top_left = get_min_components(top_left, next_point);
            bottom_right = get_max_components(bottom_right, next_point);
            Self::add_to_perimiter(&mut left_perimiter, &mut right_perimiter, next_point);
            path.push(next_point);
        }
        
        let mut points = path.clone();
        points.dedup();
        Pattern{
            path,
            top_left,
            bottom_right,
            left_perimiter: left_perimiter.to_vector(),
            right_perimiter: right_perimiter.to_vector(),
            points,
        }
    }
    fn add_to_perimiter(left_perimiter: &mut DynamicList<Coord>, right_perimiter: &mut DynamicList<Coord>, point: Coord) {
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


    pub fn draw_pattern(&self, pixmap: &mut Pixmap, origin: HexCoord, scale: f32, line_thickness: f32, line_options: &Lines, point_options: &Intersections) {
        let mut stroke = Stroke::default();
        stroke.width = line_thickness * scale;
        stroke.line_cap = LineCap::Round;

        let end_colors;
        
        match line_options {
            Lines::Monocolor(color) => {        
                self.draw_monocolor(pixmap, &stroke, origin, scale, *color);
                end_colors = (*color, *color);
            },
            Lines::Gradient(gradient_options) => {
                if gradient_options.colors.len() < 2 {
                    let col = *gradient_options.colors.get(0).unwrap_or(&Color::WHITE);
                    end_colors = (col, col);
                    self.draw_monocolor(pixmap, &stroke, origin, scale, col);
                } else {
                    end_colors = (
                        gradient_options.colors[0],
                        self.draw_gradient_lines(pixmap, &stroke, origin, scale, gradient_options)
                    );
                }
            },
            Lines::SegmentColors(colors, triangle) => {
                end_colors = (
                    colors[0],
                    self.draw_segment_lines(pixmap, &stroke, origin, scale, colors, triangle),
                );
            },
            
            
        }

        
        match point_options {
            Intersections::Nothing => (),
            Intersections::UniformPoints(point) => {
                Self::draw_points(&self.points, pixmap, origin, scale, &point);
            },
            Intersections::EndsAndMiddle(beginning, end, middle) => {
                let beginning_point = self.path[0];
                let ending_point = self.path[self.path.len()-1];

                let beginning = beginning.clone().into_point(end_colors.0);
                let end = end.clone().into_point(end_colors.1);
                
                Self::draw_points(&vec![beginning_point], pixmap, origin, scale, &beginning);
                if beginning_point != ending_point {
                    Self::draw_points(&vec![ending_point], pixmap, origin, scale, &end);
                }
                let middle_points: Vec<Coord> = self.points.clone().into_iter().filter(|&point| point != beginning_point && point != ending_point).collect();
                
                Self::draw_points(&middle_points, pixmap, origin, scale, &middle);
            },  
        }
    }

    pub fn draw_monocolor(&self, pixmap: &mut Pixmap, stroke: &Stroke,
        origin: HexCoord, scale: f32, 
        color: Color
    ) {
        let mut pb = PathBuilder::new();

        pb.move_to(origin.0, origin.1);

        for line in &self.path {
            let current = HexCoord::from(*line) * scale + origin;
            
            pb.line_to(current.0, current.1);
        }

        let path = pb.finish().unwrap();
        
        let mut paint = Paint::default();
        paint.set_color(color);
        pixmap.stroke_path(&path, &paint, &stroke, Transform::identity(), None);
    }

    fn draw_gradient_lines(&self, pixmap: &mut Pixmap, stroke: &Stroke,
        origin: HexCoord, scale: f32, 
        gradient_options: &GradientOptions,
    ) -> Color {
        let segments = self.path.len() as f32 - 1.0;

        let mut grad_colors = Vec::new();

        for i in 0..gradient_options.colors.len().min(self.path.len()/gradient_options.segs_per_color + 2) {
            let col = gradient_options.colors[i];
            grad_colors.push([col.red(), col.green(), col.blue(), col.alpha()]);
        }

        let mut grad_diffs = Vec::new();

        for i in 1..grad_colors.len() {
            let mut col = [0.0; 4];
            for j in 0..4 {
                col[j] = grad_colors[i][j]-grad_colors[i-1][j];
            }
            grad_diffs.push(col);
        }

        let mut cur_color = grad_colors[0];

        let grad_segments = grad_colors.len()-1;

        let mut paint = Paint::default();

        let mut loc_prev = origin + HexCoord::from(self.path[0]) * scale;

        paint.anti_alias = false;

        let mut path_queue = Vec::new();

        let mut visit_count: HashMap<Coord, usize> = HashMap::new();

        if gradient_options.bent_corners {
            for path in &self.path {
                if let Some(count) = visit_count.get_mut(path) {
                    *count += 1;
                } else {
                    visit_count.insert(*path, 1);
                }
            }
        }
        for i in 1..self.path.len() {
            let mut loc_next = origin + HexCoord::from(self.path[i]) * scale;
            //let loc_prev = origin + HexCoord::from(self.path[i-1]) * scale;

            let mut stops = vec![GradientStop::new(0.0, Color::from_rgba(cur_color[0], cur_color[1], cur_color[2], cur_color[3]).unwrap())];

            let progress = (i-1) as f32/segments;
            let grad_seg = (progress * grad_segments as f32) as usize;

            let seg_progress = (progress - (grad_seg as f32 / grad_segments as f32)) * grad_segments as f32;


            for j in 0..4 {
                cur_color[j] = (grad_colors[grad_seg][j]  + grad_diffs[grad_seg][j] * seg_progress).clamp(0.0, 1.0);
            }
            //println!("{}", seg_progress);


            stops.push(GradientStop::new(1.0, Color::from_rgba(cur_color[0], cur_color[1], cur_color[2], cur_color[3]).unwrap()));
            
            let shader = LinearGradient::new(
                tiny_skia::Point::from_xy(loc_prev.0, loc_prev.1),
                tiny_skia::Point::from_xy(loc_next.0, loc_next.1),
                stops,
                SpreadMode::Pad,
                Transform::identity(),
            ).unwrap();

            let mut pb = PathBuilder::new();

            pb.move_to(loc_prev.0, loc_prev.1);

            if gradient_options.bent_corners && visit_count.get(&self.path[i]).unwrap() > &1 && self.path.len()-1 != i {
                let bend_amount = 0.2;

                let stop_point = loc_next - (loc_next - loc_prev)* bend_amount;
                pb.line_to(stop_point.0, stop_point.1);

                loc_next = loc_next + (origin + HexCoord::from(self.path[i+1])*scale - loc_next) * bend_amount;
                pb.line_to(loc_next.0, loc_next.1);
            } else {
                pb.line_to(loc_next.0, loc_next.1);
            }
            
            let path = pb.finish().unwrap();

            path_queue.push((path, shader));

            loc_prev = loc_next;
        }

        for (path, shader) in path_queue.into_iter().rev() {
            paint.shader = shader;

            pixmap.stroke_path(&path, &paint, stroke, Transform::identity(), None);
        }

        return gradient_options.colors[grad_colors.len()-1];
    }

    fn draw_segment_lines(&self, pixmap: &mut Pixmap, stroke: &Stroke,
        origin: HexCoord, scale: f32, 
        colors: &Vec<Color>, triangles: &Triangle,
    ) -> Color {
        let mut visited_points: HashMap<Coord, Vec<usize>> = HashMap::new();

        let mut cur_path_builder = PathBuilder::new();
        cur_path_builder.move_to(origin.0, origin.1);

        let mut cur_color = 0;

        let mut paths = Vec::new();
        let mut triangle_queue = Vec::new();

        for i in 0..self.path.len() {
            let point = &self.path[i];
            let loc = origin + HexCoord::from(*point) * scale;
            
            if !visited_points.contains_key(point) {
                visited_points.insert(*point, Vec::new());
            }
            let visited_colors = visited_points.get_mut(point).unwrap();

            if !visited_colors.contains(&cur_color) {
                cur_path_builder.line_to(loc.0, loc.1);
                visited_colors.push(cur_color);
            } else {
                let prev_loc = origin + HexCoord::from(self.path[i-1]) * scale;
                
                let middle = (loc - prev_loc)/2.0 + prev_loc;

                cur_path_builder.line_to(middle.0, middle.1);
                paths.push((cur_color, cur_path_builder.finish().unwrap()));


                cur_path_builder = PathBuilder::new();
                cur_path_builder.move_to(middle.0, middle.1);
                cur_path_builder.line_to(loc.0, loc.1);

                if visited_colors.len() == colors.len() {
                    cur_color = (cur_color+1)%colors.len();
                } else {
                    for color in 0..colors.len() {
                        let color = (cur_color+color)%colors.len();
                        if !visited_colors.contains(&color) {
                            cur_color = color;
                            break;
                        }
                    }
                }
                
                visited_colors.push(cur_color);

                let color = colors[cur_color];
                match triangles {
                    Triangle::None => (),
                    Triangle::Match(r) | Triangle::BorderStartMatch(r, _, _) => {
                        triangle_queue.push((Marker::SinglePoint(color, *r), middle, loc));
                    },
                    Triangle::BorderMatch(r1, col, r2) => {
                        let marker;
                        if r1 > r2 {
                            marker = Marker::DoublePoint(color, *r1, *col, *r2);
                        } else {
                            marker = Marker::DoublePoint(*col, *r2, color, *r1);
                        }
                        triangle_queue.push((marker, middle, loc));
                    },
                }
            }
        }
        let mut paint = Paint::default();

        if let Some(path) = cur_path_builder.finish() {
            paint.set_color(colors[cur_color]);
            pixmap.stroke_path(&path, &paint, stroke, Transform::identity(), None);
        }
        
        for (color_index, path) in paths.iter().rev() {
            paint.set_color(colors[*color_index]);
            pixmap.stroke_path(&path, &paint, stroke, Transform::identity(), None);
        }

        for (triangle, location, next) in triangle_queue {
            Self::draw_triangle(triangle, pixmap, location, next, scale);
        }
        
        let cur_loc = origin + HexCoord::from(self.path[1]) * scale;
        let prev_loc = origin + HexCoord::from(self.path[0]) * scale;
        let mid_point = (cur_loc - prev_loc)/2.0 + prev_loc;
        match triangles {
            Triangle::None => (),
            Triangle::Match(r) => {
                Self::draw_triangle(Marker::SinglePoint(colors[0], *r), pixmap, mid_point, cur_loc, scale);
            }
            Triangle::BorderMatch(r1, c2, r2) 
            | Triangle::BorderStartMatch(r1, c2, r2) => {
                let marker;
                if r1 > r2 {
                    marker = Marker::DoublePoint(colors[0], *r1, *c2, *r2);
                } else {
                    marker = Marker::DoublePoint(*c2, *r2, colors[0], *r1);
                }
                Self::draw_triangle(marker, pixmap, mid_point, cur_loc, scale);
            },
        }

        colors[cur_color]
    }

    pub fn draw_triangle(triangle: Marker, pixmap: &mut Pixmap, location: HexCoord, next: HexCoord, scale: f32) {
        let mut paint = Paint::default();

        match triangle {
            Marker::None => (),
            Marker::SinglePoint(color, radius) => {
                let path = Self::generate_triangle_path(location, next, radius * scale);
                paint.set_color(color);
                pixmap.fill_path(&path, &paint, FillRule::Winding, Transform::default(), None);
            },
            Marker::DoublePoint(c1, r1, c2, r2) => {
                let path = Self::generate_triangle_path(location, next, r1 * scale);
                paint.set_color(c1);
                pixmap.fill_path(&path, &paint, FillRule::Winding, Transform::default(), None);

                let path = Self::generate_triangle_path(location, next, r2 * scale);
                paint.set_color(c2);
                pixmap.fill_path(&path, &paint, FillRule::Winding, Transform::default(), None);
            },
        }
    }

    pub fn generate_triangle_path(location: HexCoord, next: HexCoord, radius: f32) -> Path{
        let dir = next - location;
        let magnitude = (dir.0*dir.0 + dir.1*dir.1).sqrt();

        let point1 = location + dir/magnitude * radius;
        let point2 = Self::rotate_point(location, point1, (-120.0f32).to_radians());
        let point3 = Self::rotate_point(location, point1, (120.0f32).to_radians());


        let mut path = PathBuilder::new();
        path.move_to(point1.0, point1.1);
        path.line_to(point2.0, point2.1);
        path.line_to(point3.0, point3.1);
        path.line_to(point1.0, point1.1);

        path.finish().unwrap()
    }

    pub fn rotate_point(center: HexCoord, point: HexCoord, angle: f32) -> HexCoord{
        let c = angle.cos();
        let s = angle.sin();

        let p = point - center;

        let new_x = p.0 * c - p.1 * s;
        let new_y = p.0 * s + p.1 * c;

        HexCoord(new_x, new_y) + center
    }

    fn draw_points(points: &Vec<Coord>, pixmap: &mut Pixmap, origin: HexCoord, scale: f32, point: &Marker) {
        let mut paint = Paint::default();

        let mut paint2 = Paint::default();

        match point {
            Marker::None => (),
            Marker::SinglePoint(color, radius) => {
                paint.set_color(*color);
                for point in points {
                    let loc = HexCoord::from(*point) * scale + origin;
                    let path = PathBuilder::from_circle(loc.0, loc.1, *radius * scale).unwrap();
                    pixmap.fill_path(&path, &paint, FillRule::Winding, Transform::default(), None);
                }
            },
            Marker::DoublePoint(color1, radius1, color2, radius2) => {
                paint.set_color(*color1);
                paint2.set_color(*color2);
                for point in points {
                    let loc = HexCoord::from(*point) * scale + origin;
                    let path = PathBuilder::from_circle(loc.0, loc.1, *radius1 * scale).unwrap();
                    pixmap.fill_path(&path, &paint, FillRule::Winding, Transform::default(), None);

                    let path = PathBuilder::from_circle(loc.0, loc.1, *radius2 * scale).unwrap();
                    pixmap.fill_path(&path, &paint2, FillRule::Winding, Transform::default(), None);
                }
            },
        }
    }


    
}

fn get_min_components(a: Coord, b: Coord) -> Coord {
    let mut res = a;
    if b.0 < res.0 {
        res.0 = b.0;
    }
    if b.1 < res.1 {
        res.1 = b.1;
    }
    res
}
fn get_max_components(a: Coord, b: Coord) -> Coord {
    let mut res = a;
    if b.0 > res.0 {
        res.0 = b.0;
    }
    if b.1 > res.1 {
        res.1 = b.1;
    }
    res
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
                parts[1] = &parts[1][..parts[1].len()-1];
            } else {
                return Err(());
            }
        }

        let direction: Direction = parts[0].try_into()?;

        let angles: Vec<Angle> = parts[1].chars().map(|a| Angle::try_from(a)).collect::<Result<Vec<Angle>, _>>()?;

        return Ok(Pattern::new(direction, angles));
    }
}