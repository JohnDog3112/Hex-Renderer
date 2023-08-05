use tiny_skia::{Pixmap, Paint, PathBuilder, FillRule, Transform};

use crate::{draw_options::Marker, pattern_utils::{Coord, HexCoord}};


pub fn draw_points(points: &Vec<Coord>, pixmap: &mut Pixmap, origin: HexCoord, scale: f32, point: &Marker) {
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