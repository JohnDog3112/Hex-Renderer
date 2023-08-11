mod hex_grid;
pub use hex_grid::HexGrid;

mod square_grid;
pub use square_grid::SquareGrid;

use std::{collections::HashSet, fs, io};

use tiny_skia::Pixmap;

use crate::{
    options::{GridOptions, GridPatternOptions},
    pattern_utils::HexCoord,
    Pattern,
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

pub trait GridDraw {
    fn draw_grid(&self, scale: f32, options: &GridOptions) -> Result<Pixmap, GridDrawError>;

    fn draw_grid_png(&self, scale: f32, options: &GridOptions) -> Result<Vec<u8>, GridDrawError> {
        self.draw_grid(scale, options)?
            .encode_png()
            .map_err(|_| GridDrawError::EncodeError)
    }
    fn draw_grid_to_file(
        &self,
        file_name: &str,
        scale: f32,
        options: &GridOptions,
    ) -> Result<(), GridFileError> {
        fs::write(
            file_name,
            self.draw_grid_png(scale, options)
                .map_err(|err| GridFileError::DrawError(err))?,
        )
        .map_err(|err| GridFileError::SaveError(err))
    }
}

fn draw_grid(
    size: HexCoord,
    patterns: &Vec<(Pattern, HexCoord, f32)>,
    options: &GridOptions,
    scale: f32,
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
            (intersections, lines) = variations.into_iter().map(|a| (&a.0, &a.1)).unzip();
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

    let max_radius = options.get_max_radius();

    let border_size = max_radius * scale;

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

        if intros.contains(&pattern.angles) {
            increment = true;
        } else if retros.contains(&pattern.angles) {
            if lines_index == 0 {
                lines_index = lines.len() - 1;
            } else {
                lines_index -= 1;
            }
        }

        pattern.draw_pattern(
            &mut pixmap,
            location,
            scale * *local_scale,
            options.line_thickness,
            &lines[lines_index],
            &intersections[lines_index],
            &options.center_dot,
        );

        if increment {
            increment = false;
            lines_index = (lines_index + 1) % lines.len();
        }
    }

    Ok(pixmap)
}
