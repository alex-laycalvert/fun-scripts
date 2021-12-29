use std::process::exit;
use std::io::{Write, stdout};
use crossterm::{
    ExecutableCommand, QueueableCommand,
    terminal::{Clear, ClearType},
    style::{Stylize, StyledContent},
    cursor
};
use rand::Rng;

pub const MAX_RADIUS: u16 = 10;
pub const MIN_RADIUS: u16 = 3;
pub const MAX_COLOR_CODE: u8 = 7;
pub const MIN_COLOR_CODE: u8 = 0;
pub const MAX_CIRCLE_THICKNESS: u16 = 3;
pub const MIN_CIRCLE_THICKNESS: u16 = 1;
pub const MAX_BAR_THICKNESS: u16 = 5;
pub const MIN_BAR_THICKNESS: u16 = 1;
pub const RANDOM_COLOR: u8 = 7;
pub const DEFAULT_RADIUS: u16 = 5;
pub const DEFAULT_COLOR: u8 = 7;
pub const DEFAULT_CIRCLE_CHAR: char = '*';
pub const DEFAULT_CIRCLE_THICKNESS: u16 = 1;
pub const DEFAULT_BAR_COLOR: u8 = 4;
pub const DEFAULT_BAR_CHAR: char = '|';
pub const DEFAULT_BAR_THICKNESS: u16 = 3;
pub const DEFAULT_MESSAGE: &str = "Happy New Year";

pub const HELP: &str = "--help";
pub const HELP_SHORT: &str = "-h";
pub const RADIUS: &str = "--radius";
pub const RADIUS_SHORT: &str = "-r";
pub const COLOR: &str = "--color";
pub const COLOR_SHORT: &str = "-c";
pub const CIRCLE_CHAR: &str = "--char";
pub const CIRCLE_CHAR_SHORT: &str = "-C";
pub const CIRCLE_THICKNESS: &str = "--circle-thickness";
pub const CIRCLE_THICKNESS_SHORT: &str = "-t";
pub const BAR_COLOR: &str = "--bar-color";
pub const BAR_COLOR_SHORT: &str = "-b";
pub const BAR_CHAR: &str = "--bar-char";
pub const BAR_CHAR_SHORT: &str = "-B";
pub const BAR_THICKNESS: &str = "--bar-thickness";
pub const BAR_THICKNESS_SHORT: &str = "-T";
pub const MESSAGE: &str = "--message";
pub const MESSAGE_SHORT: &str = "-m";

