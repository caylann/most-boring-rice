// This file is part of the colorpanes source code.
//
// ©️ 2022 papojari <mailto:papojari-git.ovoid@aleeas.com> <https://matrix.to/#/@papojari:artemislena.eu> <https://papojari.codeberg.page>
//
// For the license information, please view the README.md file that was distributed with this source code.

use clap::Parser;
use recolored::Color;
use recolored::Colorize;
use std::fmt::Write;

#[derive(Parser, Debug)]
#[clap(
    name = "colorpanes",
    version,
    about = "Panes in the 8 bright terminal colors with shadows of the respective darker color."
)]
struct Cli {
    /// Print the color panes vertically instead of horizontally.
    #[clap(long, short)]
    vertical: bool,

    /// Disable margin around the entire row or column of color panes.
    #[clap(long, short)]
    no_margin: bool,

    /// Width of a color pane.
    #[clap(long, short, default_value = "6")]
    width: u32,

    /// Height of a color pane.
    #[clap(long, short, default_value = "2")]
    height: u32,

    /// Select which colors to print.
    #[clap(long, short, possible_values = ["red", "green", "yellow", "blue", "magenta", "cyan", "white"], default_values = &["red", "green", "yellow", "blue", "magenta", "cyan", "white"], multiple_values = true)]
    colors: Vec<String>,

    /// Show the name of the color before the color.
    #[clap(long, short = 'p')]
    captions: bool,
}

fn main() {
    let cli = Cli::parse();
    let vertical = cli.vertical;
    let margin = match cli.no_margin {
        true => false,
        false => true,
    };
    let width = cli.width;
    let height = cli.height;
    let captions = cli.captions;
    let colors = cli.colors;

    if vertical {
        let vertical_colorpanes = vertical_colorpanes(width, height, captions, margin, colors);

        print!("{}", vertical_colorpanes);
    } else {
        let horizontal_colorpanes =
            horizontal_colorpanes(vertical, width, height, captions, margin, colors);

        print!("{}", horizontal_colorpanes);
    };
}

// Match color name to the respective foreground recolored color.
fn bright_recolor(color: String) -> Color {
    match color.as_str() {
        "red" => return Color::BrightRed,
        "green" => return Color::BrightGreen,
        "yellow" => return Color::BrightYellow,
        "blue" => return Color::BrightBlue,
        "magenta" => return Color::BrightMagenta,
        "cyan" => return Color::BrightCyan,
        "white" => return Color::BrightWhite,
        _ => panic!("{} is not a valid color.", color),
    };
}

// Match color name to the respective background recolored color.
fn recolor(color: String) -> Color {
    match color.as_str() {
        "red" => return Color::Red,
        "green" => return Color::Green,
        "yellow" => return Color::Yellow,
        "blue" => return Color::Blue,
        "magenta" => return Color::Magenta,
        "cyan" => return Color::Cyan,
        "white" => return Color::White,
        _ => panic!("{} is not a valid color.", color),
    };
}

// Return the row of a single color pane.
fn colorpane_row(
    vertical: bool,
    row_type: String,
    width: u32,
    margin: bool,
    bright_recolor: Color,
    recolor: Color,
) -> String {
    let block = "█";
    let top_half_block = "▀";
    let bottom_half_block = "▄";
    let mut horizontal_chars = String::new();

    // A counter variable
    let mut n = 0;
    // Loop while `n` is less than the color pane width
    while n < width {
        write!(horizontal_chars, "{}", block).unwrap();

        // Increment counter
        n += 1;
    }

    let mut first_row = format!(
        "{}{}",
        horizontal_chars.color(bright_recolor),
        bottom_half_block.color(recolor)
    );
    // If color panes are horizontal, add padding between color panes.
    if vertical {
        // Add margin if margin is requested.
        first_row = match margin {
            true => format!(" {} ", first_row),
            false => format!("{}", first_row),
        };
    } else {
        first_row = format!("{} ", first_row);
    };

    let mut middle_row = format!(
        "{}{}",
        horizontal_chars.color(bright_recolor),
        block.color(recolor)
    );
    // If color panes are horizontal, add padding between color panes.
    if vertical {
        // Add margin if margin is requested.
        middle_row = match margin {
            true => format!(" {} ", middle_row),
            false => format!("{}", middle_row),
        };
    } else {
        middle_row = format!("{} ", middle_row);
    };

    n = 0;
    let mut last_row = " ".to_string();
    // Loop while `n` is less than the color pane width
    while n < width {
        last_row = format!("{}{}", last_row, top_half_block.color(recolor));

        // Increment counter
        n += 1;
    }
    // If color panes are horizontal, add padding between color panes.
    if vertical {
        // Add margin if margin is requested.
        last_row = match margin {
            true => format!(" {} ", last_row),
            false => format!("{}", last_row),
        };
    } else {
        last_row = format!("{} ", last_row);
    };

    match row_type.as_str() {
        "first" => return first_row,
        "middle" => return middle_row,
        "last" => return last_row,
        _ => panic!("{} is not a valid row type.", row_type),
    };
}

