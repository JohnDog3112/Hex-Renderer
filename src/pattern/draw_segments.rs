use std::collections::HashMap;

use tiny_skia::{Color, Paint, PathBuilder, Pixmap, Stroke, Transform};

use crate::{
    options::Triangle,
    pattern_utils::{Coord, HexCoord},
};

use super::{triangle::draw_triangle, Pattern};

pub fn draw_segment_lines(
    pattern: &Pattern,
    pixmap: &mut Pixmap,
    stroke: &Stroke,
    origin: HexCoord,
    scale: f32,
    colors: &Vec<Color>,
    triangles: &Triangle,
) -> Color {
    let mut visited_points: HashMap<Coord, Vec<usize>> = HashMap::new();

    let mut cur_path_builder = PathBuilder::new();
    cur_path_builder.move_to(origin.0, origin.1);

    let mut cur_color = 0;

    let mut paths = Vec::new();
    let mut triangle_queue = Vec::new();

    for i in 0..pattern.path.len() {
        let point = &pattern.path[i];
        let loc = origin + HexCoord::from(*point) * scale;

        if !visited_points.contains_key(point) {
            visited_points.insert(*point, Vec::new());
        }
        let visited_colors = visited_points.get_mut(point).unwrap();

        if !visited_colors.contains(&cur_color) {
            cur_path_builder.line_to(loc.0, loc.1);
            visited_colors.push(cur_color);
        } else {
            let prev_loc = origin + HexCoord::from(pattern.path[i - 1]) * scale;

            let middle = (loc - prev_loc) / 2.0 + prev_loc;

            cur_path_builder.line_to(middle.0, middle.1);
            paths.push((cur_color, cur_path_builder.finish().unwrap()));

            cur_path_builder = PathBuilder::new();
            cur_path_builder.move_to(middle.0, middle.1);
            cur_path_builder.line_to(loc.0, loc.1);

            if visited_colors.len() == colors.len() {
                cur_color = (cur_color + 1) % colors.len();
            } else {
                for color in 0..colors.len() {
                    let color = (cur_color + color) % colors.len();
                    if !visited_colors.contains(&color) {
                        cur_color = color;
                        break;
                    }
                }
            }

            visited_colors.push(cur_color);

            if let Some(marker) = triangles.to_middle_point(colors[cur_color]) {
                triangle_queue.push((marker, middle, loc));
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
        draw_triangle(triangle, pixmap, location, next, scale);
    }

    let cur_loc = origin + HexCoord::from(pattern.path[1]) * scale;
    let prev_loc = origin + HexCoord::from(pattern.path[0]) * scale;
    let mid_point = (cur_loc - prev_loc) / 2.0 + prev_loc;

    if let Some(marker) = triangles.to_start_point(colors[0]) {
        draw_triangle(marker, pixmap, mid_point, cur_loc, scale);
    }

    colors[cur_color]
}
