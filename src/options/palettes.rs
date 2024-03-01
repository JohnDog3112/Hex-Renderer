//! Set of default palettes to use for patterns.
#![allow(non_snake_case)]
use lazy_static::lazy_static;

use crate::options::Color;

lazy_static! {
    pub static ref ALL: Vec<Vec<Color>> = vec![
        DEFAULT.to_vec(),
        TURBO.to_vec(),
        DARK2.to_vec(),
        TAB10.to_vec()
    ];
    pub static ref DEFAULT: Vec<Color> = vec![
        Color(214, 9, 177, 255),
        Color(108, 25, 140, 255),
        Color(50, 102, 207, 255),
        Color(102, 110, 125, 255),
    ];
    pub static ref TURBO: Vec<Color> = vec![
        Color(63, 62, 156, 255),
        Color(65, 150, 255, 255),
        Color(25, 227, 185, 255),
        Color(132, 255, 81, 255),
        Color(223, 223, 55, 255),
        Color(253, 141, 39, 255),
        Color(214, 53, 6, 255),
        Color(122, 4, 3, 255),
    ];
    pub static ref DARK2: Vec<Color> = vec![
        Color(27, 158, 119, 255),
        Color(217, 95, 2, 255),
        Color(117, 112, 179, 255),
        Color(231, 41, 138, 255),
        Color(102, 166, 30, 255),
        Color(230, 171, 2, 255),
        Color(166, 118, 29, 255),
        Color(102, 102, 102, 255),
    ];
    pub static ref TAB10: Vec<Color> = vec![
        Color(31, 119, 180, 255),
        Color(255, 127, 14, 255),
        Color(44, 160, 44, 255),
        Color(148, 103, 189, 255),
        Color(140, 86, 75, 255),
        Color(127, 127, 127, 255),
        Color(188, 189, 34, 255),
        Color(23, 190, 207, 255),
    ];
}
