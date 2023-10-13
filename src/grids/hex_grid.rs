use tiny_skia::Pixmap;

use crate::{
    options::GridOptions,
    pattern::PatternVariant,
    pattern_utils::{Coord, HexCoord},
    Pattern,
};

use super::{GridCreationError, GridDraw, GridDrawError};

#[derive(Debug)]
pub struct HexGrid {
    patterns: Vec<(PatternVariant, HexCoord, f32)>,
    bottom_right: HexCoord,
}

impl HexGrid {
    pub fn new_normal(patterns: Vec<Pattern>, max_width: usize) -> Result<Self, GridCreationError> {
        Self::new(
            patterns.into_iter().map(PatternVariant::Normal).collect(),
            max_width,
        )
    }
    pub fn new(patterns: Vec<PatternVariant>, max_width: usize) -> Result<Self, GridCreationError> {
        if patterns.is_empty() {
            return Err(GridCreationError::EmptyPatternList);
        } else if max_width < 1 {
            return Err(GridCreationError::NegativeInput);
        }
        let mut locations = Vec::new();

        let max_width = max_width as f32;

        let mut current_x = 0;
        let mut current_x_offset = 0;
        let mut current_y = 0;

        let mut max_y_row = 0;
        let mut max_x = 0.0;

        let mut offset_left = true;

        for index in 0..patterns.len() {
            let pattern = &patterns[index].get_inner();
            let height = pattern.bottom_right.1 - pattern.top_left.1;

            if index == 0 {
                current_x -= pattern.top_left.0;
                let mut left_most = f32::MAX;
                for point in &pattern.left_perimiter {
                    let point =
                        HexCoord::from(*point + Coord(current_x, current_y - pattern.top_left.1));
                    if point.0 < left_most {
                        left_most = point.0;
                    }
                }
                current_x_offset = -left_most as i32;

                if left_most - left_most.floor() < 0.45 {
                    offset_left = false;
                }
            } else {
                let prev_pattern = patterns[index - 1].get_inner();
                let mut max_distance_decrease = i32::MAX;
                for i in 0..pattern
                    .left_perimiter
                    .len()
                    .min(prev_pattern.left_perimiter.len())
                {
                    let right_point = pattern.left_perimiter[i].0;
                    let left_point = prev_pattern.right_perimiter[i].0;

                    let dist = right_point - left_point;

                    //println!("{} - {} = {}", right_point, left_point, dist);

                    if dist < max_distance_decrease {
                        max_distance_decrease = dist;
                    }
                }
                //println!("max: {}", max_distance_decrease);
                current_x -= max_distance_decrease - 1;
            }

            if HexCoord::from(Coord(current_x + pattern.bottom_right.0, max_y_row)).0 > max_width
                && index != 0
            {
                current_x = -pattern.top_left.0;
                current_y += max_y_row + 1;

                let mut left_most = f32::MAX;
                for point in &pattern.left_perimiter {
                    let point =
                        HexCoord::from(*point + Coord(current_x, current_y - pattern.top_left.1));
                    if point.0 < left_most {
                        left_most = point.0;
                    }
                }
                current_x_offset = -left_most as i32;

                if left_most - left_most.floor() < 0.45 {
                    offset_left = false;
                }

                max_y_row = 0;

                for point in &patterns[index - 1].get_inner().right_perimiter {
                    let point = HexCoord::from(*point + locations[index - 1]);
                    if point.0 > max_x {
                        max_x = point.0;
                    }
                }
            }

            if height > max_y_row {
                max_y_row = height;
            }

            let loc = Coord(current_x + current_x_offset, current_y - pattern.top_left.1);
            locations.push(loc);
        }

        if current_y == 0 {
            let index = patterns.len() - 1;
            for point in &patterns[index].get_inner().right_perimiter {
                let point = HexCoord::from(*point + locations[index]);
                if point.0 > max_x {
                    max_x = point.0;
                }
            }
        }

        let mut left_offset = HexCoord(0.0, 0.0);
        if offset_left {
            left_offset.0 = 0.5;
        }

        let mut packed_patterns = Vec::new();
        for (i, pattern) in patterns.into_iter().enumerate() {
            let location = HexCoord::from(locations[i]) - left_offset;
            packed_patterns.push((pattern, location, 1.0));
        }

        Ok(HexGrid {
            patterns: packed_patterns,
            bottom_right: HexCoord(
                max_x - left_offset.0,
                HexCoord::get_y(current_y + max_y_row),
            ),
        })
    }
}

impl GridDraw for HexGrid {
    fn draw_grid(&self, scale: f32, options: &GridOptions) -> Result<Pixmap, GridDrawError> {
        super::draw_grid(self.bottom_right, &self.patterns, options, scale)
    }
    fn get_unpadded_size(&self) -> (f32, f32) {
        (self.bottom_right.0, self.bottom_right.1)
    }
}
