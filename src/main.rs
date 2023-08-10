mod pattern_utils;

use options::{defaults, palettes, GridOptions, GridPatternOptions, Lines};

mod pattern;
use pattern::Pattern;

mod hex_grid;

mod options;

fn main() {
    //let patterns_str = "HexPattern(WEST qqq), Air, Chicken Type, Wheat Seeds, Cow Type, Wheat, Sheep Type, Wheat, HexPattern(EAST eee), HexPattern(WEST qqq), HexPattern(EAST aawdd), HexPattern(EAST aadaa), HexPattern(SOUTH_EAST aqaawa), HexPattern(EAST aada), HexPattern(SOUTH_WEST qawde), HexPattern(EAST dedqde), HexPattern(SOUTH_EAST aqaaw), HexPattern(NORTH_EAST waaw), HexPattern(EAST aadaa), HexPattern(SOUTH_EAST aqaaw), HexPattern(NORTH_EAST waaw), HexPattern(NORTH_WEST qaeaqwded), HexPattern(NORTH_WEST qwaeawq), HexPattern(SOUTH_EAST aweeeeewaaww), HexPattern(SOUTH_WEST aaqwqaa), HexPattern(SOUTH_EAST ae), HexPattern(EAST aadaa), HexPattern(NORTH_EAST aw), HexPattern(WEST qqq), HexPattern(EAST qqqwqqqqaa), HexPattern(EAST eee), HexPattern(WEST qqq), HexPattern(SOUTH_EAST ada), HexPattern(EAST eee), HexPattern(SOUTH_EAST awdd), HexPattern(NORTH_WEST qwaeawq), HexPattern(SOUTH_EAST aqaaedwd), HexPattern(WEST ddad), HexPattern(EAST aawdd), HexPattern(SOUTH_EAST deaqq), HexPattern(EAST eee), HexPattern(NORTH_EAST qaq), HexPattern(SOUTH_WEST aa), HexPattern(SOUTH_EAST aqaaeee), HexPattern(SOUTH_EAST qqqqqwdeddwa), HexPattern(NORTH_EAST dadad), HexPattern(SOUTH_EAST ada)";

    //let patterns_str = "HexPattern(NORTH_WEST wawqwawwwewwwewwwawqwawwwewwwewdeaweewaqaweewaawwww), HexPattern(WEST qqq), HexPattern(EAST eee), HexPattern(WEST qqq), HexPattern(EAST aawdd), HexPattern(EAST aadaa), HexPattern(SOUTH_EAST aqaawa), HexPattern(EAST aada), HexPattern(SOUTH_WEST qawde), HexPattern(EAST dedqde), HexPattern(SOUTH_EAST aqaaw), HexPattern(NORTH_EAST waaw), HexPattern(EAST aadaa), HexPattern(SOUTH_EAST aqaaw), HexPattern(NORTH_EAST waaw), HexPattern(NORTH_WEST qaeaqwded), HexPattern(NORTH_WEST qwaeawq), HexPattern(SOUTH_EAST aweeeeewaaww), HexPattern(SOUTH_WEST aaqwqaa), HexPattern(SOUTH_EAST ae), HexPattern(EAST aadaa), HexPattern(NORTH_EAST aw), HexPattern(WEST qqq), HexPattern(EAST qqqwqqqqaa), HexPattern(EAST eee), HexPattern(WEST qqq), HexPattern(SOUTH_EAST ada), HexPattern(EAST eee), HexPattern(SOUTH_EAST awdd), HexPattern(NORTH_WEST qwaeawq), HexPattern(SOUTH_EAST aqaaedwd), HexPattern(WEST ddad), HexPattern(EAST aawdd), HexPattern(SOUTH_EAST deaqq), HexPattern(EAST eee), HexPattern(NORTH_EAST qaq), HexPattern(SOUTH_WEST aa), HexPattern(SOUTH_EAST aqaaeee), HexPattern(SOUTH_EAST qqqqqwdeddwa), HexPattern(NORTH_EAST dadad), HexPattern(SOUTH_EAST ada)";

    //let patterns_str = "HexPattern(WEST qqqaw), HexPattern(WEST eaqa), HexPattern(EAST aadaa), HexPattern(EAST aadaa), HexPattern(SOUTH_EAST aweeeeewaaww), HexPattern(EAST aawdd), HexPattern(SOUTH_EAST ddewedd), HexPattern(NORTH_EAST aweaqa), HexPattern(SOUTH_WEST aaqwqaa), HexPattern(SOUTH_EAST a), HexPattern(SOUTH_WEST aaqwqaa), HexPattern(SOUTH_EAST ae), HexPattern(EAST aadaa), HexPattern(NORTH_EAST aw), HexPattern(WEST qqq), HexPattern(NORTH_WEST aqaeqded), HexPattern(EAST eee), HexPattern(WEST qqq), HexPattern(SOUTH_EAST a), HexPattern(EAST eee), HexPattern(SOUTH_EAST awdd), HexPattern(SOUTH_EAST deaqq), HexPattern(SOUTH_EAST a)";

    //let patterns_str = "HexPattern(EAST waqa), HexPattern(EAST aadaa), HexPattern(SOUTH_EAST awdwaaww), HexPattern(EAST waqwwaqa), HexPattern(NORTH_EAST deddw), HexPattern(EAST ad), HexPattern(SOUTH_EAST awdwaaww), HexPattern(SOUTH_EAST awdwa), HexPattern(SOUTH_EAST awdd), HexPattern(NORTH_EAST waawaqwawqq), HexPattern(EAST aadaa), HexPattern(SOUTH_EAST awdwaaww), HexPattern(EAST waqwwaqa), HexPattern(EAST aadaa), HexPattern(SOUTH_EAST aqaawa), HexPattern(EAST aada), HexPattern(EAST aawdd), HexPattern(SOUTH_EAST aqaa), HexPattern(EAST aawdd), HexPattern(EAST aqwwaqwaad), HexPattern(EAST aawdd), HexPattern(SOUTH_EAST aqaaw), HexPattern(NORTH_EAST waaw), HexPattern(SOUTH_EAST aqaawww), HexPattern(WEST ddad), HexPattern(EAST aadaa), HexPattern(EAST waqaeaq), HexPattern(SOUTH_EAST aqaawww), HexPattern(WEST ddad), HexPattern(EAST aawdd), HexPattern(EAST aqwwaqwaad), HexPattern(EAST aawdd), HexPattern(EAST aadaa), HexPattern(EAST aqwaq), HexPattern(NORTH_EAST aw), HexPattern(SOUTH_EAST aqaaw), HexPattern(EAST aada), HexPattern(EAST aqwaq), HexPattern(SOUTH_EAST aqaaedwd), HexPattern(EAST aada), HexPattern(SOUTH_EAST aqaawa), HexPattern(SOUTH_WEST ewdqdwe), HexPattern(SOUTH_EAST aqaaw), HexPattern(SOUTH_EAST aqaawaa), HexPattern(WEST ddad), HexPattern(SOUTH_EAST aqaaq), HexPattern(WEST ddad), HexPattern(EAST aadaa), HexPattern(SOUTH_EAST awdwa), HexPattern(EAST ad), HexPattern(EAST aawdd), HexPattern(SOUTH_EAST awdwaaww), HexPattern(EAST aawdd), HexPattern(NORTH_EAST waawaqwawqq), HexPattern(SOUTH_EAST awdwa), HexPattern(EAST aawdd), HexPattern(SOUTH_EAST awdd), HexPattern(NORTH_EAST waawaqwawqq), HexPattern(SOUTH_EAST aqaawa), HexPattern(SOUTH_WEST ewdqdwe), HexPattern(SOUTH_EAST awdd), HexPattern(NORTH_WEST qwaeawq), HexPattern(NORTH_EAST qqaeaae), HexPattern(NORTH_EAST dwqqqqqwddww), HexPattern(EAST aadaa), HexPattern(EAST aqaeaq), HexPattern(WEST qqq), HexPattern(SOUTH_WEST aaqwqaa), HexPattern(SOUTH_EAST aqaaedwd), HexPattern(WEST ddad), HexPattern(EAST aawdd), HexPattern(EAST aadaa), HexPattern(SOUTH_EAST aweeeeewaaww), HexPattern(EAST aawdd), HexPattern(EAST wawqwawaw), HexPattern(NORTH_EAST dedq), HexPattern(WEST dwwdwwdwdd), HexPattern(WEST qqq), \"\\\", HexPattern(EAST eee), HexPattern(NORTH_WEST qwaeawq), HexPattern(EAST aawdd), HexPattern(NORTH_EAST waawaqwawqq), HexPattern(WEST qqq), \"/\", HexPattern(EAST eee), HexPattern(NORTH_WEST qwaeawq), HexPattern(NORTH_EAST waawaqwawqq), HexPattern(SOUTH_EAST aqaawaa), HexPattern(EAST aada), HexPattern(EAST waqwwaqa), HexPattern(NORTH_EAST deddw), HexPattern(EAST ad), HexPattern(WEST qqq), HexPattern(SOUTH_EAST a), HexPattern(EAST eee), HexPattern(NORTH_WEST qwaeawq), HexPattern(WEST qqq), HexPattern(SOUTH_WEST edqde), HexPattern(EAST eee), HexPattern(NORTH_WEST qwaeawq), HexPattern(SOUTH_EAST awdd), HexPattern(SOUTH_EAST deaqq), HexPattern(EAST aawdd), HexPattern(EAST eee), HexPattern(EAST aawdd), HexPattern(NORTH_EAST qeewdweddw), HexPattern(SOUTH_EAST aqaaedwd), HexPattern(WEST ddad), HexPattern(SOUTH_EAST aqaaedwd), HexPattern(WEST ddad), HexPattern(EAST aadaa), HexPattern(SOUTH_EAST aqaaw), HexPattern(NORTH_WEST wddw), HexPattern(NORTH_WEST eqqwawqaaw), HexPattern(EAST aadaadaa), HexPattern(WEST qqq), HexPattern(NORTH_WEST qaeaq), HexPattern(EAST eee), HexPattern(NORTH_WEST qwaeawq), HexPattern(NORTH_EAST qeewdweddw), HexPattern(EAST aadaadaa), HexPattern(NORTH_EAST qeewdweddw), HexPattern(SOUTH_WEST ewdqdwe), HexPattern(SOUTH_EAST deaqq), HexPattern(EAST aawdd), HexPattern(NORTH_WEST eqqwawqaaw), HexPattern(SOUTH_EAST deaqq), HexPattern(SOUTH_EAST aeea), HexPattern(EAST aadaa), HexPattern(EAST aqaeaq), HexPattern(EAST aadaa), HexPattern(SOUTH_EAST aqaaw), HexPattern(EAST ad), HexPattern(WEST qqq), HexPattern(NORTH_WEST qwaeawq), HexPattern(SOUTH_WEST aaqwqaa), HexPattern(SOUTH_EAST ae), HexPattern(NORTH_EAST dedq), HexPattern(EAST eee), HexPattern(WEST qqq), HexPattern(NORTH_EAST de), HexPattern(WEST qqq), \"Too Many Matches Found!\", HexPattern(EAST eee), HexPattern(NORTH_WEST qwaeawq), HexPattern(NORTH_EAST de), HexPattern(SOUTH_EAST adada), HexPattern(SOUTH_EAST aqae), HexPattern(EAST eee), HexPattern(SOUTH_EAST awdd), HexPattern(EAST aawdd), HexPattern(SOUTH_EAST aqaa), HexPattern(EAST ad), HexPattern(EAST aawdd), HexPattern(WEST qqq), HexPattern(WEST qqq), \"No Matches Found\", HexPattern(EAST eee), HexPattern(NORTH_WEST qwaeawq), HexPattern(NORTH_EAST de), HexPattern(SOUTH_EAST adada), HexPattern(SOUTH_EAST aqae), HexPattern(EAST eee), HexPattern(EAST aawdd), HexPattern(SOUTH_EAST awdd), HexPattern(SOUTH_EAST deaqq), HexPattern(WEST qqq), HexPattern(SOUTH_WEST aqdee), HexPattern(EAST eee), HexPattern(NORTH_WEST qwaeawq), HexPattern(WEST qqq), HexPattern(EAST), HexPattern(EAST eee), HexPattern(NORTH_WEST qwaeawq), HexPattern(SOUTH_EAST awdd), HexPattern(SOUTH_EAST deaqq), HexPattern(EAST aawdd), HexPattern(EAST eaqaaw), HexPattern(NORTH_EAST qaq), HexPattern(SOUTH_WEST aa), HexPattern(EAST qded), HexPattern(SOUTH_EAST a)";
    //let patterns_str = "NORTH_WEST wawqwawwwewwwewwwawqwawwwewwwewdeaweewaqaweewaawwww";

    //let patterns_str = "NORTH_WEST wawqwawwwewwwewwwawqwawwwewwwewdeaweeadedaeewaawwww";
    //let patterns_str = "SOUTH_EAST wqwwwqwwwdwewdwqqdaeeeeeaddwweaqaawewawqwawwwewwwew";

    //let patterns_str = "NORTH_EAST qaqqqqwwawwqqeaddwwddas";

    let patterns_str = "NORTH_EAST qeqwqwqwqwqeqssssssaeqeaqeqaeqaqdededwaqdedsssssdssess";
    //let patterns_str = "NORTH_EAST qeqwqwqwqwqeqaeqeaqeqaeqaqded";

    //let patterns_str = "EAST wwwsdsdsdsdsds";

    //let patterns_str = patterns_str.to_string() + ", " + patterns_str1;
    //let patterns_str = "EAST wsssss";

    //let patterns_str = "ne ".to_string() + &(0..2000).map(|_| 'w').collect::<String>();

    //let patterns_str = "EAST wewewewewew";

    let patterns: Vec<Pattern> = patterns_str
        .split(", ")
        .filter_map(|str| Pattern::try_from(str).map_or(None, |pattern| Some(pattern)))
        .collect();

    let grid = hex_grid::HexGrid::generate_grid(patterns, 30);

    let scale = 100.0;

    let options = GridOptions {
        line_thickness: defaults::constants::LINE_THICKNESS,
        pattern_options: GridPatternOptions::Uniform(
            defaults::components::SEGMENT_INTERSECTION.clone(),
            Lines::SegmentColors {
                colors: palettes::TAB10.to_vec(),
                triangles: defaults::components::TRIANGLE.clone(),
                collisions: defaults::components::COLLISIONS.clone(),
            },
        ),
    };
    /*let options = GridOptions {
        line_thickness: defaults::constants::LINE_THICKNESS,
        pattern_options: GridPatternOptions::Uniform(
            defaults::components::SEGMENT_INTERSECTION.clone(),
            Lines::SegmentColors(
                vec![
                    Color::from_rgba8(255, 0, 0, 255),
                    Color::from_rgba8(255, 165, 0, 255),
                    Color::from_rgba8(255, 255, 0, 255),
                    Color::from_rgba8(0, 255, 0, 255),
                    Color::from_rgba8(0, 0, 255, 255),
                    Color::from_rgba8(75, 0, 130, 255),
                    Color::from_rgba8(127, 0, 255, 255),
                ],
                defaults::components::TRIANGLE.clone(),
            ),
        ),
    };*/

    /*let transparent_dot = Point::Single(Marker {
        color: Color::from_rgba8(255, 255, 255, 100),
        radius: defaults::constants::GRADIENT_INNER_RADIUS,
    });

    let options = GridOptions {
        line_thickness: defaults::constants::LINE_THICKNESS,
        pattern_options: GridPatternOptions::gen_changing_gradient(
            Intersections::EndsAndMiddle {
                start: defaults::components::GRADIENT_START_POINT.clone(),
                end: transparent_dot.into(),
                middle: transparent_dot,
            },
            palettes::ALL.to_vec(),
            true,
        ),
    };*/

    grid.draw_grid_to_file("text.png", scale, &options);
}
