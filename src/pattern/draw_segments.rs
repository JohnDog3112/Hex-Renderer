use std::collections::HashMap;

use tiny_skia::{
    Color, LineCap, LineJoin, Paint, PathBuilder, Pixmap, Stroke, StrokeDash, Transform,
};

use crate::{
    options::{CollisionOption, Marker, Triangle},
    pattern::text::draw_text,
    pattern_utils::{ConnectionPoint, Coord, HexCoord, LineDrawer},
};

use super::{
    point::draw_point,
    triangle::{draw_triangle, rotate_point},
    Pattern,
};

#[allow(clippy::too_many_arguments)]
pub fn draw_segment_lines(
    pattern: &Pattern,
    pixmap: &mut Pixmap,
    stroke: &Stroke,
    origin: HexCoord,
    scale: f32,
    colors: &Vec<Color>,
    triangles: &Triangle,
    point_radius: f32,
    collisions: &CollisionOption,
) -> Color {
    let point_radius = stroke.width.max(scale * point_radius);
    let mut visited_points: HashMap<Coord, Vec<usize>> = HashMap::new();

    let mut travelled_collisions: HashMap<ConnectionPoint, Vec<bool>> = HashMap::new();

    let mut cur_color = 0;

    let mut triangle_queue: Vec<(crate::options::Point, HexCoord, HexCoord, f32)> = Vec::new();

    let mut last_collision_lane = None;

    let mut paint = Paint::default();
    paint.set_color(colors[0]);
    let mut drawer = LineDrawer::new(origin, stroke.clone(), paint);

    let mut prev_loc = origin;
    let mut prev_point = Coord(0, 0);

    let (full_dash, stripes, too_many_lines, bad_color, label) = {
        let mut full_dash = false;
        let mut stripes = false;
        let mut too_many_lines = i32::MAX;
        let mut bad_color = Color::WHITE;
        let mut label = None;
        match collisions {
            CollisionOption::Dashes(color) => {
                full_dash = true;
                bad_color = *color;
            }
            CollisionOption::MatchedDashes => {
                stripes = true;
                too_many_lines = 1;
            }
            CollisionOption::ParallelLines => (),
            CollisionOption::OverloadedParallel { max_line, overload } => {
                too_many_lines = *max_line.max(&1) as i32;
                match overload {
                    crate::options::OverloadOptions::Dashes(color) => bad_color = *color,
                    crate::options::OverloadOptions::LabeledDashes {
                        color,
                        label: marker,
                    } => {
                        bad_color = *color;
                        label = Some(marker);
                    }
                    crate::options::OverloadOptions::MatchedDashes => stripes = true,
                }
            }
        }
        (full_dash, stripes, too_many_lines, bad_color, label)
    };

    let mut collision_stroke = stroke.clone();
    collision_stroke.dash = StrokeDash::new(vec![scale / 18.0, scale / 16.0], 0.0);
    collision_stroke.line_cap = LineCap::Butt;
    collision_stroke.line_join = LineJoin::Bevel;

    let mut ended_on_collision = false;

    let mut visited: HashMap<ConnectionPoint, (i32, Coord)> = HashMap::new();

    for i in 0..pattern.path.len() {
        let point = &pattern.path[i];
        let loc = origin + HexCoord::from(*point) * scale;

        ended_on_collision = false;

        if !visited_points.contains_key(point) {
            visited_points.insert(*point, Vec::new());
        }
        let visited_colors = visited_points.get_mut(point).unwrap();
        let connection_point = ConnectionPoint::new(
            *point,
            *pattern
                .path
                .get((i as i32 - 1).try_into().unwrap_or(0))
                .unwrap_or(&Coord(0, 0)),
        );

        let collisions = pattern.collisions.get(&connection_point).unwrap_or(&-1) + 1;

        let (visited_count, collision_start) =
            *visited.get(&connection_point).unwrap_or(&(0, *point));

        let (start, end, triangle_scale) = if collisions == 0 || full_dash {
            last_collision_lane = None;
            (prev_loc, loc, 1.0)
        } else {
            let mut collisions = collisions;
            if collisions > too_many_lines {
                collisions = too_many_lines;
            }
            let lane = add_lane(
                last_collision_lane,
                collisions,
                &mut travelled_collisions,
                connection_point.clone(),
            );

            last_collision_lane = Some(lane);

            let (start, end, line_width) =
                calculate_start_end(loc, prev_loc, stroke.width, collisions, lane);

            drawer.move_to(start);
            drawer.set_width(line_width);

            (start, end, line_width / stroke.width)
        };

        let not_draw_red = full_dash && visited_count > 0;
        let not_draw_lines = visited_count >= too_many_lines;
        let not_draw_stripes = stripes && collisions >= too_many_lines;
        let draw = !not_draw_red && !not_draw_lines && !not_draw_stripes;

        if draw && visited_colors.contains(&cur_color) {
            let middle = (end - start) / 2.0 + start;

            if let Some(marker) =
                triangles.to_middle_point(*colors.get(cur_color).unwrap_or(&bad_color))
            {
                triangle_queue.push((marker, middle, end, triangle_scale));
            }

            drawer.line_to(middle);

            cur_color = get_next_color(cur_color, visited_colors, colors.len());

            drawer.set_color(colors[cur_color]);
        }

        if draw {
            drawer.line_to(end);
        } else {
            drawer.move_to(end);
        }

        if stripes && collisions >= too_many_lines {
            let segment_length = (scale - point_radius * 2.0) / (collisions as f32 + 1.0) / 2.0;
            let start_offset = segment_length * (visited_count as f32 + 1.0) * 2.0 + point_radius;
            let end_offset = start_offset + segment_length;

            let (mut start, mut end) = (prev_loc, loc);
            if collision_start == prev_point {
                (start, end) = (loc, prev_loc);
            }
            let unit_vec = (end - start).unit_vec();

            let start_seg = unit_vec * start_offset + start;
            let end_seg = unit_vec * end_offset + start;

            let mut stroke = stroke.clone();
            stroke.line_cap = LineCap::Butt;
            stroke.line_join = LineJoin::Miter;

            drawer.set_stroke(stroke);

            if visited_count == 0 {
                let start_set = start + unit_vec * (point_radius + segment_length);
                drawer.move_to(start);
                drawer.line_to(start_set);
            }
            if visited_colors.contains(&cur_color) {
                cur_color = get_next_color(cur_color, visited_colors, colors.len());
            }
            drawer.move_to(start_seg);
            drawer.set_color(colors[cur_color]);

            drawer.line_to(end_seg);
        } else if (full_dash && collisions > 0 || collisions >= too_many_lines)
            && !visited.contains_key(&connection_point)
        {
            drawer.set_stroke(collision_stroke.clone());
            drawer.set_color(bad_color);
            drawer.move_to(prev_loc);
            drawer.line_to(loc);
            drawer.priority_finish();
            drawer.set_color(colors[cur_color]);

            if collisions >= too_many_lines && !full_dash {
                if let Some(label) = label {
                    draw_label(pixmap, label, prev_loc, loc, stroke, scale, collisions);
                }
            }
        }

        if collisions != 0 {
            if full_dash {
                ended_on_collision = true;
            }
            drawer.move_to(loc);
            drawer.set_stroke(stroke.clone());
            if let Some((count, _)) = visited.get_mut(&connection_point) {
                *count += 1;
            } else {
                visited.insert(connection_point, (1, collision_start));
            }
        }

        if visited_colors.len() != colors.len() {
            visited_colors.push(cur_color);
        }
        prev_loc = loc;
        prev_point = *point;
    }

    drawer.draw(pixmap);

    for (triangle, location, next, scaler) in triangle_queue {
        draw_triangle(triangle, pixmap, location, next, scale * scaler);
    }

    let cur_loc = origin + HexCoord::from(pattern.path[1]) * scale;
    let prev_loc = origin + HexCoord::from(pattern.path[0]) * scale;
    let mid_point = (cur_loc - prev_loc) / 2.0 + prev_loc;

    if let Some(marker) = triangles.to_start_point(colors[0]) {
        draw_triangle(marker, pixmap, mid_point, cur_loc, scale);
    }
    drawer.draw_priority(pixmap);

    if !ended_on_collision {
        colors[cur_color]
    } else {
        bad_color
    }
}

