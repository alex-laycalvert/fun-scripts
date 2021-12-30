use std::env;
use crossterm::{terminal::size, Result};
use std::process::exit;
use chrono::prelude::*;

use newyears::*;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
        exit(1);
    });

    let (cols, rows) = size()?;
    let top_row = 3;
    let ctr_col = cols / 2;
    let ctr_row = rows / 2;

    let start_row = config.radius + config.circle_thickness;
    let mut curr_row = start_row + top_row;




    

    while curr_row + config.radius + config.circle_thickness < rows {
        let utc_time = Utc::now();
        let local_time = Local::now();
        let timezone = local_time.offset();
        let local_dt = utc_time.with_timezone(timezone);
        let target_year = local_dt.year() + 1;
        let target_utc = Local.ymd(target_year, 1, 1).and_hms(0, 0, 0);
        let curr_year = local_dt.year();
        let curr_month = local_dt.month();
        let curr_day = local_dt.day();
        let curr_hour = local_dt.hour();
        let curr_min = local_dt.minute();
        let curr_sec = local_dt.second();
        let message = time_to_string(curr_hour, curr_min, curr_sec);
        clear_all()?;
        for i in 0..config.bar_thickness {
            draw_ver_line((ctr_col - config.bar_thickness / 2) + i, top_row, rows, config.bar_char, config.bar_color)?;
        }
        draw_circle(ctr_col, curr_row, config.radius, config.circle_char, config.color_code, config.circle_thickness)?;
        // draw_hor_line(ctr_col - config.radius*2 + 1, ctr_col + config.radius*2, curr_row, '=', config.color_code)?;
        draw_hor_line(ctr_col - config.radius*2 + 1, ctr_col + config.radius*2, curr_row - config.radius / 2, '=', config.color_code)?;
        draw_hor_line(ctr_col - config.radius*2 + 1, ctr_col + config.radius*2, curr_row + config.radius / 2, '=', config.color_code)?;
        draw_message(ctr_col - message.len() as u16 / 2, curr_row, &message, config.time_color)?;
        update()?;
        std::thread::sleep(std::time::Duration::from_millis(500));
        curr_row += 1;
    }


    clear_all()?;


    Ok(())
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
