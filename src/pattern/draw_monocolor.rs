use tiny_skia::{Pixmap, Stroke, Color, PathBuilder, Paint, Transform};

use crate::pattern_utils::HexCoord;

use super::Pattern;

pub fn draw_monocolor_lines(pattern: &Pattern, pixmap: &mut Pixmap, stroke: &Stroke,
    origin: HexCoord, scale: f32, 
    color: Color
) {
    let mut pb = PathBuilder::new();

    pb.move_to(origin.0, origin.1);

    for line in &pattern.path {
        let current = HexCoord::from(*line) * scale + origin;
        
        pb.line_to(current.0, current.1);
    }

    let path = pb.finish().unwrap();
    
    let mut paint = Paint::default();
    paint.set_color(color);
    pixmap.stroke_path(&path, &paint, &stroke, Transform::identity(), None);
}