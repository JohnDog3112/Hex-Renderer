use tiny_skia::{Color, FillRule, Paint, PathBuilder, Pixmap, Transform};

use crate::{
    options::Point,
    pattern_utils::{Coord, HexCoord},
};

pub fn draw_points(
    points: &Vec<Coord>,
    pixmap: &mut Pixmap,
    origin: HexCoord,
    scale: f32,
    point: &Point,
) {
    let mut paint = Paint::default();
    let mut paint2 = Paint::default();

    match point {
        Point::None => (),
        Point::Single(marker) => {
            paint.set_color(marker.color);
            for point in points {
                let loc = HexCoord::from(*point) * scale + origin;
                let path = PathBuilder::from_circle(loc.0, loc.1, marker.radius * scale).unwrap();
                pixmap.fill_path(&path, &paint, FillRule::Winding, Transform::default(), None);
            }
        }
        Point::Double { inner, outer } => {
            paint.set_color(outer.color);
            paint2.set_color(inner.color);
            for point in points {
                let loc = HexCoord::from(*point) * scale + origin;
                let path = PathBuilder::from_circle(loc.0, loc.1, outer.radius * scale).unwrap();
                pixmap.fill_path(&path, &paint, FillRule::Winding, Transform::default(), None);

                let path = PathBuilder::from_circle(loc.0, loc.1, inner.radius * scale).unwrap();
                pixmap.fill_path(
                    &path,
                    &paint2,
                    FillRule::Winding,
                    Transform::default(),
                    None,
                );
            }
        }
    }
}

pub fn draw_point(pixmap: &mut Pixmap, center: HexCoord, radius: f32, color: Color) {
    let path = PathBuilder::from_circle(center.0, center.1, radius).unwrap();
    let mut paint = Paint::default();
    paint.set_color(color);
    pixmap.fill_path(&path, &paint, FillRule::Winding, Transform::default(), None);
}
