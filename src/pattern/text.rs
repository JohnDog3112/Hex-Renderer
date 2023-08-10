use lazy_static::lazy_static;
use rusttype::{point, Font, Scale};
use tiny_skia::{Color, Pixmap, Transform};

use crate::pattern_utils::HexCoord;

const HEIGHT_SCALE: f32 = 22.0 / 30.0;
const WIDTH_SCALE: f32 = 0.48333326;

lazy_static! {
    static ref FONT: Font<'static> = {
        let font_file = include_bytes!("../Lato-Regular.ttf");
        Font::try_from_bytes(font_file).expect("error constructing font!")
    };
}

pub fn draw_text(pixmap: &mut Pixmap, str: &str, mut color: Color, center: HexCoord, radius: f32) {
    let rect_width = radius * 2.0_f32.sqrt();

    let scaler = rect_width / (HEIGHT_SCALE).max(WIDTH_SCALE * str.len() as f32);

    let scale = Scale::uniform(scaler);

    let width = WIDTH_SCALE * scaler * str.len() as f32;
    let height = HEIGHT_SCALE * scaler;

    let offset = point(0.0, height - 3.0 / 30.0 * scaler);

    let mut tmp_map = Pixmap::new(width as u32, height as u32).unwrap();

    let glyphs: Vec<_> = FONT.layout(str, scale, offset).collect();

    let pixels = tmp_map.pixels_mut();

    for g in glyphs {
        if let Some(bb) = g.pixel_bounding_box() {
            g.draw(|x, y, v| {
                let x = x as i32 + bb.min.x;
                let y = y as i32 + bb.min.y;

                if x >= 0 && x < width as i32 && y >= 0 && y < height as i32 {
                    let x = x as usize;
                    let y = y as usize;
                    color.set_alpha(v);
                    pixels[x + y * width as usize] = color.premultiply().to_color_u8();
                }
            })
        }
    }
    let map_offset = center - HexCoord(width, height) / 2.0;

    let paint = tiny_skia::PixmapPaint::default();

    pixmap.draw_pixmap(
        map_offset.0 as i32,
        map_offset.1 as i32,
        tmp_map.as_ref(),
        &paint,
        Transform::identity(),
        None,
    );
}
