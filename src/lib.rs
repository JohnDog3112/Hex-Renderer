#![feature(test)]
pub mod pattern_utils;

mod pattern;
pub use pattern::Pattern;

pub mod grids;

pub mod options;
pub use options::defaults;

pub use tiny_skia::Color;

extern crate test;

#[cfg(test)]
mod tests {
    use crate::grids::{GridDraw, HexGrid};

    use super::*;
    use test::Bencher;

    #[bench]
    fn test(b: &mut Bencher) {
        let patterns_str = "HexPattern(EAST waqa), HexPattern(EAST aadaa), HexPattern(SOUTH_EAST awdwaaww), HexPattern(EAST waqwwaqa), HexPattern(NORTH_EAST deddw), HexPattern(EAST ad), HexPattern(SOUTH_EAST awdwaaww), HexPattern(SOUTH_EAST awdwa), HexPattern(SOUTH_EAST awdd), HexPattern(NORTH_EAST waawaqwawqq), HexPattern(EAST aadaa), HexPattern(SOUTH_EAST awdwaaww), HexPattern(EAST waqwwaqa), HexPattern(EAST aadaa), HexPattern(SOUTH_EAST aqaawa), HexPattern(EAST aada), HexPattern(EAST aawdd), HexPattern(SOUTH_EAST aqaa), HexPattern(EAST aawdd), HexPattern(EAST aqwwaqwaad), HexPattern(EAST aawdd), HexPattern(SOUTH_EAST aqaaw), HexPattern(NORTH_EAST waaw), HexPattern(SOUTH_EAST aqaawww), HexPattern(WEST ddad), HexPattern(EAST aadaa), HexPattern(EAST waqaeaq), HexPattern(SOUTH_EAST aqaawww), HexPattern(WEST ddad), HexPattern(EAST aawdd), HexPattern(EAST aqwwaqwaad), HexPattern(EAST aawdd), HexPattern(EAST aadaa), HexPattern(EAST aqwaq), HexPattern(NORTH_EAST aw), HexPattern(SOUTH_EAST aqaaw), HexPattern(EAST aada), HexPattern(EAST aqwaq), HexPattern(SOUTH_EAST aqaaedwd), HexPattern(EAST aada), HexPattern(SOUTH_EAST aqaawa), HexPattern(SOUTH_WEST ewdqdwe), HexPattern(SOUTH_EAST aqaaw), HexPattern(SOUTH_EAST aqaawaa), HexPattern(WEST ddad), HexPattern(SOUTH_EAST aqaaq), HexPattern(WEST ddad), HexPattern(EAST aadaa), HexPattern(SOUTH_EAST awdwa), HexPattern(EAST ad), HexPattern(EAST aawdd), HexPattern(SOUTH_EAST awdwaaww), HexPattern(EAST aawdd), HexPattern(NORTH_EAST waawaqwawqq), HexPattern(SOUTH_EAST awdwa), HexPattern(EAST aawdd), HexPattern(SOUTH_EAST awdd), HexPattern(NORTH_EAST waawaqwawqq), HexPattern(SOUTH_EAST aqaawa), HexPattern(SOUTH_WEST ewdqdwe), HexPattern(SOUTH_EAST awdd), HexPattern(NORTH_WEST qwaeawq), HexPattern(NORTH_EAST qqaeaae), HexPattern(NORTH_EAST dwqqqqqwddww), HexPattern(EAST aadaa), HexPattern(EAST aqaeaq), HexPattern(WEST qqq), HexPattern(SOUTH_WEST aaqwqaa), HexPattern(SOUTH_EAST aqaaedwd), HexPattern(WEST ddad), HexPattern(EAST aawdd), HexPattern(EAST aadaa), HexPattern(SOUTH_EAST aweeeeewaaww), HexPattern(EAST aawdd), HexPattern(EAST wawqwawaw), HexPattern(NORTH_EAST dedq), HexPattern(WEST dwwdwwdwdd), HexPattern(WEST qqq), \"\\\", HexPattern(EAST eee), HexPattern(NORTH_WEST qwaeawq), HexPattern(EAST aawdd), HexPattern(NORTH_EAST waawaqwawqq), HexPattern(WEST qqq), \"/\", HexPattern(EAST eee), HexPattern(NORTH_WEST qwaeawq), HexPattern(NORTH_EAST waawaqwawqq), HexPattern(SOUTH_EAST aqaawaa), HexPattern(EAST aada), HexPattern(EAST waqwwaqa), HexPattern(NORTH_EAST deddw), HexPattern(EAST ad), HexPattern(WEST qqq), HexPattern(SOUTH_EAST a), HexPattern(EAST eee), HexPattern(NORTH_WEST qwaeawq), HexPattern(WEST qqq), HexPattern(SOUTH_WEST edqde), HexPattern(EAST eee), HexPattern(NORTH_WEST qwaeawq), HexPattern(SOUTH_EAST awdd), HexPattern(SOUTH_EAST deaqq), HexPattern(EAST aawdd), HexPattern(EAST eee), HexPattern(EAST aawdd), HexPattern(NORTH_EAST qeewdweddw), HexPattern(SOUTH_EAST aqaaedwd), HexPattern(WEST ddad), HexPattern(SOUTH_EAST aqaaedwd), HexPattern(WEST ddad), HexPattern(EAST aadaa), HexPattern(SOUTH_EAST aqaaw), HexPattern(NORTH_WEST wddw), HexPattern(NORTH_WEST eqqwawqaaw), HexPattern(EAST aadaadaa), HexPattern(WEST qqq), HexPattern(NORTH_WEST qaeaq), HexPattern(EAST eee), HexPattern(NORTH_WEST qwaeawq), HexPattern(NORTH_EAST qeewdweddw), HexPattern(EAST aadaadaa), HexPattern(NORTH_EAST qeewdweddw), HexPattern(SOUTH_WEST ewdqdwe), HexPattern(SOUTH_EAST deaqq), HexPattern(EAST aawdd), HexPattern(NORTH_WEST eqqwawqaaw), HexPattern(SOUTH_EAST deaqq), HexPattern(SOUTH_EAST aeea), HexPattern(EAST aadaa), HexPattern(EAST aqaeaq), HexPattern(EAST aadaa), HexPattern(SOUTH_EAST aqaaw), HexPattern(EAST ad), HexPattern(WEST qqq), HexPattern(NORTH_WEST qwaeawq), HexPattern(SOUTH_WEST aaqwqaa), HexPattern(SOUTH_EAST ae), HexPattern(NORTH_EAST dedq), HexPattern(EAST eee), HexPattern(WEST qqq), HexPattern(NORTH_EAST de), HexPattern(WEST qqq), \"Too Many Matches Found!\", HexPattern(EAST eee), HexPattern(NORTH_WEST qwaeawq), HexPattern(NORTH_EAST de), HexPattern(SOUTH_EAST adada), HexPattern(SOUTH_EAST aqae), HexPattern(EAST eee), HexPattern(SOUTH_EAST awdd), HexPattern(EAST aawdd), HexPattern(SOUTH_EAST aqaa), HexPattern(EAST ad), HexPattern(EAST aawdd), HexPattern(WEST qqq), HexPattern(WEST qqq), \"No Matches Found\", HexPattern(EAST eee), HexPattern(NORTH_WEST qwaeawq), HexPattern(NORTH_EAST de), HexPattern(SOUTH_EAST adada), HexPattern(SOUTH_EAST aqae), HexPattern(EAST eee), HexPattern(EAST aawdd), HexPattern(SOUTH_EAST awdd), HexPattern(SOUTH_EAST deaqq), HexPattern(WEST qqq), HexPattern(SOUTH_WEST aqdee), HexPattern(EAST eee), HexPattern(NORTH_WEST qwaeawq), HexPattern(WEST qqq), HexPattern(EAST), HexPattern(EAST eee), HexPattern(NORTH_WEST qwaeawq), HexPattern(SOUTH_EAST awdd), HexPattern(SOUTH_EAST deaqq), HexPattern(EAST aawdd), HexPattern(EAST eaqaaw), HexPattern(NORTH_EAST qaq), HexPattern(SOUTH_WEST aa), HexPattern(EAST qded), HexPattern(SOUTH_EAST a)";
        let patterns: Vec<Pattern> = patterns_str
            .split(", ")
            .filter_map(|str| Pattern::try_from(str).map_or(None, |pattern| Some(pattern)))
            .collect();
        let grid = HexGrid::new(patterns.clone(), 30).unwrap();
        b.iter(|| grid.draw_grid_png(50.0, &defaults::SEGMENT));
    }
}
