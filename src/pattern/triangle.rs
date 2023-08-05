use tiny_skia::{Pixmap, Paint, FillRule, Transform, Path, PathBuilder};

use crate::{draw_options::Marker, pattern_utils::HexCoord};


pub fn draw_triangle(triangle: Marker, pixmap: &mut Pixmap, location: HexCoord, next: HexCoord, scale: f32) {
    let mut paint = Paint::default();

    match triangle {
        Marker::None => (),
        Marker::SinglePoint(color, radius) => {
            let path = generate_triangle_path(location, next, radius * scale);
            paint.set_color(color);
            pixmap.fill_path(&path, &paint, FillRule::Winding, Transform::default(), None);
        },
        Marker::DoublePoint(c1, r1, c2, r2) => {
            let path = generate_triangle_path(location, next, r1 * scale);
            paint.set_color(c1);
            pixmap.fill_path(&path, &paint, FillRule::Winding, Transform::default(), None);

            let path = generate_triangle_path(location, next, r2 * scale);
            paint.set_color(c2);
            pixmap.fill_path(&path, &paint, FillRule::Winding, Transform::default(), None);
        },
    }
}

fn generate_triangle_path(location: HexCoord, next: HexCoord, radius: f32) -> Path {
    let dir = next - location;
    let magnitude = (dir.0*dir.0 + dir.1*dir.1).sqrt();

    let point1 = location + dir/magnitude * radius;
    let point2 = rotate_point(location, point1, (-120.0f32).to_radians());
    let point3 = rotate_point(location, point1, (120.0f32).to_radians());


    let mut path = PathBuilder::new();
    path.move_to(point1.0, point1.1);
    path.line_to(point2.0, point2.1);
    path.line_to(point3.0, point3.1);
    path.line_to(point1.0, point1.1);

    path.finish().unwrap()
}

fn rotate_point(center: HexCoord, point: HexCoord, angle: f32) -> HexCoord{
    let c = angle.cos();
    let s = angle.sin();

    let p = point - center;

    let new_x = p.0 * c - p.1 * s;
    let new_y = p.0 * s + p.1 * c;

    HexCoord(new_x, new_y) + center
}