fn get_next_color(cur_color: usize, visited: &Vec<usize>, color_count: usize) -> usize {
    if visited.len() >= color_count {
        (cur_color + 1) % color_count
    } else {
        let mut col = cur_color;
        for color in 0..color_count {
            let color = (cur_color + color) % color_count;
            if !visited.contains(&color) {
                col = color;
                break;
            }
        }
        col
    }
}

fn add_lane(
    last_collision_lane: Option<i32>,
    collisions: i32,
    travelled_collisions: &mut HashMap<ConnectionPoint, Vec<bool>>,
    connection_point: ConnectionPoint,
) -> i32 {
    let preferred_lane = last_collision_lane.unwrap_or(0);

    let mut lane = preferred_lane % collisions;
    if let Some(visited) = travelled_collisions.get_mut(&connection_point) {
        if visited[lane as usize] {
            for (j, &visited) in visited.iter().enumerate() {
                if !visited {
                    lane = j as i32;
                    break;
                }
            }
        }
        visited[lane as usize] = true;
    } else {
        let mut vec = vec![false; collisions as usize];
        vec[lane as usize] = true;
        travelled_collisions.insert(connection_point, vec);
    }
    lane
}

fn calculate_start_end(
    loc: HexCoord,
    prev_loc: HexCoord,
    width: f32,
    collisions: i32,
    lane: i32,
) -> (HexCoord, HexCoord, f32) {
    let line_space = width * 1.0;

    let segment_space = line_space / collisions as f32;

    let line_width = segment_space / 2.0;

    //line_width = width * x / collisions / 2.0

    // x = line_width/width * collisions * 2.0

    let line_loc = segment_space * lane as f32;

    let offset = line_loc - line_space / 2.0 + line_width;

    let mut offset_vec = (loc - prev_loc).unit_vec() * offset;
    // a / 2.0 + a * lane

    if loc.0 < prev_loc.0 || (loc.0 == prev_loc.0 && loc.1 < prev_loc.1) {
        offset_vec = offset_vec * -1.0;
    }

    let start = rotate_point(prev_loc, prev_loc + offset_vec, 90f32.to_radians());
    let end = rotate_point(loc, loc + offset_vec, 90f32.to_radians());

    (start, end, line_width)
}

