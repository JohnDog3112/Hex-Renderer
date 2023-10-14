use hex_renderer::{
    grids::{GridDraw, HexGrid},
    options::{
        palettes, CollisionOption, EndPoint, GridOptions, GridPatternOptions, Intersections, Lines,
        Marker, OverloadOptions, Point, Triangle,
    },
    pattern_utils::Angle,
    Pattern,
};
use tiny_skia::Color;

fn main() {
    //patterns to draw
    let patterns = "west qqq, west qqq, west qqq, west qqq, NORTH_EAST qeqwqwqwqwqeqssssaeqeaqeqaeqaqdededwaqdedsssssdssess, east eee, east eee, east eee, east eee";
    let patterns = patterns
        .split(", ")
        .map(Pattern::try_from)
        .collect::<Result<Vec<Pattern>, _>>()
        .expect("Invalid Pattern List!");

    let hex_grid = HexGrid::new_normal(patterns, 50).expect("Failed to make grid!");

    //When drawing the grid, it takes in a GridOptions type
    //it's parameters are as follows:
    //  1. line_thickness
    //      -- thickness of line in comparison to line_width
    //      -- 0.12 means the line thickness is 12% that of it's length
    //  2. pattern_options
    //      -- two types, Uniform and Changing
    //      -- Uniform contains specifications for how to draw the lines
    //      -- of the patterns and the dots to draw on top
    //
    //      -- Changing is a vector of what's in changing
    //      -- + a set of patterns for when to switch
    //      -- for instance, go to the next when seeing the intro patten
    //      -- and going to the previous when seeing the retro pattern
    //  3. center_dot
    //      -- The center dot is a dot put in the center of semi-contained
    //      -- patterns
    //      -- it is defined by a point which will be explained later.

    //example
    let _options = GridOptions {
        //line thickness is 12% of line length
        line_thickness: 0.12,
        //uniform, (all patterns drawn the same)
        pattern_options: GridPatternOptions::Uniform(
            //no intersections (points) on the grid of the pattern
            Intersections::Nothing,
            //monocolor renderer
            Lines::Monocolor {
                //draws the lines with white
                color: Color::WHITE,
                //corners are bent like in the game
                bent: true,
            },
        ),
        //no center dot
        center_dot: Point::None,
    };

    //now, on the next step down, you have the line renderers
    //this is where there's an acutal distinction in "renderers"
    //like mentioned in the previous example, there are 3 types:
    // monocolor - single color
    // gradient - transitions between colors
    // segment - switches between colors on conflict

    //each of these takes a color which is imported via tiny_skia
    //the color format is RGBA (where A is alpha/transparency)
    let color = Color::from_rgba8(0, 128, 128, 255);

    //the monocolor type just takes in the color and if it's bent
    //bent is where the lines bend around corners when multiple
    //lines go throguht the same intersection/corner like in game
    let _monocolor = Lines::Monocolor { color, bent: true };

    //the gradient gradually changes between the colors given
    //throughtout the entire pattern
    //the segments_per_color parameter is to set a minimum
    //amount of segments before transitioning to the next color
    //for example if it's set to 10 segments_per_color:
    //      --  then 1-9 patterns will use 2 colors
    //      --  10-19 will use 3 colors
    //      --  20-29 will use 4 colors and so on until it's out of colors

    //the bent parameter behaves the same as the one in monocolor

    let colors = vec![
        color,
        Color::from_rgba8(255, 0, 0, 255),
        Color::from_rgba8(0, 255, 0, 255),
        Color::from_rgba8(0, 0, 255, 255),
        Color::from_rgba8(128, 128, 128, 255),
        Color::from_rgba8(128, 128, 0, 255),
    ];

    let _gradient = Lines::Gradient {
        //colors to transition between
        colors,
        //minimum segments per switch (starts at 2)
        segments_per_color: 10,
        //whether to bend the lines at corners line in game
        bent: true,
    };

    //the segment option is by far the most complicated
    //like the gradient one, it takes in a list of colors to switch between

    //however, it also has options for the triangles (pointers/arrows) that demonstrate the switch
    //and what to do when you get collisions (two or more lines between the same 2 points)

    //for the triangles/arrows there are several options as follows:

    //none, simply don't draw any arrows between color switches
    let _none = Triangle::None;

    //match simply draws an arrow along color switches
    //that is the same color as the color it's switching from
    //the radius is a percentage of the line width
    let _match = Triangle::Match { radius: 0.16 };

    //border_match is the same as match above, except it adds an extra
    //bordering triangle around (or inside) the matching triangle
    //the smaller radius is drawn on top
    let _bordered_match = Triangle::BorderMatch {
        match_radius: 0.16,
        //the border is a marker which
        //simply holds the radius and color of the border
        border: Marker {
            color: Color::WHITE,
            radius: 0.25,
        },
    };

    //lastly, there's border_start_match
    //this is exactly the same as border_match, except
    //it only draws the border around the starting arrow
    let _border_start_match = Triangle::BorderStartMatch {
        match_radius: 0.16,
        border: Marker {
            color: Color::WHITE,
            radius: 0.25,
        },
    };

    //then, there's the collision options
    //there are several variants as follows:

    //matched dashes keeps track of the color of each line passing through it
    //and draws the colors as a dashed line in place of a solid line
    let _matched_dashes = CollisionOption::MatchedDashes;

    //this one draws the first line (before collisions)
    //and then draws a dash with the provided color over it
    let _dashes = CollisionOption::Dashes(Color::from_rgba8(255, 0, 0, 255));

    //parallel lines just draws the lines parallel to eachother (while shrinking them)
    //so that the colliding lines sort of just push eachother to the side
    let _parallel_lines = CollisionOption::ParallelLines;

    //lastly, there's overloaded parallel lines
    //this is an extension to parallel_lines for cases
    //where there are so many colliding lines it's hard to read

    //it takes in the number of lines before it switches methods (max_line)
    //and the alternate method (overload)

    //overlaod has 3 options as follows:
    //same as _dashes except it's over the parallel lines
    let _dashes_overload = OverloadOptions::Dashes(Color::from_rgba8(255, 0, 0, 255));

    //same as _matched_dashes
    let _matched_dashes_overload = OverloadOptions::MatchedDashes;

    //this is an extension to _dashes_overload
    //it adds a label off to the side that displays how many colliding lines there are
    //the label is a Marker that specifies the color and the radius of the label (circular)
    let _labeled_dashes_overload = OverloadOptions::LabeledDashes {
        color: Color::from_rgba8(255, 0, 0, 255),
        label: Marker {
            color: Color::WHITE,
            radius: 0.4,
        },
    };

    //puting that together, you get the overloaded_parallel_lines
    let _overloaded_parallel_lines = CollisionOption::OverloadedParallel {
        max_line: 10,
        overload: _labeled_dashes_overload,
    };

    let colors = vec![
        color,
        Color::from_rgba8(255, 0, 0, 255),
        Color::from_rgba8(0, 255, 0, 255),
        Color::from_rgba8(0, 0, 255, 255),
        Color::from_rgba8(128, 128, 128, 255),
        Color::from_rgba8(128, 128, 0, 255),
    ];

    //and then, all of that can be combined to make the SegmentColor options
    let _segment = Lines::SegmentColors {
        colors,
        triangles: _border_start_match,
        collisions: _overloaded_parallel_lines,
    };

    //then, from there, you have intersections
    //the intersections are simply points the points
    //on the grid of the pattern

    //it has three types as follows:
    //nothing which is simply no points at the intersections
    let _nothing = Intersections::Nothing;

    //uniform points simply uses the same point for every intersection
    //where a point has the following variants:
    //none which is just nothing
    let _none = Point::None;

    //single which is just a marker specifying the color and radius
    let _single_point = Point::Single(Marker {
        color: Color::WHITE,
        radius: 0.07,
    });

    //and finally double which has an inner and outer point defined by two markers
    let _double_point = Point::Double {
        inner: Marker {
            color: Color::WHITE,
            radius: 0.07,
        },
        outer: Marker {
            color: Color::from_rgba8(255, 255, 0, 255),
            radius: 0.1,
        },
    };

    //witht those, the UniformPoint intesection can be defined:
    let _uniform_points = Intersections::UniformPoints(_single_point);

    //lastly, you have EndAndMiddle
    //this takes in an EndPoint for the start and end and a Point for the middle

    //the EndPoint is just a modified variation of Point
    //that doesn't include the inner color as it's provided by the stating/ending color

    //the point variant is just a wrapper for a normal Point
    //could alternatively be point.into();
    let _simple_end_point = EndPoint::Point(_single_point);

    //this is equivalent to _single_point except the color is provided
    //from whatever the underlying line color is
    let _single_end_point = EndPoint::Match { radius: 0.07 };

    //same as _double_end_point but with provided color
    let _double_end_point = EndPoint::BorderedMatch {
        match_radius: 0.1,
        border: Marker {
            color: Color::WHITE,
            radius: 0.1,
        },
    };

    //then combined into the EndsAndMiddle intersection you can get this:
    let _end_and_middle_points = Intersections::EndsAndMiddle {
        start: _double_end_point,
        end: _simple_end_point,
        middle: _single_point,
    };

    //for the last part of the main GridOptions, there's the collision point
    //this is simply a point (the same one mentioned above)
    //and is automatically placed in the middle if provided

    let _collision_point = Point::Single(Marker {
        color: Color::WHITE,
        radius: 0.1,
    });

    //then putting it all together:
    let _uniform_options = GridOptions {
        line_thickness: 0.12,
        pattern_options: GridPatternOptions::Uniform(_end_and_middle_points, _segment.clone()),
        center_dot: _collision_point,
    };
    hex_grid
        .draw_grid_to_file("uniform_options_example.png", 50.0, &_uniform_options)
        .expect("Failed to write to file!");

    //for the changing GridPatternOptions, 2 extra parameters are required:
    //a set of pattern_sigs to progress to the next pattern option set
    //and a set to go to the previous

    //here's the setup for intro/retro
    //it only takes in a vector of angle_sigs as the
    //starting direction is ignored
    let _progress_patterns = vec![vec![Angle::Left, Angle::Left, Angle::Left]];
    let _regress_patterns = vec![vec![Angle::Right, Angle::Right, Angle::Right]];

    //then for the changing options you just list several intersection/line options and the above patterns
    let _changing_options = GridOptions {
        line_thickness: 0.12,
        pattern_options: GridPatternOptions::Changing {
            variations: vec![
                (_end_and_middle_points, _segment),
                (_uniform_points, _gradient),
                (_nothing, _monocolor),
            ],
            intros: _progress_patterns,
            retros: _regress_patterns,
        },
        center_dot: _collision_point,
    };

    hex_grid
        .draw_grid_to_file("changing_options_example.png", 50.0, &_changing_options)
        .expect("Failed to write to file!");

    //since the changing variant probably only needs to switch colors,
    //there are some built in functions for that
    //however, they set the intro/retro patterns and don't allow configuration of those

    //and if you don't want to program in custom palettes, there's options::palettes

    //for monocolor, you just give whether it bends and a vector of colors
    //it will generate a variant of monocolor for each color provided
    let _changing_monocolor =
        GridPatternOptions::gen_changing_monocolor(_nothing, palettes::DARK2.to_vec(), true);

    //gen_changing_gradient takes in the intersection
    //a vector of vector of colors and whether or not it's bent.
    //it generated a gradient for each vector of colors given with the bent condition passed down
    let _changing_segment = GridPatternOptions::gen_changing_gradient(
        _end_and_middle_points,
        palettes::ALL.to_vec(),
        true,
    );

    //same as gradient, except it also takes int he triangles/arrows and collision options
    //uses the same triangle/collision settings for all variants (only changes line color)
    let _changing_gradient = GridPatternOptions::gen_changing_segment(
        _end_and_middle_points,
        palettes::ALL.to_vec(),
        _border_start_match,
        _overloaded_parallel_lines,
    );

    let _generator_options = GridOptions {
        line_thickness: 0.12,
        pattern_options: _changing_gradient,
        center_dot: _single_point,
    };

    hex_grid
        .draw_grid_to_file("generator_options_example.png", 50.0, &_generator_options)
        .expect("Failed to write to file!");

    //in addition, if you want to use the defaults for some things,
    //all the parts to make the defaults are in hex_renderer::options::defaults
}
