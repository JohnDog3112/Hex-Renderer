use std::collections::HashMap;

use tiny_skia::{Pixmap, Stroke, Color, Paint, GradientStop, LinearGradient, SpreadMode, Transform, PathBuilder};

use crate::pattern_utils::{HexCoord, Coord};

use super::Pattern;


pub fn draw_gradient_lines(pattern: &Pattern, pixmap: &mut Pixmap, stroke: &Stroke,
    origin: HexCoord, scale: f32, 
    colors: &Vec<Color>, segs_per_color: usize, bent_corners: bool
) -> Color {
    let segments = pattern.path.len() as f32 - 1.0;

    let mut grad_colors = Vec::new();

    for i in 0..colors.len().min(pattern.path.len()/segs_per_color + 2) {
        let col = colors[i];
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

    let mut loc_prev = origin + HexCoord::from(pattern.path[0]) * scale;

    paint.anti_alias = false;

    let mut path_queue = Vec::new();

    let mut visit_count: HashMap<Coord, usize> = HashMap::new();

    if bent_corners {
        for path in &pattern.path {
            if let Some(count) = visit_count.get_mut(path) {
                *count += 1;
            } else {
                visit_count.insert(*path, 1);
            }
        }
    }
    for i in 1..pattern.path.len() {
        let mut loc_next = origin + HexCoord::from(pattern.path[i]) * scale;
        //let loc_prev = origin + HexCoord::from(pattern.path[i-1]) * scale;

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

        if bent_corners && visit_count.get(&pattern.path[i]).unwrap() > &1 && pattern.path.len()-1 != i {
            let bend_amount = 0.2;

            let stop_point = loc_next - (loc_next - loc_prev)* bend_amount;
            pb.line_to(stop_point.0, stop_point.1);

            loc_next = loc_next + (origin + HexCoord::from(pattern.path[i+1])*scale - loc_next) * bend_amount;
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

    return colors[grad_colors.len()-1];
}