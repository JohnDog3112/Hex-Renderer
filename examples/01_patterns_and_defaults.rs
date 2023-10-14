use hex_renderer::grids::GridDraw;
use hex_renderer::grids::HexGrid;
use hex_renderer::grids::SquareGrid;
use hex_renderer::pattern_utils;
use hex_renderer::Pattern;

fn main() {
    //In order to draw a set of patterns, they need to be in a readable format first
    //the simplest format is just a vector of Patterns.

    //there is some built in parsing for strings, however it can only take in
    //one pattern at a time and accepts the following formats:
    // <starting_direction> <angle_sigs>
    // hexpattern(<starting_direction> <angle_sigs>)

    //where starting_direction can be formated in any of the following ways:
    //  -- north_east | northeast | ne -- caps are ignored
    //and angle sigs are any of the following characters: weqdas
    let patterns =
        "HexPattern(north_east qqq), HexPattern(north_east qqq), HexPattern(north_east qqq)";

    //to convert from a string of the above format into a list of patterns
    //you can split it into each pattern and then try to convert it using Pattern::try_from
    let patterns = patterns
        .split(", ")
        .map(Pattern::try_from)
        .collect::<Result<Vec<Pattern>, _>>()
        .expect("Invalid Pattern List!");

    //alternatively, you can build the patterns by hand
    //angle sigs are mapped as followed:
    // w - Forward
    // e - Right
    // d - BackRight
    // s - Backwards
    // a - BackLeft
    // q - Left

    let starting_direction = pattern_utils::Direction::NorthEast;
    let angle_sigs = vec![
        pattern_utils::Angle::Left,
        pattern_utils::Angle::Left,
        pattern_utils::Angle::Left,
    ];

    let built_pattern = Pattern::new(starting_direction, angle_sigs);

    let built_pattern_list = vec![built_pattern.clone(), built_pattern.clone(), built_pattern];

    //another important component for drawing the patterns is their configuration
    //for this, one of the default options will be used, but custom configuration
    //will be in the next example

    //there are 3 different types of pattern rendering as follows
    // monocolor - simply one color
    // gradient - transitions from one color to another
    // segment - switches colors on conflict

    //each of the above types have their own set of defaults. Including their base and below

    //such as pointed-gradient which just adds points to the gradients,
    //a bent_monocolor that adds bends to the corners of the monocolor renderer
    //and uniform variants of each which don't change color
    //   when going inside nested intro/retro blocks

    let monocolor = &hex_renderer::defaults::MONOCOLOR;
    let gradient = &hex_renderer::defaults::GRADIENT;
    let segment = &hex_renderer::defaults::SEGMENT;

    //Then, to actually the patterns on a grid,
    //there are 2 variatns, a hex grid and a square grid
    //creating a grid is seperate from drawing one
    //when creating a grid, it simply aligns and/or scales
    //all the patterns on the grid so they can be drawn later

    //the hex grid is just a hexagonal grid where all of the
    //paterns are aligned side to side in respect to that grid

    //the square grid on the other hand provides a certain
    //block of space for each pattern and then scales them
    //to fit within their alloted block

    //there are two ways to initialize each with patterns
    //however, this tutorial will stick to the simpler way

    //The HexGrid takes in a set of patterns and a max_width
    //the max_width is simply how many points on the grid it should fill
    // (in the x direction) before putting patterns on the next line down
    let max_width = 50;
    let hex_grid = HexGrid::new_normal(patterns, max_width).expect("Failed to make Hex Grid!");

    //for the square grid, max_width is how many tiles (patterns) long
    //each row is rather than the width of the tiles themselves
    let max_width = 10;
    //max_scale sets a cap of how enlarged a pattern can be
    //it stops really small patterns from being enlarged too much
    //must be between 0 and 1 where 1 is no limit
    let max_scale = 0.4;
    //x_pad and y_pad are how much space should be put
    //between the tiles in the x and y axis respectively
    //it is based on the percentage of the tile (in that axis)
    //that should be devoted to padding
    let x_pad = 0.2;
    let y_pad = 0.1;
    let square_grid =
        SquareGrid::new_normal(built_pattern_list, max_width, max_scale, x_pad, y_pad)
            .expect("Failed to make Square Grid!");

    //now to draw the grid, there are 3 main functions
    //draw_grid -- simply draws the grid to a tiny_skia::Pixmap
    //draw_grid_png -- returns grid as a png represented by a vector of bytes
    //draw_grid_to_file -- saves the grid as a png to the provided file

    //each of these types takes in the scale of the grid
    // + the patterns options talked about above

    //for the hex_grid, the scale is how many pixels apart each
    //point on the hex_grid should be spaced apart
    let scale = 50.0;
    hex_grid
        .draw_grid_to_file("monocolor_hex_grid.png", scale, monocolor)
        .expect("Unable to write file!");

    hex_grid
        .draw_grid_to_file("segment_hex_grid.png", scale, segment)
        .expect("Unable to write to file!");

    //for the square_grid, the scale is how many pixels tall/wide each tile (pattern) should be
    let scale = 200.0;
    square_grid
        .draw_grid_to_file("monocolor_square_grid.png", scale, monocolor)
        .expect("Unable to write to file!");

    square_grid
        .draw_grid_to_file("segment_square_grid.png", scale, segment)
        .expect("Unable to write to file!");

    //if you would rather draw things based on the size of the final image,
    //you can call grid.get_bound_scale to get the scale that fits
    //within the bounds you have provided
    //though, since different draw options have slightly different sizes (mainly due to padding),
    //those need to be provided in order to get the proper scaling

    //500 x 500 pixel bound
    let bound = (500.0, 500.0);

    let hex_scale = hex_grid.get_bound_scale(bound, gradient);
    hex_grid
        .draw_grid_to_file("bound_hex_grid.png", hex_scale, gradient)
        .expect("Unable to write to file!");

    let square_scale = square_grid.get_bound_scale(bound, gradient);
    square_grid
        .draw_grid_to_file("bound_square_grid.png", square_scale, gradient)
        .expect("Unable to write to file!");
}
