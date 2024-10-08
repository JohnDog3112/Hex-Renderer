//! All of the grids to draw images on.
//! 
//! Includes [SquareGrid] and [HexGrid] which are rendered using [GridDraw]
//! 


mod hex_grid;
pub use hex_grid::HexGrid;

mod square_grid;
pub use square_grid::SquareGrid;

use std::{collections::HashSet, fs, io};

use tiny_skia::Pixmap;

use crate::{
    options::{GridOptions, GridPatternOptions, Intersections, Lines},
    pattern::PatternVariant,
    pattern_utils::HexCoord,
};

#[derive(Debug)]
pub enum GridError {
    FileError(GridFileError),
    DrawError(GridDrawError),
    CreationError(GridCreationError),
}

#[derive(Debug)]
pub enum GridFileError {
    SaveError(io::Error),
    DrawError(GridDrawError),
}
#[derive(Debug)]
pub enum GridDrawError {
    ImproperScale(f32),
    EncodeError,
}
#[derive(Debug)]
pub enum GridCreationError {
    NegativeInput,
    EmptyPatternList,
}

///Set of function for drawing grids
pub trait GridDraw {

    ///Draws the grid with a given padding around it
    /// * scale - Size (in pixels) of width (distance between points for [HexGrid], tile_size for [SquareGrid])
    /// * options - [GridOptions] for rendering the patterns
    /// * padding - Amount of padding around grid as a percentage of scale
    fn draw_grid_with_padding(&self, scale: f32, options: &GridOptions, padding: f32) -> Result<Pixmap, GridDrawError>;

    ///Draws a grid with padding based on the options
    /// * scale - Size (in pixels) of width (distance between points for [HexGrid], tile_size for [SquareGrid])
    /// * options - [GridOptions] for rendering patterns
    fn draw_grid(&self, scale: f32, options: &GridOptions) -> Result<Pixmap, GridDrawError> {
        let max_radius = options.get_max_radius();

        self.draw_grid_with_padding(scale, options, max_radius * 1.1)
    }

    ///Size of grid without padding
    fn get_unpadded_size(&self) -> (f32, f32);

    ///Size of grid with automatic padding based on [GridOptions]
    fn get_size(&self, options: &GridOptions) -> (f32, f32) {
        let max_radius = options.get_max_radius();

        let size = self.get_unpadded_size();
        (max_radius * 2.0 * 1.1 + size.0, max_radius * 2.0 * 1.1 + size.1)
    }

    ///Scale needed to draw a grid that fits within the given bound
    /// Uses padding generated from [GridOptions]
    fn get_bound_scale(&self, bound: (f32, f32), options: &GridOptions) -> f32 {
        let size = self.get_size(options);

        (bound.0 / size.0).min(bound.1 / size.1).max(1.0)
    }

    ///Renders/Draws the image as a png returned as a vector of bytes.
    /// * scale - Size (in pixels) of width (distance between points for [HexGrid], tile_size for [SquareGrid])
    /// * options - [GridOptions] for rendering patterns
    fn draw_grid_png(&self, scale: f32, options: &GridOptions) -> Result<Vec<u8>, GridDrawError> {
        self.draw_grid(scale, options)?
            .encode_png()
            .map_err(|_| GridDrawError::EncodeError)
    }

    ///Renders/Draws the image as a png and saves it to the given file
    /// * file_name - Name of the file to save the image as
    /// * scale - Size (in pixels) of width (distance between points for [HexGrid], tile_size for [SquareGrid])
    /// * options - [GridOptions] for rendering patterns
    fn draw_grid_to_file(
        &self,
        file_name: &str,
        scale: f32,
        options: &GridOptions,
    ) -> Result<(), GridFileError> {
        fs::write(
            file_name,
            self.draw_grid_png(scale, options)
                .map_err(GridFileError::DrawError)?,
        )
        .map_err(GridFileError::SaveError)
    }
}

fn draw_grid_with_padding(
    size: HexCoord,
    patterns: &Vec<(PatternVariant, HexCoord, f32)>,
    options: &GridOptions,
    scale: f32,
    padding: f32,
) -> Result<Pixmap, GridDrawError> {
    if scale < 1.0 {
        return Err(GridDrawError::ImproperScale(scale));
    }

    let intersections;
    let lines;
    let intro_angles;
    let retro_angles;

    match &options.pattern_options {
        GridPatternOptions::Uniform(inter, lin) => {
            intersections = vec![inter];
            lines = vec![lin];
            intro_angles = vec![];
            retro_angles = vec![];
        }
        GridPatternOptions::Changing {
            variations,
            intros,
            retros,
        } => {
            (intersections, lines) = variations.iter().map(|a| (&a.0, &a.1)).unzip();
            intro_angles = intros.clone();
            retro_angles = retros.clone();
        }
    }
    let (intros, retros) = {
        let mut intros = HashSet::new();
        let mut retros = HashSet::new();
        for intro in intro_angles {
            intros.insert(intro);
        }
        for retro in retro_angles {
            retros.insert(retro);
        }
        (intros, retros)
    };

    let monocolor_lines = lines
        .iter()
        .map(|line| Lines::Monocolor {
            bent: false,
            color: match line {
                Lines::Monocolor { color, bent: _ } => *color,
                Lines::Gradient {
                    colors,
                    segments_per_color: _,
                    bent: _,
                } => colors[0],
                Lines::SegmentColors {
                    colors,
                    triangles: _,
                    collisions: _,
                } => colors[0],
            },
        })
        .collect::<Vec<Lines>>();

    let monocolor_intersections = intersections
        .iter()
        .map(|intersection| match intersection {
            Intersections::Nothing => Intersections::Nothing,
            Intersections::UniformPoints(point) => Intersections::UniformPoints(*point),
            Intersections::EndsAndMiddle {
                start: _,
                end: _,
                middle,
            } => Intersections::UniformPoints(*middle),
        })
        .collect::<Vec<Intersections>>();

    let border_size = padding * scale;

    let offset = HexCoord(border_size, border_size);

    let mut pixmap = Pixmap::new(
        (border_size * 2.0 + size.0 * scale) as u32,
        (border_size * 2.0 + size.1 * scale) as u32,
    )
    .unwrap();

    let mut lines_index = 0;

    let mut increment = false;

    for (pattern, location, local_scale) in patterns {
        let location = *location * scale + offset;

        if intros.contains(&pattern.get_inner().angles) {
            increment = true;
        } else if retros.contains(&pattern.get_inner().angles) {
            if lines_index == 0 {
                lines_index = lines.len() - 1;
            } else {
                lines_index -= 1;
            }
        }
        match pattern {
            PatternVariant::Normal(pattern) => {
                pattern.draw_pattern(
                    &mut pixmap,
                    location,
                    scale * *local_scale,
                    options.line_thickness,
                    lines[lines_index],
                    intersections[lines_index],
                    &options.center_dot,
                );
            }
            PatternVariant::Monocolor(pattern) => {
                pattern.draw_pattern(
                    &mut pixmap,
                    location,
                    scale * *local_scale,
                    options.line_thickness,
                    &monocolor_lines[lines_index],
                    &monocolor_intersections[lines_index],
                    &options.center_dot,
                );
            }
        }

        if increment {
            increment = false;
            lines_index = (lines_index + 1) % lines.len();
        }
    }

    Ok(pixmap)
}
