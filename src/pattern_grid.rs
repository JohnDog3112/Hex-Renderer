use crate::{coord::Coord, hex_coord::HexCoord};

use super::Pattern;

#[derive(Debug)]
pub struct PatternGrid {
    pub patterns: Vec<Pattern>,
    pub locations: Vec<Coord>,
    pub bottom_right: HexCoord,
}

impl PatternGrid {
    pub fn generate(patterns: Vec<Pattern>, max_width: i32) -> Self {
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

            if HexCoord::from(Coord(current_x+pattern.bottom_right.0, max_y_row)).0 > max_width {
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
}