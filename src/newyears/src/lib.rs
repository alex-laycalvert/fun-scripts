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
pub const DEFAULT_MSG_COLOR: u8 = 7;
pub const DEFAULT_TIME_COLOR: u8 = 4;
pub const DEFAULT_NOW: bool = false;

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
pub const MESSAGE_COLOR: &str = "--message-color";
pub const MESSAGE_COLOR_SHORT: &str = "-M";
pub const TIME_COLOR: &str = "--time-color";
pub const TIME_COLOR_SHORT: &str = "-i";
pub const NOW: &str = "--now";
pub const NOW_SHORT: &str = "-n";

pub struct Config {
    pub radius: u16,
    pub color_code: u8,
    pub circle_char: char,
    pub circle_thickness: u16,
    pub bar_color: u8,
    pub bar_char: char,
    pub bar_thickness: u16,
    pub message: String,
    pub time_color: u8,
    pub message_color: u8,
    pub now: bool,
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
        let mut time_color = DEFAULT_TIME_COLOR;
        let mut message_color = DEFAULT_MSG_COLOR;
        let mut now = DEFAULT_NOW;

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
            } else if args[i] == TIME_COLOR || args[i] == TIME_COLOR_SHORT {
                if i + 1 == args.len() {
                    return Err("No value specified for time color");
                }
                let value = &args[i + 1];
                let value: u8 = match value.trim().parse() {
                    Ok(v) => v,
                    Err(_) => {
                        Config::print_usage();
                        eprintln!("Unable to process value for time color: {}", value);
                        exit(1);
                    },
                };
                if value < MIN_COLOR_CODE {
                    time_color = MIN_COLOR_CODE;
                } else if value > MAX_COLOR_CODE {
                    time_color = MAX_COLOR_CODE;
                } else {
                    time_color = value;
                }
            } else if args[i] == MESSAGE_COLOR || args[i] == MESSAGE_COLOR_SHORT {
                if i + 1 == args.len() {
                    return Err("No value specified for message color");
                }
                let value = &args[i + 1];
                let value: u8 = match value.trim().parse() {
                    Ok(v) => v,
                    Err(_) => {
                        Config::print_usage();
                        eprintln!("Unable to process value for message color: {}", value);
                        exit(1);
                    },
                };
                if value < MIN_COLOR_CODE {
                    message_color = MIN_COLOR_CODE;
                } else if value > MAX_COLOR_CODE {
                    message_color = MAX_COLOR_CODE;
                } else {
                    message_color = value;
                }
            } else if args[i] == NOW || args[i] == NOW_SHORT {
                now = true;
            }

        }
        let message = String::from(message);
        Ok( Config { radius, color_code, circle_char, circle_thickness, bar_color, bar_char, bar_thickness, message, time_color, message_color, now } )
    }


    pub fn print_usage() {
        println!("newyears - A simple New Year's countdown written in Rust");
        println!();
        println!("usage: newyears [options]");
        println!();
        println!("description:");
        println!("      This is a simple counter to midnight that will, by default, only run on December 31st of the current year. You can force it to run on the current day but it will only count to midnight because I wrote this in one day before New Year's and didn't have time to learn more Rust");
        println!();
        println!("options:");
        println!("    -h, --help                        display this menu");
        println!("    -r <N>, --radius <N>              set radius of the New Year's ball to the integer N (default: 7, min: 3, max: 10)");
        println!("    -c <C>, --color <C>               set the color of the New Year's ball and countdown message to the color code C");
        println!("                                      see available color codes below");
        println!("    -t <N>, --circle-thickness <N>    set the thickness of the New' Years ball to the integer N (default: 1, min: 1, max: 3)");
        println!("    -b <C>, --bar-color <C>           set the color of the bar to the color code C");
        println!("    -B <char>, --bar-char <char>      set the character that the bar is comprised of (default: '|')");
        println!("                                      note: must be a single character");
        println!("    -T <N>, --bar-thickness <N>       set the thickness of the bar (default: 3, min: 1, max: 5)");
        println!("    -m <string>, --message <string>   set the message that will be displayed in big ASCII text at the end of the countdown");
        println!("    -M <C>, --message-color <C>       set the color of the message being displayed at the end to the color code C");
        println!("    -i <C>, --time-color <C>          set the color of the countdown on the New Year's ball to the color code C");
        println!("    -n, --now                         make the coutndown start immediately");
        println!("                                      this effectively just makes a countdown to midnight of the current day and does not do a countdown to New Year's of the current year");
        println!();
        println!("color codes:");
        println!("    0 - red");
        println!("    1 - yellow");
        println!("    2 - green");
        println!("    3 - blue");
        println!("    4 - cyan");
        println!("    5 - magenta");
        println!("    6 - white");
        println!("    7 - random");
        println!();
        println!("source: https://github.com/alex-laycalvert/fun-scripts");
        println!();
        println!("by: alex-laycalvert");
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

pub fn draw_message(col: u16, row: u16, message: &str, color: u8) -> crossterm::Result<()> {
    let mut col = col;
    let mut row = row;
    if col <= 1 {
        col = 2;
    }
    if row <= 1 {
        row = 1;
    }
    let msg_len = message.len() as u16;
    // top border
    for i in (col - 2)..(col + msg_len + 2) {
        draw_char(i, row - 1, color_char('=', color))?;
    }
    // message
    draw_char(col - 2, row, color_char('|', color))?;
    draw_char(col - 1, row, color_char(' ', color))?;
    for (i, &v) in message.as_bytes().iter().enumerate() {
        draw_char(col + i as u16, row, color_char(v as char, color))?;
    }
    draw_char(col + msg_len, row, color_char(' ', color))?;
    draw_char(col + msg_len + 1, row, color_char('|', color))?;
    // bottom border
    for i in (col - 2)..(col + msg_len + 2) {
        draw_char(i, row + 1, color_char('=', color))?;
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

pub fn move_cursor(col: u16, row: u16) -> crossterm::Result<()> {
    stdout().queue(cursor::MoveTo(col, row))?;
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


