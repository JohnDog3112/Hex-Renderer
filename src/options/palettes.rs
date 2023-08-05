#![allow(non_snake_case)]
use lazy_static::lazy_static;
use tiny_skia::Color;

lazy_static! {
    pub static ref ALL: Vec<Vec<Color>> = vec![
        DEFAULT.to_vec(),
        TURBO.to_vec(),
        DARK2.to_vec(),
        TAB10.to_vec()
    ];
    pub static ref DEFAULT: Vec<Color> = vec![
        Color::from_rgba8(214, 9, 177, 255),
        Color::from_rgba8(108, 25, 140, 255),
        Color::from_rgba8(50, 102, 207, 255),
        Color::from_rgba8(102, 110, 125, 255),
    ];
    pub static ref TURBO: Vec<Color> = vec![
        Color::from_rgba8(63, 62, 156, 255),
        Color::from_rgba8(65, 150, 255, 255),
        Color::from_rgba8(25, 227, 185, 255),
        Color::from_rgba8(132, 255, 81, 255),
        Color::from_rgba8(223, 223, 55, 255),
        Color::from_rgba8(253, 141, 39, 255),
        Color::from_rgba8(214, 53, 6, 255),
        Color::from_rgba8(122, 4, 3, 255),
    ];
    pub static ref DARK2: Vec<Color> = vec![
        Color::from_rgba8(27, 158, 119, 255),
        Color::from_rgba8(217, 95, 2, 255),
        Color::from_rgba8(117, 112, 179, 255),
        Color::from_rgba8(231, 41, 138, 255),
        Color::from_rgba8(102, 166, 30, 255),
        Color::from_rgba8(230, 171, 2, 255),
        Color::from_rgba8(166, 118, 29, 255),
        Color::from_rgba8(102, 102, 102, 255),
    ];
    pub static ref TAB10: Vec<Color> = vec![
        Color::from_rgba8(31, 119, 180, 255),
        Color::from_rgba8(255, 127, 14, 255),
        Color::from_rgba8(44, 160, 44, 255),
        Color::from_rgba8(148, 103, 189, 255),
        Color::from_rgba8(140, 86, 75, 255),
        Color::from_rgba8(127, 127, 127, 255),
        Color::from_rgba8(188, 189, 34, 255),
        Color::from_rgba8(23, 190, 207, 255),
    ];
}
