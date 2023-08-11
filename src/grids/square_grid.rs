use tiny_skia::Pixmap;

use crate::options::GridOptions;
use crate::{pattern_utils::HexCoord, Pattern};

use super::{GridCreationError, GridDraw, GridDrawError};

pub struct SquareGrid {
    patterns: Vec<(Pattern, HexCoord, f32)>,
    size: HexCoord,
}

impl SquareGrid {
    pub fn new(
        patterns: Vec<Pattern>,
        max_width: usize,
        max_scale: f32,
        x_pad: f32,
        y_pad: f32,
    ) -> Result<Self, GridCreationError> {
        if patterns.len() == 0 {
            return Err(GridCreationError::EmptyPatternList);
        } else if max_width == 0 || x_pad < 0.0 || y_pad < 0.0 {
            return Err(GridCreationError::NegativeInput);
        }
        let mut new_patterns: Vec<(Pattern, HexCoord, f32)> = Vec::new();

        for (i, pattern) in patterns.clone().into_iter().enumerate() {
            let y = i / max_width;
            let x = i - y * max_width;

            let x = x as f32 * (1.0 + x_pad);
            let y = y as f32 * (1.0 + y_pad);

            let pos = HexCoord(x, y);

            let area = pattern.bottom_right_bound - pattern.top_left_bound;

            let largest_bound = area.0.max(area.1);

            let scale = (1.0 / largest_bound).min(max_scale);

            let center = area / 2.0 + pattern.top_left_bound;

            let pattern_loc = pos + HexCoord(0.5, 0.5) - center * scale;

            new_patterns.push((pattern, pattern_loc, scale));
        }

        let size = HexCoord(
            max_width.min(new_patterns.len()) as f32 * (1.0 + x_pad) - x_pad,
            (new_patterns.len() as f32 / max_width as f32).ceil() * (1.0 + y_pad) - y_pad,
        );

        Ok(Self {
            patterns: new_patterns,
            size,
        })
    }
}

impl GridDraw for SquareGrid {
    fn draw_grid(&self, scale: f32, options: &GridOptions) -> Result<Pixmap, GridDrawError> {
        super::draw_grid(self.size, &self.patterns, options, scale)
    }
}