pub struct Config {
    pub radius: u16,
    pub color_code: u8,
    pub circle_char: char,
    pub circle_thickness: u16,
    pub bar_color: u8,
    pub bar_char: char,
    pub bar_thickness: u16,
    pub message: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {

        let mut radius = DEFAULT_RADIUS;
        let mut color_code = DEFAULT_COLOR;
        let mut circle_char = DEFAULT_CIRCLE_CHAR;
        let mut circle_thickness = DEFAULT_CIRCLE_THICKNESS;
        let mut bar_color = DEFAULT_BAR_COLOR;
        let mut bar_char = DEFAULT_BAR_CHAR;
        let mut bar_thickness = DEFAULT_BAR_THICKNESS;
        let mut message = DEFAULT_MESSAGE;

        for i in 0..args.len() {
            if args[i] == HELP || args[i] == HELP_SHORT {
                Config::print_usage();
                exit(0);
            } else if args[i] == RADIUS || args[i] == RADIUS_SHORT {
                if i + 1 == args.len() {
                    return Err("No value specified for radius");
                }
                let value = &args[i + 1];
                let value: u16 = match value.trim().parse() {
                    Ok(v) => v,
                    Err(_) => {
                        Config::print_usage();
                        eprintln!("Unable to process value for radius: {}", value);
                        exit(1);
                    },
                };
                if value < MIN_RADIUS {
                    radius = MIN_RADIUS;
                } else if value > MAX_RADIUS {
                    radius = MAX_RADIUS;
                } else {
                    radius = value;
                }
                    
            } else if args[i] == COLOR || args[i] == COLOR_SHORT {
                if i + 1 == args.len() {
                    return Err("No value specified for circle color");
                }
                let value = &args[i + 1];
                let value: u8 = match value.trim().parse() {
                    Ok(v) => v,
                    Err(_) => {
                        Config::print_usage();
                        eprintln!("Unable to process value for color: {}", value);
                        exit(1);
                    },
                };
                if value < MIN_COLOR_CODE {
                    color_code = MIN_COLOR_CODE;
                } else if value > MAX_COLOR_CODE {
                    color_code = MAX_COLOR_CODE;
                } else {
                    color_code = value;
                }
            } else if args[i] == CIRCLE_CHAR || args[i] == CIRCLE_CHAR_SHORT {
                if i + 1 == args.len() {
                    return Err("No value specified for circle chararacter");
                }
                let value = &args[i + 1];
                let value: char = match value.trim().parse() {
                    Ok(v) => v,
                    Err(_) => {
                        Config::print_usage();
                        eprintln!("Unable to process value for circle character: {}", value);
                        exit(1);
                    },
                };
                circle_char = value;
            } else if args[i] == CIRCLE_THICKNESS || args[i] == CIRCLE_THICKNESS_SHORT {
                if i + 1 == args.len() {
                    return Err("No value specified for circle thickness");
                }
                let value = &args[i + 1];
                let value: u16 = match value.trim().parse() {
                    Ok(v) => v,
                    Err(_) => {
                        Config::print_usage();
                        eprintln!("Unable to process value for circle thickness: {}", value);
                        exit(1);
                    },
                };
                if value < MIN_CIRCLE_THICKNESS {
                    circle_thickness = MIN_CIRCLE_THICKNESS;
                } else if value > MAX_CIRCLE_THICKNESS {
                    circle_thickness = MAX_CIRCLE_THICKNESS;
                } else {
                    circle_thickness = value;
                }
            } else if args[i] == BAR_COLOR || args[i] == BAR_COLOR_SHORT {
                if i + 1 == args.len() {
                    return Err("No value specified for bar color");
                }
                let value = &args[i + 1];
                let value: u8 = match value.trim().parse() {
                    Ok(v) => v,
                    Err(_) => {
                        Config::print_usage();
                        eprintln!("Unable to process value for bar color: {}", value);
                        exit(1);
                    },
                };
                if value < MIN_COLOR_CODE {
                    bar_color = MIN_COLOR_CODE;
                } else if value > MAX_COLOR_CODE {
                    bar_color = MAX_COLOR_CODE;
                } else {
                    bar_color = value;
                }
            } else if args[i] == BAR_CHAR || args[i] == BAR_CHAR_SHORT {
                if i + 1 == args.len() {
                    return Err("No value specified for bar character");
                }
                let value = &args[i + 1];
                let value: char = match value.trim().parse() {
                    Ok(v) => v,
                    Err(_) => {
                        Config::print_usage();
                        eprintln!("Unable to process value for bar character: {}", value);
                        exit(1);
                    },
                };
                bar_char = value;
            } else if args[i] == BAR_THICKNESS || args[i] == BAR_THICKNESS_SHORT {
                if i + 1 == args.len() {
                    return Err("No value specified for bar thickness");
                }
                let value = &args[i + 1];
                let value: u16 = match value.trim().parse() {
                    Ok(v) => v,
                    Err(_) => {
                        Config::print_usage();
                        eprintln!("Unable to process value for bar thickness: {}", value);
                        exit(1);
                    },
                };
                if value < MIN_BAR_THICKNESS {
                    bar_thickness = MIN_BAR_THICKNESS;
                } else if value > MAX_BAR_THICKNESS {
                    bar_thickness = MAX_BAR_THICKNESS;
                } else {
                    bar_thickness = value;
                }
            } else if args[i] == MESSAGE || args[i] == MESSAGE_SHORT {
                if i + 1 == args.len() {
                    return Err("No value specified for message");
                }
                message = &args[i + 1];
            }
        }
        let message = String::from(message);
        Ok( Config { radius, color_code, circle_char, circle_thickness, bar_color, bar_char, bar_thickness, message } )
    }


