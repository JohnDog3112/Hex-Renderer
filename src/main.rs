use std::time::Instant;

mod angle;
use angle::Angle;

mod direction;
use direction::Direction;

mod coord;
use coord::Coord;

mod hex_coord;
use draw_options::{Lines, Intersections, Marker, GradientOptions};
use hex_coord::HexCoord;

mod pattern;
use pattern::Pattern;

mod dynamic_list;

mod pattern_grid;

mod draw_options;

mod pattern_grid_options;


use pattern_grid_options::{GridOptions, GridDrawOptions};
use tiny_skia::*;

use crate::draw_options::{Triangle, EndPoint};

const SCALE: f32 = 50.0;
const LINE_THICKNESS: f32 = 0.12;

fn main() {

    let start = Instant::now();

    let mut paint = Paint::default();
    paint.anti_alias = true;
    paint.set_color_rgba8(128, 0, 128, 255);



    let patterns_str = "HexPattern(WEST qqq), Air, Chicken Type, Wheat Seeds, Cow Type, Wheat, Sheep Type, Wheat, HexPattern(EAST eee), HexPattern(WEST qqq), HexPattern(EAST aawdd), HexPattern(EAST aadaa), HexPattern(SOUTH_EAST aqaawa), HexPattern(EAST aada), HexPattern(SOUTH_WEST qawde), HexPattern(EAST dedqde), HexPattern(SOUTH_EAST aqaaw), HexPattern(NORTH_EAST waaw), HexPattern(EAST aadaa), HexPattern(SOUTH_EAST aqaaw), HexPattern(NORTH_EAST waaw), HexPattern(NORTH_WEST qaeaqwded), HexPattern(NORTH_WEST qwaeawq), HexPattern(SOUTH_EAST aweeeeewaaww), HexPattern(SOUTH_WEST aaqwqaa), HexPattern(SOUTH_EAST ae), HexPattern(EAST aadaa), HexPattern(NORTH_EAST aw), HexPattern(WEST qqq), HexPattern(EAST qqqwqqqqaa), HexPattern(EAST eee), HexPattern(WEST qqq), HexPattern(SOUTH_EAST ada), HexPattern(EAST eee), HexPattern(SOUTH_EAST awdd), HexPattern(NORTH_WEST qwaeawq), HexPattern(SOUTH_EAST aqaaedwd), HexPattern(WEST ddad), HexPattern(EAST aawdd), HexPattern(SOUTH_EAST deaqq), HexPattern(EAST eee), HexPattern(NORTH_EAST qaq), HexPattern(SOUTH_WEST aa), HexPattern(SOUTH_EAST aqaaeee), HexPattern(SOUTH_EAST qqqqqwdeddwa), HexPattern(NORTH_EAST dadad), HexPattern(SOUTH_EAST ada)";
    
    let patterns_str = "HexPattern(NORTH_WEST wawqwawwwewwwewwwawqwawwwewwwewdeaweewaqaweewaawwww), HexPattern(WEST qqq), HexPattern(EAST eee), HexPattern(WEST qqq), HexPattern(EAST aawdd), HexPattern(EAST aadaa), HexPattern(SOUTH_EAST aqaawa), HexPattern(EAST aada), HexPattern(SOUTH_WEST qawde), HexPattern(EAST dedqde), HexPattern(SOUTH_EAST aqaaw), HexPattern(NORTH_EAST waaw), HexPattern(EAST aadaa), HexPattern(SOUTH_EAST aqaaw), HexPattern(NORTH_EAST waaw), HexPattern(NORTH_WEST qaeaqwded), HexPattern(NORTH_WEST qwaeawq), HexPattern(SOUTH_EAST aweeeeewaaww), HexPattern(SOUTH_WEST aaqwqaa), HexPattern(SOUTH_EAST ae), HexPattern(EAST aadaa), HexPattern(NORTH_EAST aw), HexPattern(WEST qqq), HexPattern(EAST qqqwqqqqaa), HexPattern(EAST eee), HexPattern(WEST qqq), HexPattern(SOUTH_EAST ada), HexPattern(EAST eee), HexPattern(SOUTH_EAST awdd), HexPattern(NORTH_WEST qwaeawq), HexPattern(SOUTH_EAST aqaaedwd), HexPattern(WEST ddad), HexPattern(EAST aawdd), HexPattern(SOUTH_EAST deaqq), HexPattern(EAST eee), HexPattern(NORTH_EAST qaq), HexPattern(SOUTH_WEST aa), HexPattern(SOUTH_EAST aqaaeee), HexPattern(SOUTH_EAST qqqqqwdeddwa), HexPattern(NORTH_EAST dadad), HexPattern(SOUTH_EAST ada)";

    let patterns_str = "HexPattern(WEST qqqaw), HexPattern(WEST eaqa), HexPattern(EAST aadaa), HexPattern(EAST aadaa), HexPattern(SOUTH_EAST aweeeeewaaww), HexPattern(EAST aawdd), HexPattern(SOUTH_EAST ddewedd), HexPattern(NORTH_EAST aweaqa), HexPattern(SOUTH_WEST aaqwqaa), HexPattern(SOUTH_EAST a), HexPattern(SOUTH_WEST aaqwqaa), HexPattern(SOUTH_EAST ae), HexPattern(EAST aadaa), HexPattern(NORTH_EAST aw), HexPattern(WEST qqq), HexPattern(NORTH_WEST aqaeqded), HexPattern(EAST eee), HexPattern(WEST qqq), HexPattern(SOUTH_EAST a), HexPattern(EAST eee), HexPattern(SOUTH_EAST awdd), HexPattern(SOUTH_EAST deaqq), HexPattern(SOUTH_EAST a)";

    //let patterns_str = "NORTH_WEST wawqwawwwewwwewwwawqwawwwewwwewdeaweewaqaweewaawwww";

    //let patterns_str = "NORTH_WEST wawqwawwwewwwewwwawqwawwwewwwewdeaweeadedaeewaawwww";
    //let patterns_str = "SOUTH_EAST wqwwwqwwwdwewdwqqdaeeeeeaddwweaqaawewawqwawwwewwwew";

    let patterns: Vec<Pattern> = patterns_str.split(", ").filter_map(|str| Pattern::try_from(str).map_or(None, |pattern| Some(pattern))).collect();
    


    let grid = pattern_grid::PatternGrid::generate(patterns.clone(), 20);
    
    
    grid.draw("image.png", GridOptions::segments());

}
