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
    let ctr_col = cols / 2;
    let ctr_row = rows / 2;

    let start_row = config.radius + config.circle_thickness;
    let mut curr_row = start_row;

    while curr_row + config.radius + config.circle_thickness < rows {
        clear_all()?;
        let curr_hour = Local::now().hour();
        let curr_min = Local::now().minute();
        let curr_sec = Local::now().second();
        let message = curr_hour.to_string() + ":" + &curr_min.to_string() + ":" + &curr_sec.to_string();
        for i in 0..config.bar_thickness {
            draw_ver_line((ctr_col - config.bar_thickness / 2) + i, 0, rows, config.bar_char, config.bar_color)?;
        }
        draw_circle(ctr_col, curr_row, config.radius, config.circle_char, config.color_code, config.circle_thickness)?;
        draw_hor_line(ctr_col - config.radius*2 + 1, ctr_col + config.radius*2, curr_row, '=', config.color_code)?;
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
