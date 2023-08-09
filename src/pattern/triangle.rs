use tiny_skia::{FillRule, Paint, Path, PathBuilder, Pixmap, Transform};

use crate::{options::Point, pattern_utils::HexCoord};

pub fn draw_triangle(
    triangle: Point,
    pixmap: &mut Pixmap,
    location: HexCoord,
    next: HexCoord,
    scale: f32,
) {
    let mut paint = Paint::default();

    match triangle {
        Point::None => (),
        Point::Single(marker) => {
            let path = generate_triangle_path(location, next, marker.radius * scale);
            paint.set_color(marker.color);
            pixmap.fill_path(&path, &paint, FillRule::Winding, Transform::default(), None);
        }
        Point::Double { inner, outer } => {
            let path = generate_triangle_path(location, next, outer.radius * scale);
            paint.set_color(outer.color);
            pixmap.fill_path(&path, &paint, FillRule::Winding, Transform::default(), None);

            let path = generate_triangle_path(location, next, inner.radius * scale);
            paint.set_color(inner.color);
            pixmap.fill_path(&path, &paint, FillRule::Winding, Transform::default(), None);
        }
    }
}

fn generate_triangle_path(location: HexCoord, next: HexCoord, radius: f32) -> Path {
    let dir = next - location;
    let magnitude = (dir.0 * dir.0 + dir.1 * dir.1).sqrt();

    let point1 = location + dir / magnitude * radius;
    let point2 = rotate_point(location, point1, (-120.0f32).to_radians());
    //let point3 = location - dir / magnitude * radius / 6.0;
    let point4 = rotate_point(location, point1, (120.0f32).to_radians());

    let mut path = PathBuilder::new();
    path.move_to(point1.0, point1.1);
    path.line_to(point2.0, point2.1);
    //path.line_to(point3.0, point3.1);
    path.line_to(point4.0, point4.1);
    path.line_to(point1.0, point1.1);

    path.finish().unwrap()
}

pub fn rotate_point(center: HexCoord, point: HexCoord, angle: f32) -> HexCoord {
    let c = angle.cos();
    let s = angle.sin();

    let p = point - center;

    let new_x = p.0 * c - p.1 * s;
    let new_y = p.0 * s + p.1 * c;

    HexCoord(new_x, new_y) + center
}