// Return a row of all requested color panes.
fn colorpanes_row(
    vertical: bool,
    row_type: String,
    width: u32,
    margin: bool,
    colors: Vec<String>,
) -> String {
    match row_type.as_str() {
        "first" => {
            let row = colorpanes_row_type(vertical, "first".to_string(), width, margin, colors);
            return row;
        }
        "middle" => {
            let row = colorpanes_row_type(vertical, "middle".to_string(), width, margin, colors);
            return row;
        }
        "last" => {
            let row = colorpanes_row_type(vertical, "last".to_string(), width, margin, colors);
            return row;
        }
        _ => panic!("{} is not a row type.", row_type),
    };
}

fn colorpanes_row_type(
    vertical: bool,
    row_type: String,
    width: u32,
    margin: bool,
    colors: Vec<String>,
) -> String {
    let mut row = String::new();
    for color in colors {
        let bright_recolor = bright_recolor(color.clone());
        let recolor = recolor(color.clone());

        write!(
            row,
            "{}",
            colorpane_row(
                vertical,
                row_type.clone(),
                width,
                margin,
                bright_recolor,
                recolor
            )
        )
        .unwrap();
    }
    // Add margin if margin is requested.
    row = match margin {
        true => format!(" {} ", row),
        false => format!("{}", row),
    };
    return row;
}

fn horizontal_colorpanes(
    vertical: bool,
    width: u32,
    height: u32,
    captions: bool,
    margin: bool,
    colors: Vec<String>,
) -> String {
    let mut captions_row = String::new();

    for color in &colors {
        let number_of_caption_spaces =
            i64::from(width) + 2 - i64::try_from(color.chars().count()).unwrap();
        let mut n = 0;
        let mut captions_spaces = String::new();
        while n < number_of_caption_spaces {
            write!(captions_spaces, "{}", " ").unwrap();

            // Increment counter
            n += 1;
        }

        write!(captions_row, "{}{}", color, captions_spaces).unwrap();
    }

    // Add margin if margin is requested.
    captions_row = match margin {
        true => format!(" {} ", captions_row),
        false => format!("{}", captions_row),
    };

    let first_row = colorpanes_row(vertical, "first".to_string(), width, margin, colors.clone());

    let middle_row = colorpanes_row(
        vertical,
        "middle".to_string(),
        width,
        margin,
        colors.clone(),
    );

    let last_row = colorpanes_row(vertical, "last".to_string(), width, margin, colors.clone());

    // A counter variable
    let mut n = 0;
    let mut middle_rows = String::new();
    while n < height {
        write!(middle_rows, "{}\n", middle_row).unwrap();

        // Increment counter
        n += 1;
    }

    // Put rows together and add caption row if captions were requested.
    let mut horizontal_colorpanes = match captions {
        true => format!(
            "{}\n{}\n{}{}\n",
            captions_row, first_row, middle_rows, last_row
        ),
        false => format!("{}\n{}{}\n", first_row, middle_rows, last_row),
    };

    horizontal_colorpanes = top_and_bottom_margin(horizontal_colorpanes, margin);

    return horizontal_colorpanes;
}

// Return colorpane string in requested color.
fn colorpane(width: u32, height: u32, captions: bool, margin: bool, color: String) -> String {
    let bright_recolor = bright_recolor(color.clone());
    let recolor = recolor(color.clone());

    let first_row = colorpane_row(
        true,
        "first".to_string(),
        width,
        margin,
        bright_recolor,
        recolor,
    );
    let middle_row = colorpane_row(
        true,
        "middle".to_string(),
        width,
        margin,
        bright_recolor,
        recolor,
    );
    let last_row = colorpane_row(
        true,
        "last".to_string(),
        width,
        margin,
        bright_recolor,
        recolor,
    );

    // A counter variable
    let mut n = 0;
    let mut middle_rows = String::new();
    while n < height {
        write!(middle_rows, "{}\n", middle_row).unwrap();

        // Increment counter
        n += 1;
    }

    let mut colorpane = String::new();

    if margin {
        if captions {
            write!(colorpane, " {} \n", color).unwrap();
        };
    } else {
        if captions {
            write!(colorpane, "{}\n", color).unwrap();
        };
    };

    colorpane = format!("{}{}\n{}{}", colorpane, first_row, middle_rows, last_row);

    return colorpane;
}

fn vertical_colorpanes(
    width: u32,
    height: u32,
    captions: bool,
    margin: bool,
    colors: Vec<String>,
) -> String {
    // Print color pane for every selected color.
    let mut vertical_colorpanes = String::new();
    for color in colors {
        write!(
            vertical_colorpanes,
            "{}\n",
            colorpane(width, height, captions, margin, color)
        )
        .unwrap();
    }

    vertical_colorpanes = top_and_bottom_margin(vertical_colorpanes, margin);

    return vertical_colorpanes;
}

// Add top and bottom margin if margin was requested.
fn top_and_bottom_margin(mut content: String, margin: bool) -> String {
    if margin {
        content = format!("\n{}\n", content);
    };

    return content;
}
