use std::collections::HashMap;

use tiny_skia::{Color, Paint, Pixmap, Stroke};

use crate::pattern_utils::{Coord, HexCoord, LineDrawer};

use super::Pattern;

pub fn draw_monocolor_lines(
    pattern: &Pattern,
    pixmap: &mut Pixmap,
    stroke: &Stroke,
    origin: HexCoord,
    scale: f32,
    color: Color,
    bent_corners: bool,
) {
    let mut paint = Paint::default();
    paint.set_color(color);

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

    let mut line_drawer = LineDrawer::new(origin, stroke.clone(), paint);

    for (i, line) in pattern.path.iter().enumerate() {
        let current = HexCoord::from(*line) * scale + origin;

        if bent_corners
            && ((visit_count.get(&pattern.path[i]).unwrap() > &1 && i != 0)
                || pattern.path.len() - 1 == i)
        {
            let next = current;

            let current = HexCoord::from(pattern.path[i - 1]) * scale + origin;
            let bend_amount = 0.2;

            let stop_point = next - (next - current) * bend_amount;
            line_drawer.line_to(stop_point);

            if pattern.path.len() - 1 != i {
                line_drawer.line_to(
                    next + (origin + HexCoord::from(pattern.path[i + 1]) * scale - next)
                        * bend_amount,
                );
            }
        } else {
            line_drawer.line_to(current);
        }
    }

    line_drawer.draw_all(pixmap);
}