fn draw_label(
    pixmap: &mut Pixmap,
    label: &Marker,
    prev_loc: HexCoord,
    loc: HexCoord,
    stroke: &Stroke,
    scale: f32,
    collisions: i32,
) {
    let radius = label.radius * scale;

    let offset = (loc - prev_loc).unit_vec() * (stroke.width / 2.0 + radius);
    let line_offset = (loc - prev_loc).unit_vec() * stroke.width / 2.0;

    let middle = (loc - prev_loc) / 2.0 + prev_loc;

    let point = rotate_point(middle, middle + offset, -90f32.to_radians());
    let line_point = rotate_point(middle, middle + line_offset, -90f32.to_radians());

    let mut paint = Paint::default();
    paint.set_color(label.color);

    let stroke = Stroke {
        width: radius * 2.0,
        line_cap: LineCap::Butt,
        ..Default::default()
    };
    //stroke.width = radius * 2.0;
    //stroke.line_cap = LineCap::Butt;

    let mut path = PathBuilder::new();
    path.move_to(line_point.0, line_point.1);
    path.line_to(point.0, point.1);
    let path = path.finish().unwrap();

    pixmap.stroke_path(&path, &paint, &stroke, Transform::identity(), None);

    draw_point(pixmap, point, radius, label.color);
    draw_text(
        pixmap,
        &format!("{collisions}"),
        Color::BLACK,
        point,
        radius,
    );
}
