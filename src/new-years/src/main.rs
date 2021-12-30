use std::env;
use crossterm::{terminal::size, Result};
use std::process::exit;
use chrono::prelude::*;
use figlet_rs::FIGfont;

use newyears::*;

const TARGET_HOUR: u32 = 23;
const TARGET_MIN: u32 = 59;
const TARGET_SEC: u32 = 59;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
        exit(1);
    });
    let current_year = Local::now().year();
    if Local::today() < Local.ymd(current_year, 12, 31) && !&config.now {
        println!("Come back on December 31 to see the ball drop");
        exit(0);
    }

    let message = &config.message;
    draw_loop(&config)?;
    clear_all()?;

    loop {
        draw_fig_msg(size()?.0 / 2, 5, String::from((current_year + 1).to_string()), config.message_color)?;
        draw_fig_msg(size()?.0 / 2, size()?.1 / 2, message.to_string(), config.message_color)?;
        std::thread::sleep(std::time::Duration::from_millis(500));
    }
}

fn draw_fig_msg(start_col: u16, start_row: u16, message: String, color: u8) -> Result<()> {
    let std_font = FIGfont::standand().unwrap();
    let fig_msg = std_font.convert(&message.to_string());
    if !fig_msg.is_some() {
        exit(1);
    }
    let mut curr_col;
    let mut curr_row;
    let mut offset = 0;
    let fig_msg = fig_msg.unwrap();

    let mut total_length = 0;
    for &fig_char in &fig_msg.characters {
        total_length += fig_char.width;
    }
    let total_height = fig_msg.height;

    for &fig_char in &fig_msg.characters {
        for i in 0..fig_char.characters.len() {
            for (j, &c) in fig_char.characters[i].as_bytes().iter().enumerate() {
                curr_col = j + offset;
                curr_row = i;
                draw_char(start_col + curr_col as u16 - total_length as u16 / 2, start_row + curr_row as u16 - total_height as u16 / 2, color_char(c as char, color))?;
            }
        }
        offset += fig_char.width as usize;
    }
    move_cursor(0, 0)?;
    update()?;
    Ok(())
}

fn seconds(h: u32, m: u32, s: u32) -> u32 {
    h * 3600 + m * 60 + s
}

fn hours(s: u32) -> u32 {
    s / 3600
}

fn mins(s: u32) -> u32 {
    s % 3600 / 60
}

fn secs(s: u32) -> u32 {
    s % 60
}

fn time_to_string(h: u32, m: u32, s: u32) -> String {
    let mut h_str = String::new();
    let mut m_str = String::new();
    let mut s_str = String::new();
    if h < 10 {
        h_str.push('0');
    }
    if m < 10 {
        m_str.push('0');
    }
    if s < 10 {
        s_str.push('0');
    }
    h_str.push_str(&h.to_string());
    m_str.push_str(&m.to_string());
    s_str.push_str(&s.to_string());
    h_str + ":" + &m_str + ":" + &s_str
}

fn draw_loop(config: &Config) -> Result<()> {
    let (cols, rows) = size()?;
    let top_row = 3;
    let ctr_col = cols / 2;

    let bar_len = rows - top_row - 1;
    let move_len = bar_len - 2*config.radius - 2*config.circle_thickness;

    let now = Local::now();
    let this_year = now.year();

    let init_time = Local::now().time();
    let init_hour = init_time.hour();
    let init_min = init_time.minute();
    let init_sec = init_time.second();
    let is = seconds(init_hour, init_min, init_sec);

    let ts = seconds(TARGET_HOUR, TARGET_MIN, TARGET_SEC);
    let init_target_diff = ts as i32 - is as i32;

    if init_target_diff < 0 {
        return Ok(());
    }

    let start_row = config.radius + config.circle_thickness + top_row;
    let mut curr_row = start_row;

    loop {
        let curr_time = Local::now().time();
        let curr_hour = curr_time.hour();
        let curr_min = curr_time.minute();
        let curr_sec = curr_time.second();
        let cs = seconds(curr_hour, curr_min, curr_sec);
        let ds: i32 = ts as i32 - cs as i32;
        if ds == 0 {
            break;
        }
        let ds = ds as u32;
        let h = hours(ds);
        let m = mins(ds);
        let s = secs(ds);
        let message = time_to_string(h, m, s);
        clear_all()?;
        let cd_message = format!("COUNTDOWN TO {}", this_year + 1);
        draw_message(ctr_col - cd_message.len() as u16 / 2, 0, &cd_message, config.color_code)?;
        for i in 0..config.bar_thickness {
            draw_ver_line((ctr_col - config.bar_thickness / 2) + i, top_row, rows, config.bar_char, config.bar_color)?;
        }
        draw_circle(ctr_col, curr_row, config.radius, config.circle_char, config.color_code, config.circle_thickness)?;
        // draw_hor_line(ctr_col - config.radius*2 + 1, ctr_col + config.radius*2, curr_row, '=', config.color_code)?;
        draw_hor_line(ctr_col - config.radius*2 + 1, ctr_col + config.radius*2, curr_row - config.radius / 2, '=', config.color_code)?;
        draw_hor_line(ctr_col - config.radius*2 + 1, ctr_col + config.radius*2, curr_row + config.radius / 2, '=', config.color_code)?;
        draw_message(ctr_col - message.len() as u16 / 2, curr_row, &message, config.time_color)?;
        move_cursor(0, 0)?;
        update()?;
        if ds <= move_len as u32 {
            curr_row += 1;
        }
        std::thread::sleep(std::time::Duration::from_millis(1000));
    } 
    Ok(())
}