    pub fn print_usage() {
        println!("USAGE: ");
    }
}

pub fn clear_all() -> crossterm::Result<()> {
    stdout().execute(Clear(ClearType::All))?;
    stdout().execute(cursor::MoveTo(0, 0))?;
    Ok(())
}

pub fn update() -> crossterm::Result<()> {
    stdout().flush()?;
    Ok(())
}

pub fn draw_char(col: u16, row: u16, color_char: StyledContent<char>) -> crossterm::Result<()> {
    stdout().queue(cursor::MoveTo(col, row))?;
    print!("{}", color_char);
    Ok(())
}

pub fn draw_hor_line(start_col: u16, end_col: u16, row: u16, character: char, color: u8) -> crossterm::Result<()> {
    for i in start_col..end_col {
        draw_char(i, row, color_char(character, color))?;
    }
    Ok(())
}

pub fn draw_ver_line(col: u16, start_row: u16, end_row: u16, character: char, color: u8) -> crossterm::Result<()> {
    for i in start_row..end_row {
        draw_char(col, i, color_char(character, color))?;
    }
    Ok(())
}

pub fn draw_message(col: u16, row: u16, message: &str, with_border: bool) -> crossterm::Result<()> {
    if !with_border {
        stdout().queue(cursor::MoveTo(col, row))?;
        print!("{}", message);
    } else {
        let msg_len = message.len() + 4;
        stdout().queue(cursor::MoveTo(col, row))?;
        for _i in 0..msg_len {
            print!("=");
        }
        stdout().queue(cursor::MoveTo(col, row + 1))?;
        print!("| ");
        print!("{:?}", message);
        print!(" |");
        stdout().queue(cursor::MoveTo(col, row + 2))?;
        for _i in 0..msg_len {
            print!("=");
        }
    }
    Ok(())
}

pub fn draw_circle(origin_col: u16, origin_row: u16, radius: u16, character: char, color: u8, thickness: u16) -> crossterm::Result<()> {
    if thickness < 1 {
        return Ok(());
    }
    for it in (0..thickness).rev() {
        let r = radius + it;
        for i in (origin_row - r)..(origin_row + r + 1) {
            for j in (origin_col - r*2)..(origin_col + r*2 + 1) {
                let x: i32 = j as i32 - origin_col as i32;
                let y: i32 = i as i32 - origin_row as i32;
                if within_circle((x / 2) as f32, y as f32, r) {
                    draw_char(j, i, color_char(' ', 0))?;
                }
                if on_circle((x / 2) as f32, y as f32, r) && j % 2 == 0 {
                    draw_char(j, i, color_char(character, color))?;
                }
            }
        }
    }
    Ok(())
}

pub fn on_circle(x: f32, y: f32, r: u16) -> bool {
    (x*x + y*y).sqrt() as i32 == r as i32
}

pub fn within_circle(x: f32, y: f32, r: u16) -> bool {
    (x*x + y*y).sqrt() as i32 <= r as i32
}

pub fn color_char(character: char, color_choice: u8) -> StyledContent<char> {
    let color_char: StyledContent<char>;
    if color_choice == 0 {
        color_char = character.red();
    } else if color_choice == 1 {
        color_char = character.yellow();
    } else if color_choice == 2 {
        color_char = character.green();
    } else if color_choice == 3 {
        color_char = character.blue();
    } else if color_choice == 4 {
        color_char = character.cyan();
    } else if color_choice == 5 {
        color_char = character.magenta();
    } else if color_choice == 6 {
        color_char = character.white();
    } else if color_choice == 7 {
        let random_choice = rand::thread_rng().gen::<u8>() % 7;
        if random_choice == 0 {
            color_char = character.red();
        } else if color_choice == 1 {
            color_char = character.yellow();
        } else if random_choice == 2 {
            color_char = character.green();
        } else if random_choice == 3 {
            color_char = character.blue();
        } else if random_choice == 4 {
            color_char = character.cyan();
        } else if random_choice == 5 {
            color_char = character.magenta();
        } else {
            color_char = character.white();
        } 
    } else {
        color_char = character.black();
    }
    color_char

}


