use std::{time::Instant, mem::size_of_val};

mod pattern_utils;

use draw_options::{Lines, Intersections, Marker};

mod pattern;
use pattern::Pattern;


mod pattern_grid;

mod draw_options;

mod pattern_grid_options;


use pattern_grid_options::{GridOptions, GridDrawOptions};
use tiny_skia::*;

use crate::draw_options::{Triangle, EndPoint};


fn main() {

    let start = Instant::now();

    let mut paint = Paint::default();
    paint.anti_alias = true;
    paint.set_color_rgba8(128, 0, 128, 255);



    let patterns_str = "HexPattern(WEST qqq), Air, Chicken Type, Wheat Seeds, Cow Type, Wheat, Sheep Type, Wheat, HexPattern(EAST eee), HexPattern(WEST qqq), HexPattern(EAST aawdd), HexPattern(EAST aadaa), HexPattern(SOUTH_EAST aqaawa), HexPattern(EAST aada), HexPattern(SOUTH_WEST qawde), HexPattern(EAST dedqde), HexPattern(SOUTH_EAST aqaaw), HexPattern(NORTH_EAST waaw), HexPattern(EAST aadaa), HexPattern(SOUTH_EAST aqaaw), HexPattern(NORTH_EAST waaw), HexPattern(NORTH_WEST qaeaqwded), HexPattern(NORTH_WEST qwaeawq), HexPattern(SOUTH_EAST aweeeeewaaww), HexPattern(SOUTH_WEST aaqwqaa), HexPattern(SOUTH_EAST ae), HexPattern(EAST aadaa), HexPattern(NORTH_EAST aw), HexPattern(WEST qqq), HexPattern(EAST qqqwqqqqaa), HexPattern(EAST eee), HexPattern(WEST qqq), HexPattern(SOUTH_EAST ada), HexPattern(EAST eee), HexPattern(SOUTH_EAST awdd), HexPattern(NORTH_WEST qwaeawq), HexPattern(SOUTH_EAST aqaaedwd), HexPattern(WEST ddad), HexPattern(EAST aawdd), HexPattern(SOUTH_EAST deaqq), HexPattern(EAST eee), HexPattern(NORTH_EAST qaq), HexPattern(SOUTH_WEST aa), HexPattern(SOUTH_EAST aqaaeee), HexPattern(SOUTH_EAST qqqqqwdeddwa), HexPattern(NORTH_EAST dadad), HexPattern(SOUTH_EAST ada)";
    
    let patterns_str = "HexPattern(NORTH_WEST wawqwawwwewwwewwwawqwawwwewwwewdeaweewaqaweewaawwww), HexPattern(WEST qqq), HexPattern(EAST eee), HexPattern(WEST qqq), HexPattern(EAST aawdd), HexPattern(EAST aadaa), HexPattern(SOUTH_EAST aqaawa), HexPattern(EAST aada), HexPattern(SOUTH_WEST qawde), HexPattern(EAST dedqde), HexPattern(SOUTH_EAST aqaaw), HexPattern(NORTH_EAST waaw), HexPattern(EAST aadaa), HexPattern(SOUTH_EAST aqaaw), HexPattern(NORTH_EAST waaw), HexPattern(NORTH_WEST qaeaqwded), HexPattern(NORTH_WEST qwaeawq), HexPattern(SOUTH_EAST aweeeeewaaww), HexPattern(SOUTH_WEST aaqwqaa), HexPattern(SOUTH_EAST ae), HexPattern(EAST aadaa), HexPattern(NORTH_EAST aw), HexPattern(WEST qqq), HexPattern(EAST qqqwqqqqaa), HexPattern(EAST eee), HexPattern(WEST qqq), HexPattern(SOUTH_EAST ada), HexPattern(EAST eee), HexPattern(SOUTH_EAST awdd), HexPattern(NORTH_WEST qwaeawq), HexPattern(SOUTH_EAST aqaaedwd), HexPattern(WEST ddad), HexPattern(EAST aawdd), HexPattern(SOUTH_EAST deaqq), HexPattern(EAST eee), HexPattern(NORTH_EAST qaq), HexPattern(SOUTH_WEST aa), HexPattern(SOUTH_EAST aqaaeee), HexPattern(SOUTH_EAST qqqqqwdeddwa), HexPattern(NORTH_EAST dadad), HexPattern(SOUTH_EAST ada)";

    let patterns_str = "HexPattern(WEST qqqaw), HexPattern(WEST eaqa), HexPattern(EAST aadaa), HexPattern(EAST aadaa), HexPattern(SOUTH_EAST aweeeeewaaww), HexPattern(EAST aawdd), HexPattern(SOUTH_EAST ddewedd), HexPattern(NORTH_EAST aweaqa), HexPattern(SOUTH_WEST aaqwqaa), HexPattern(SOUTH_EAST a), HexPattern(SOUTH_WEST aaqwqaa), HexPattern(SOUTH_EAST ae), HexPattern(EAST aadaa), HexPattern(NORTH_EAST aw), HexPattern(WEST qqq), HexPattern(NORTH_WEST aqaeqded), HexPattern(EAST eee), HexPattern(WEST qqq), HexPattern(SOUTH_EAST a), HexPattern(EAST eee), HexPattern(SOUTH_EAST awdd), HexPattern(SOUTH_EAST deaqq), HexPattern(SOUTH_EAST a)";

    let patterns_str = "HexPattern(EAST waqa), HexPattern(EAST aadaa), HexPattern(SOUTH_EAST awdwaaww), HexPattern(EAST waqwwaqa), HexPattern(NORTH_EAST deddw), HexPattern(EAST ad), HexPattern(SOUTH_EAST awdwaaww), HexPattern(SOUTH_EAST awdwa), HexPattern(SOUTH_EAST awdd), HexPattern(NORTH_EAST waawaqwawqq), HexPattern(EAST aadaa), HexPattern(SOUTH_EAST awdwaaww), HexPattern(EAST waqwwaqa), HexPattern(EAST aadaa), HexPattern(SOUTH_EAST aqaawa), HexPattern(EAST aada), HexPattern(EAST aawdd), HexPattern(SOUTH_EAST aqaa), HexPattern(EAST aawdd), HexPattern(EAST aqwwaqwaad), HexPattern(EAST aawdd), HexPattern(SOUTH_EAST aqaaw), HexPattern(NORTH_EAST waaw), HexPattern(SOUTH_EAST aqaawww), HexPattern(WEST ddad), HexPattern(EAST aadaa), HexPattern(EAST waqaeaq), HexPattern(SOUTH_EAST aqaawww), HexPattern(WEST ddad), HexPattern(EAST aawdd), HexPattern(EAST aqwwaqwaad), HexPattern(EAST aawdd), HexPattern(EAST aadaa), HexPattern(EAST aqwaq), HexPattern(NORTH_EAST aw), HexPattern(SOUTH_EAST aqaaw), HexPattern(EAST aada), HexPattern(EAST aqwaq), HexPattern(SOUTH_EAST aqaaedwd), HexPattern(EAST aada), HexPattern(SOUTH_EAST aqaawa), HexPattern(SOUTH_WEST ewdqdwe), HexPattern(SOUTH_EAST aqaaw), HexPattern(SOUTH_EAST aqaawaa), HexPattern(WEST ddad), HexPattern(SOUTH_EAST aqaaq), HexPattern(WEST ddad), HexPattern(EAST aadaa), HexPattern(SOUTH_EAST awdwa), HexPattern(EAST ad), HexPattern(EAST aawdd), HexPattern(SOUTH_EAST awdwaaww), HexPattern(EAST aawdd), HexPattern(NORTH_EAST waawaqwawqq), HexPattern(SOUTH_EAST awdwa), HexPattern(EAST aawdd), HexPattern(SOUTH_EAST awdd), HexPattern(NORTH_EAST waawaqwawqq), HexPattern(SOUTH_EAST aqaawa), HexPattern(SOUTH_WEST ewdqdwe), HexPattern(SOUTH_EAST awdd), HexPattern(NORTH_WEST qwaeawq), HexPattern(NORTH_EAST qqaeaae), HexPattern(NORTH_EAST dwqqqqqwddww), HexPattern(EAST aadaa), HexPattern(EAST aqaeaq), HexPattern(WEST qqq), HexPattern(SOUTH_WEST aaqwqaa), HexPattern(SOUTH_EAST aqaaedwd), HexPattern(WEST ddad), HexPattern(EAST aawdd), HexPattern(EAST aadaa), HexPattern(SOUTH_EAST aweeeeewaaww), HexPattern(EAST aawdd), HexPattern(EAST wawqwawaw), HexPattern(NORTH_EAST dedq), HexPattern(WEST dwwdwwdwdd), HexPattern(WEST qqq), \"\\\", HexPattern(EAST eee), HexPattern(NORTH_WEST qwaeawq), HexPattern(EAST aawdd), HexPattern(NORTH_EAST waawaqwawqq), HexPattern(WEST qqq), \"/\", HexPattern(EAST eee), HexPattern(NORTH_WEST qwaeawq), HexPattern(NORTH_EAST waawaqwawqq), HexPattern(SOUTH_EAST aqaawaa), HexPattern(EAST aada), HexPattern(EAST waqwwaqa), HexPattern(NORTH_EAST deddw), HexPattern(EAST ad), HexPattern(WEST qqq), HexPattern(SOUTH_EAST a), HexPattern(EAST eee), HexPattern(NORTH_WEST qwaeawq), HexPattern(WEST qqq), HexPattern(SOUTH_WEST edqde), HexPattern(EAST eee), HexPattern(NORTH_WEST qwaeawq), HexPattern(SOUTH_EAST awdd), HexPattern(SOUTH_EAST deaqq), HexPattern(EAST aawdd), HexPattern(EAST eee), HexPattern(EAST aawdd), HexPattern(NORTH_EAST qeewdweddw), HexPattern(SOUTH_EAST aqaaedwd), HexPattern(WEST ddad), HexPattern(SOUTH_EAST aqaaedwd), HexPattern(WEST ddad), HexPattern(EAST aadaa), HexPattern(SOUTH_EAST aqaaw), HexPattern(NORTH_WEST wddw), HexPattern(NORTH_WEST eqqwawqaaw), HexPattern(EAST aadaadaa), HexPattern(WEST qqq), HexPattern(NORTH_WEST qaeaq), HexPattern(EAST eee), HexPattern(NORTH_WEST qwaeawq), HexPattern(NORTH_EAST qeewdweddw), HexPattern(EAST aadaadaa), HexPattern(NORTH_EAST qeewdweddw), HexPattern(SOUTH_WEST ewdqdwe), HexPattern(SOUTH_EAST deaqq), HexPattern(EAST aawdd), HexPattern(NORTH_WEST eqqwawqaaw), HexPattern(SOUTH_EAST deaqq), HexPattern(SOUTH_EAST aeea), HexPattern(EAST aadaa), HexPattern(EAST aqaeaq), HexPattern(EAST aadaa), HexPattern(SOUTH_EAST aqaaw), HexPattern(EAST ad), HexPattern(WEST qqq), HexPattern(NORTH_WEST qwaeawq), HexPattern(SOUTH_WEST aaqwqaa), HexPattern(SOUTH_EAST ae), HexPattern(NORTH_EAST dedq), HexPattern(EAST eee), HexPattern(WEST qqq), HexPattern(NORTH_EAST de), HexPattern(WEST qqq), \"Too Many Matches Found!\", HexPattern(EAST eee), HexPattern(NORTH_WEST qwaeawq), HexPattern(NORTH_EAST de), HexPattern(SOUTH_EAST adada), HexPattern(SOUTH_EAST aqae), HexPattern(EAST eee), HexPattern(SOUTH_EAST awdd), HexPattern(EAST aawdd), HexPattern(SOUTH_EAST aqaa), HexPattern(EAST ad), HexPattern(EAST aawdd), HexPattern(WEST qqq), HexPattern(WEST qqq), \"No Matches Found\", HexPattern(EAST eee), HexPattern(NORTH_WEST qwaeawq), HexPattern(NORTH_EAST de), HexPattern(SOUTH_EAST adada), HexPattern(SOUTH_EAST aqae), HexPattern(EAST eee), HexPattern(EAST aawdd), HexPattern(SOUTH_EAST awdd), HexPattern(SOUTH_EAST deaqq), HexPattern(WEST qqq), HexPattern(SOUTH_WEST aqdee), HexPattern(EAST eee), HexPattern(NORTH_WEST qwaeawq), HexPattern(WEST qqq), HexPattern(EAST), HexPattern(EAST eee), HexPattern(NORTH_WEST qwaeawq), HexPattern(SOUTH_EAST awdd), HexPattern(SOUTH_EAST deaqq), HexPattern(EAST aawdd), HexPattern(EAST eaqaaw), HexPattern(NORTH_EAST qaq), HexPattern(SOUTH_WEST aa), HexPattern(EAST qded), HexPattern(SOUTH_EAST a)";
    //let patterns_str = "NORTH_WEST wawqwawwwewwwewwwawqwawwwewwwewdeaweewaqaweewaawwww";

    //let patterns_str = "NORTH_WEST wawqwawwwewwwewwwawqwawwwewwwewdeaweeadedaeewaawwww";
    //let patterns_str = "SOUTH_EAST wqwwwqwwwdwewdwqqdaeeeeeaddwweaqaawewawqwawwwewwwew";


    let patterns: Vec<Pattern> = patterns_str.split(", ").filter_map(|str| Pattern::try_from(str).map_or(None, |pattern| Some(pattern))).collect();
    


    let grid = pattern_grid::PatternGrid::generate_grid(patterns.clone(), 30);
    
    //let intersections = gen(Color::BLACK);
    let triangle = Triangle::BorderStartMatch(0.16, Color::WHITE, 0.24);

    /*let options = GridOptions {
        line_thickness: 0.12,
        scale: 50.0,
        draw_options: GridDrawOptions::Changing(vec![
            (gen(Color::WHITE), Lines::SegmentColors(vec![
                Color::from_rgba8(214, 9, 177, 255),
                Color::from_rgba8(108, 25, 140, 255),
                Color::from_rgba8(50, 102, 207, 255),
                Color::from_rgba8(102, 110, 125, 255),
            ], triangle.clone())),
            (gen(Color::from_rgba8(252, 8, 20, 255)), Lines::SegmentColors(vec![
                Color::from_rgba8(63, 62, 156, 255),
                Color::from_rgba8(65, 150, 255, 255),
                Color::from_rgba8(25, 227, 185, 255),
                Color::from_rgba8(132, 255, 81, 255),
                Color::from_rgba8(223, 223, 55, 255),
                Color::from_rgba8(253, 141, 39, 255),
                Color::from_rgba8(214, 53, 6, 255),
                Color::from_rgba8(122, 4, 3, 255),
            ], triangle.clone())),
            (gen(Color::from_rgba8(252, 114, 8, 255)), Lines::SegmentColors(vec![
                Color::from_rgba8(27, 158, 119, 255),
                Color::from_rgba8(217, 95, 2, 255),
                Color::from_rgba8(117, 112, 179, 255),
                Color::from_rgba8(231, 41, 138, 255),
                Color::from_rgba8(102, 166, 30, 255),
                Color::from_rgba8(230, 171, 2, 255),
                Color::from_rgba8(166, 118, 29, 255),
                Color::from_rgba8(102, 102, 102, 255),
            ], triangle.clone())),
            (gen(Color::from_rgba8(250, 0, 225, 255)), Lines::SegmentColors(vec![
                Color::from_rgba8(31, 119, 180, 255),
                Color::from_rgba8(255, 127, 14, 255),
                Color::from_rgba8(44, 160, 44, 255),
                Color::from_rgba8(148, 103, 189, 255),
                Color::from_rgba8(140, 86, 75, 255),
                Color::from_rgba8(127, 127, 127, 255),
                Color::from_rgba8(188, 189, 34, 255),
                Color::from_rgba8(23, 190, 207, 255),
            ], triangle.clone())),
        ]),
    };*/

    let options = GridOptions {
        line_thickness: 0.12,
        scale: 50.0,
        draw_options: GridDrawOptions::Changing(vec![
            (gen_dots(Color::WHITE), Lines::Gradient(vec![
                Color::from_rgba8(214, 9, 177, 255),
                Color::from_rgba8(108, 25, 140, 255),
                Color::from_rgba8(50, 102, 207, 255),
                Color::from_rgba8(102, 110, 125, 255),
            ], 15, true)),
            (gen_dots(Color::WHITE), Lines::Gradient(vec![
                Color::from_rgba8(63, 62, 156, 255),
                Color::from_rgba8(65, 150, 255, 255),
                Color::from_rgba8(25, 227, 185, 255),
                Color::from_rgba8(132, 255, 81, 255),
                Color::from_rgba8(223, 223, 55, 255),
                Color::from_rgba8(253, 141, 39, 255),
                Color::from_rgba8(214, 53, 6, 255),
                Color::from_rgba8(122, 4, 3, 255),
            ], 15, true)),
            (gen_dots(Color::WHITE), Lines::Gradient(vec![
                Color::from_rgba8(27, 158, 119, 255),
                Color::from_rgba8(217, 95, 2, 255),
                Color::from_rgba8(117, 112, 179, 255),
                Color::from_rgba8(231, 41, 138, 255),
                Color::from_rgba8(102, 166, 30, 255),
                Color::from_rgba8(230, 171, 2, 255),
                Color::from_rgba8(166, 118, 29, 255),
                Color::from_rgba8(102, 102, 102, 255),
            ], 15, true)),
            (gen_dots(Color::WHITE), Lines::Gradient(vec![
                Color::from_rgba8(31, 119, 180, 255),
                Color::from_rgba8(255, 127, 14, 255),
                Color::from_rgba8(44, 160, 44, 255),
                Color::from_rgba8(148, 103, 189, 255),
                Color::from_rgba8(140, 86, 75, 255),
                Color::from_rgba8(127, 127, 127, 255),
                Color::from_rgba8(188, 189, 34, 255),
                Color::from_rgba8(23, 190, 207, 255),
            ], 15, true)),
        ]),
    };

    /*let options = GridOptions {
        line_thickness: 0.12,
        scale: 50.0,
        draw_options: GridDrawOptions::Changing(vec![
            (gen(Color::WHITE), Lines::Monocolor(Color::from_rgba8(214, 9, 177, 255))),
            (gen(Color::WHITE), Lines::Monocolor(Color::from_rgba8(63, 62, 156, 255))),
            (gen(Color::WHITE), Lines::Monocolor(Color::from_rgba8(27, 158, 119, 255))),
            (gen(Color::WHITE), Lines::Monocolor(Color::from_rgba8(31, 119, 180, 255))),
        ]),
    };*/
    grid.draw_grid_to_file("image.png", options);
    
}
fn gen(col: Color) -> Intersections {
    Intersections::EndsAndMiddle(
        EndPoint::Marker(Marker::SinglePoint(Color::WHITE, 0.1)), 
        EndPoint::Marker(Marker::SinglePoint(Color::WHITE, 0.1)), 
        Marker::SinglePoint(Color::WHITE, 0.1)
    )
}

fn gen_dots(col: Color) -> Intersections {
    Intersections::EndsAndMiddle(
        EndPoint::BorderedMatch(0.05, col, 0.07),
        EndPoint::Marker(Marker::SinglePoint(col, 0.07)),
        Marker::SinglePoint(col, 0.07))
}