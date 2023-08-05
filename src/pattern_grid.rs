use std::fs;

use tiny_skia::Pixmap;

use crate::{pattern_grid_options::{GridDrawOptions, GridOptions}, pattern_utils::{HexCoord, Coord, Angle}};

use super::Pattern;

#[derive(Debug)]
pub struct PatternGrid {
    pub patterns: Vec<Pattern>,
    pub locations: Vec<Coord>,
    pub bottom_right: HexCoord,
}

impl PatternGrid {
    pub fn generate_grid(patterns: Vec<Pattern>, max_width: i32) -> Self {
        let mut locations = Vec::new();

        let max_width = max_width as f32;

        let mut current_x = 0;
        let mut current_x_offset = 0;
        let mut current_y = 0;

        let mut max_y_row = 0;
        let mut max_x = 0.0;

        for index in 0..patterns.len() {
            let pattern = &patterns[index];
            let height = pattern.bottom_right.1 - pattern.top_left.1;

            if index == 0 {
                current_x -= pattern.top_left.0;
                let mut left_most = f32::MAX;
                for point in &pattern.left_perimiter {
                    let point = HexCoord::from(*point + Coord(current_x, current_y - pattern.top_left.1));
                    if point.0 < left_most {
                        left_most = point.0;
                    }
                }
                current_x_offset = -left_most as i32;


            } else {
                let prev_pattern = &patterns[index-1];
                let mut max_distance_decrease = i32::MAX;
                for i in 0..pattern.left_perimiter.len().min(prev_pattern.left_perimiter.len()) {
                    let right_point = pattern.left_perimiter[i].0;
                    let left_point = prev_pattern.right_perimiter[i].0;

                    let dist = right_point - left_point;

                    //println!("{} - {} = {}", right_point, left_point, dist);

                    if dist < max_distance_decrease {
                        max_distance_decrease = dist;
                    }
                }
                //println!("max: {}", max_distance_decrease);
                current_x -= max_distance_decrease-1;
            }

            if HexCoord::from(Coord(current_x+pattern.bottom_right.0, max_y_row)).0 > max_width && index != 0 {
                current_x = -pattern.top_left.0;
                current_y += max_y_row + 1;
                
                let mut left_most = f32::MAX;
                for point in &pattern.left_perimiter {
                    let point = HexCoord::from(*point + Coord(current_x, current_y - pattern.top_left.1));
                    if point.0 < left_most {
                        left_most = point.0;
                    }
                }
                current_x_offset = -left_most as i32;
                max_y_row = 0;

                for point in &patterns[index-1].right_perimiter {
                    let point = HexCoord::from(*point + locations[index-1]);
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
            let index = patterns.len()-1;
            for point in &patterns[index].right_perimiter {
                let point = HexCoord::from(*point + locations[index]);
                if point.0 > max_x {
                    max_x = point.0;
                }
            }
        }

        PatternGrid {
            patterns,
            locations,
            bottom_right: HexCoord(max_x, HexCoord::get_y(current_y + max_y_row)),
        }
    }

    pub fn draw_grid_to_file(&self, file_name: &str, options: GridOptions) {
        fs::write(file_name, self.draw_grid(options)).unwrap();
    }

    pub fn draw_grid(&self, options: GridOptions) -> Vec<u8> {

        let intersections;
        let lines;

        match options.draw_options {
            GridDrawOptions::Uniform(inter, lin) => {
                intersections = vec![inter];
                lines = vec![lin];
            },
            GridDrawOptions::Changing(variations) => {
                (intersections, lines) = variations.into_iter().unzip();
            }
        }

        let mut max_radius = options.line_thickness;
        
        for i in 0..lines.len() {
            max_radius = max_radius.max(intersections[i].get_max_radius()).max(lines[i].get_max_radius());
        }


        let border_size = max_radius * options.scale;

        let offset = HexCoord(border_size, border_size);
        let map_size = self.bottom_right * options.scale + offset * 2.0;
        let mut pixmap = Pixmap::new(map_size.0 as u32, map_size.1 as u32).unwrap();

        let mut lines_index = 0;

        let mut increment = false;

        let intro_pattern = vec![Angle::Left, Angle::Left, Angle::Left];
        let retro_pattern = vec![Angle::Right, Angle::Right, Angle::Right];

        for i in 0..self.patterns.len() {
            let pattern = &self.patterns[i];
            let location = HexCoord::from(self.locations[i])*options.scale + offset;

            if pattern.angles == intro_pattern {
                increment = true;
            } else if pattern.angles == retro_pattern {
                if lines_index == 0 {
                    lines_index = lines.len()-1;
                } else {
                    lines_index -= 1;
                }
            }

            pattern.draw_pattern(&mut pixmap, location, options.scale, options.line_thickness, &lines[lines_index], &intersections[lines_index]);

            if increment {
                increment = false;
                lines_index = (lines_index+1)%lines.len();
            }
            
        }

        pixmap.encode_png().unwrap()

    }
}