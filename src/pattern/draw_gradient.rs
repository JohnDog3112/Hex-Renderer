use std::collections::HashMap;

use tiny_skia::{
    Color, GradientStop, LinearGradient, Paint, Pixmap, SpreadMode, Stroke, Transform,
};

use crate::pattern_utils::{Coord, HexCoord, LineDrawer};

use super::Pattern;

#[allow(clippy::too_many_arguments)]
pub fn draw_gradient_lines(
    pattern: &Pattern,
    pixmap: &mut Pixmap,
    stroke: &Stroke,
    origin: HexCoord,
    scale: f32,
    colors: &[Color],
    segs_per_color: usize,
    bent_corners: bool,
) -> Color {
    let segments = pattern.path.len() as f32 - 1.0;

    let mut grad_colors = Vec::new();

    for col in colors.iter().take(pattern.path.len() / segs_per_color + 2) {
        //let col = colors[i];
        grad_colors.push([col.red(), col.green(), col.blue(), col.alpha()]);
    }

    let mut grad_diffs = Vec::new();

    for i in 1..grad_colors.len() {
        let mut col = [0.0; 4];
        for (j, col) in col.iter_mut().enumerate() {
            *col = grad_colors[i][j] - grad_colors[i - 1][j];
        }
        grad_diffs.push(col);
    }

    let mut cur_color = grad_colors[0];

    let grad_segments = grad_colors.len() - 1;

    let mut loc_prev = origin + HexCoord::from(pattern.path[0]) * scale;

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
    let paint = Paint::<'_> {
        anti_alias: false,
        ..Default::default()
    };
    let mut line_drawer = LineDrawer::new(origin, stroke.clone(), paint);

    let mut prev_shade_color =
        Color::from_rgba(cur_color[0], cur_color[1], cur_color[2], cur_color[3]).unwrap();

    for i in 1..pattern.path.len() {
        let mut loc_next = origin + HexCoord::from(pattern.path[i]) * scale;

        let progress = (i - 1) as f32 / segments;
        let grad_seg = (progress * grad_segments as f32) as usize;

        let seg_progress =
            (progress - (grad_seg as f32 / grad_segments as f32)) * grad_segments as f32;

        for (j, cur_color) in cur_color.iter_mut().enumerate() {
            *cur_color =
                (grad_colors[grad_seg][j] + grad_diffs[grad_seg][j] * seg_progress).clamp(0.0, 1.0);
        }

        let cur_col =
            Color::from_rgba(cur_color[0], cur_color[1], cur_color[2], cur_color[3]).unwrap();

        line_drawer.set_shader(
            LinearGradient::new(
                tiny_skia::Point::from_xy(loc_prev.0, loc_prev.1),
                tiny_skia::Point::from_xy(loc_next.0, loc_next.1),
                vec![
                    GradientStop::new(0.0, prev_shade_color),
                    GradientStop::new(1.0, cur_col),
                ],
                SpreadMode::Pad,
                Transform::identity(),
            )
            .unwrap(),
        );

        if bent_corners
            && visit_count.get(&pattern.path[i]).unwrap() > &1
            && pattern.path.len() - 1 != i
        {
            let bend_amount = 0.2;

            let stop_point = loc_next - (loc_next - loc_prev) * bend_amount;
            line_drawer.line_to(stop_point);

            loc_next = loc_next
                + (origin + HexCoord::from(pattern.path[i + 1]) * scale - loc_next) * bend_amount;
        }
        line_drawer.line_to(loc_next);

        loc_prev = loc_next;
        prev_shade_color = cur_col;
    }

    line_drawer.draw_all(pixmap);

    colors[grad_colors.len() - 1]
}
