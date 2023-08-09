use tiny_skia::{Color, Paint, Path, PathBuilder, Pixmap, Stroke, Transform};

use super::HexCoord;

pub struct LineDrawer<'a> {
    prev_point: HexCoord,
    paths: Vec<(Path, Stroke, Paint<'a>)>,
    priority_paths: Vec<(Path, Stroke, Paint<'a>)>,
    path: PathBuilder,
    stroke: Stroke,
    paint: Paint<'a>,
}
impl<'a> LineDrawer<'a> {
    pub fn new(start_point: HexCoord, stroke: Stroke, paint: Paint<'a>) -> Self {
        let mut path = PathBuilder::new();
        path.move_to(start_point.0, start_point.1);
        Self {
            prev_point: start_point,
            paths: Vec::new(),
            priority_paths: Vec::new(),
            path,
            stroke,
            paint,
        }
    }
    pub fn line_to(&mut self, point: HexCoord) {
        self.prev_point = point;
        self.path.line_to(point.0, point.1);
    }

    fn new_path(&mut self, start_point: HexCoord, mut stroke: Stroke, mut paint: Paint<'a>) {
        let mut tmp_path = PathBuilder::new();
        tmp_path.move_to(start_point.0, start_point.1);

        self.prev_point = start_point;

        std::mem::swap(&mut tmp_path, &mut self.path);
        std::mem::swap(&mut paint, &mut self.paint);
        std::mem::swap(&mut stroke, &mut self.stroke);
        if let Some(path) = tmp_path.finish() {
            self.paths.push((path, stroke, paint));
        }
    }

    pub fn move_to(&mut self, point: HexCoord) {
        self.new_path(point, self.stroke.clone(), self.paint.clone());
    }

    pub fn set_color(&mut self, color: Color) {
        self.new_path(self.prev_point, self.stroke.clone(), self.paint.clone());
        self.paint.set_color(color);
    }
    pub fn set_width(&mut self, width: f32) {
        self.new_path(self.prev_point, self.stroke.clone(), self.paint.clone());
        self.stroke.width = width;
    }

    pub fn set_stroke(&mut self, stroke: Stroke) {
        self.new_path(self.prev_point, stroke, self.paint.clone());
    }

    pub fn priority_finish(&mut self) {
        let mut tmp = PathBuilder::new();
        tmp.move_to(self.prev_point.0, self.prev_point.1);

        std::mem::swap(&mut tmp, &mut self.path);

        if let Some(path) = tmp.finish() {
            self.priority_paths
                .push((path, self.stroke.clone(), self.paint.clone()));
        }
    }

    pub fn draw(&mut self, pixmap: &mut Pixmap) {
        self.new_path(self.prev_point, self.stroke.clone(), self.paint.clone());
        for (path, stroke, paint) in self.paths.iter().rev() {
            pixmap.stroke_path(&path, &paint, &stroke, Transform::identity(), None)
        }
    }
    pub fn draw_priority(self, pixmap: &mut Pixmap) {
        for (path, stroke, paint) in self.priority_paths {
            pixmap.stroke_path(&path, &paint, &stroke, Transform::identity(), None)
        }
    }
    pub fn draw_all(mut self, pixmap: &mut Pixmap) {
        self.draw(pixmap);
        self.draw_priority(pixmap);
    }
}
