use tiny_skia::{Color, Paint, Pixmap, Stroke};

use crate::pattern_utils::{HexCoord, LineDrawer};

use super::Pattern;

pub fn draw_monocolor_lines(
    pattern: &Pattern,
    pixmap: &mut Pixmap,
    stroke: &Stroke,
    origin: HexCoord,
    scale: f32,
    color: Color,
) {
    let mut paint = Paint::default();
    paint.set_color(color);

    let mut line_drawer = LineDrawer::new(origin, stroke.clone(), paint);

    for line in &pattern.path {
        let current = HexCoord::from(*line) * scale + origin;
        line_drawer.line_to(current);
    }

    line_drawer.draw_all(pixmap);
}
