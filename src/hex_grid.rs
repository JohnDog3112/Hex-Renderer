use std::fs;

use tiny_skia::Pixmap;

use crate::{
    options::{GridOptions, GridPatternOptions},
    pattern::Pattern,
    pattern_utils::{Angle, Coord, HexCoord},
};

#[derive(Debug)]
pub struct HexGrid {
    pub patterns: Vec<Pattern>,
    pub locations: Vec<Coord>,
    pub bottom_right: HexCoord,
    pub offset_left: bool,
}

impl HexGrid {
    pub fn generate_grid(patterns: Vec<Pattern>, max_width: i32) -> Self {
        let mut locations = Vec::new();

        let max_width = max_width as f32;

        let mut current_x = 0;
        let mut current_x_offset = 0;
        let mut current_y = 0;

        let mut max_y_row = 0;
        let mut max_x = 0.0;

        let mut offset_left = true;

        for index in 0..patterns.len() {
            let pattern = &patterns[index];
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
                let prev_pattern = &patterns[index - 1];
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

                for point in &patterns[index - 1].right_perimiter {
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
            for point in &patterns[index].right_perimiter {
                let point = HexCoord::from(*point + locations[index]);
                if point.0 > max_x {
                    max_x = point.0;
                }
            }
        }

        HexGrid {
            patterns,
            locations,
            bottom_right: HexCoord(max_x, HexCoord::get_y(current_y + max_y_row)),
            offset_left,
        }
    }

    pub fn draw_grid_to_file(&self, file_name: &str, scale: f32, options: &GridOptions) {
        fs::write(file_name, self.draw_grid(scale, options)).unwrap();
    }

    pub fn draw_grid(&self, scale: f32, options: &GridOptions) -> Vec<u8> {
        let intersections;
        let lines;

        match &options.pattern_options {
            GridPatternOptions::Uniform(inter, lin) => {
                intersections = vec![inter];
                lines = vec![lin];
            }
            GridPatternOptions::Changing(variations) => {
                (intersections, lines) = variations.into_iter().map(|a| (&a.0, &a.1)).unzip();
            }
        }

        let mut max_radius = options.line_thickness;

        for i in 0..lines.len() {
            max_radius = max_radius
                .max(intersections[i].get_max_radius())
                .max(lines[i].get_max_radius());
        }

        let border_size = max_radius * scale;

        let offset = HexCoord(border_size, border_size);
        let map_size = self.bottom_right * scale + offset * 2.0;

        let mut left_offset = 0.0;
        if self.offset_left {
            left_offset = 0.5;
        }
        let mut pixmap =
            Pixmap::new((map_size.0 - left_offset * scale) as u32, map_size.1 as u32).unwrap();

        let mut lines_index = 0;

        let mut increment = false;

        let intro_pattern = vec![Angle::Left, Angle::Left, Angle::Left];
        let retro_pattern = vec![Angle::Right, Angle::Right, Angle::Right];

        for i in 0..self.patterns.len() {
            let pattern = &self.patterns[i];
            let location =
                (HexCoord::from(self.locations[i]) - HexCoord(left_offset, 0.0)) * scale + offset;

            if pattern.angles == intro_pattern {
                increment = true;
            } else if pattern.angles == retro_pattern {
                if lines_index == 0 {
                    lines_index = lines.len() - 1;
                } else {
                    lines_index -= 1;
                }
            }

            pattern.draw_pattern(
                &mut pixmap,
                location,
                scale,
                options.line_thickness,
                &lines[lines_index],
                &intersections[lines_index],
            );

            if increment {
                increment = false;
                lines_index = (lines_index + 1) % lines.len();
            }
        }

        pixmap.encode_png().unwrap()
    }
}
