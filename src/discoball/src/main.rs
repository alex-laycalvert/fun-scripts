use std::io::{Write, stdout};
use crossterm::{
    ExecutableCommand, QueueableCommand,
    terminal::{Clear, ClearType, size}, 
    cursor, style::{self, Stylize}, Result
};

fn main() -> Result<()> {
    let mut stdout = stdout();
    clear(&mut stdout)?;

    let (cols, rows) = size()?;
    let ctr_col = cols / 2;
    let ctr_row = rows / 2;
    let radius = 20;

    stdout.queue(cursor::MoveTo(0, 0))?;
    println!("rows: {} cols: {}", rows, cols);

    let it = radius / 2;
    for i in (0..it).rev() {
        clear(&mut stdout)?;
        // draw discoball
        draw_circle(&mut stdout, ctr_col, ctr_row, radius, 'o', 1)?;

        // draw lines
        draw_horizontal_line_chars(&mut stdout, ctr_col - radius*2 + 1, ctr_col + radius*2, ctr_row, &['~', ' ', '~', '='])?;

        if i >= it / 2 {
            draw_vertical_curve_right(&mut stdout, ctr_col + (i - it / 2)*2, ctr_row - radius + 1, ctr_row + radius, '|', i - it / 2)?;
        } else {
            draw_vertical_curve_left(&mut stdout, ctr_col - (it / 2 + i)*2, ctr_row - radius + 1, ctr_row + radius, '|', it / 2 - i)?;
        }
        std::thread::sleep(std::time::Duration::from_millis(500));
    }

    
    stdout.execute(cursor::MoveTo(0, 0))?;
    clear(&mut stdout)?;
    println!();
    Ok(())
}

fn clear(out: &mut std::io::Stdout) -> Result<()> {
    out.execute(Clear(ClearType::All))?;
    Ok(())
}

fn draw_vertical_curve_right(stdout: &mut std::io::Stdout, col: u16, start_row: u16, end_row: u16, character: char, curve_radius: u16) -> Result<()> {
    let height = end_row - start_row;
    let factor = height / ((curve_radius + 1) * 2);
    let mut curr_col = col;
    for i in start_row..end_row {
        stdout.queue(cursor::MoveTo(curr_col, i))?;
        print!("{}", character);
        if (i - start_row + 1) % factor == 0 && i + 1 < start_row + height / 2 {
            curr_col += 1;
        } else if (i - start_row) % factor == 0 && i > (start_row + height / 2) + 1 {
            curr_col -= 1;
        }
    }
    stdout.flush()?;
    Ok(())
}

fn draw_vertical_curve_left(stdout: &mut std::io::Stdout, col: u16, start_row: u16, end_row: u16, character: char, curve_radius: u16) -> Result<()> {
    let height = end_row - start_row;
    let factor = height / ((curve_radius + 1) * 2);
    let mut curr_col = col;
    for i in start_row..end_row {
        stdout.queue(cursor::MoveTo(curr_col, i))?;
        print!("{}", character);
        if (i - start_row + 1) % factor == 0 && i + 1 < start_row + height / 2 {
            curr_col -= 1;
        } else if (i - start_row) % factor == 0 && i > (start_row + height / 2) + 1 {
            curr_col += 1;
        }
    }
    stdout.flush()?;
    Ok(())
}

fn draw_horizontal_line(stdout: &mut std::io::Stdout, start_col: u16, end_col: u16, row: u16, character: char) -> Result<()> {
    stdout.queue(cursor::MoveTo(start_col, row))?;
    for _i in start_col..end_col {
        print!("{}", character);
    }
    stdout.flush()?;
    Ok(())
}

fn draw_horizontal_line_chars(stdout: &mut std::io::Stdout, start_col: u16, end_col: u16, row: u16, characters: &[char]) -> Result<()> {
    stdout.queue(cursor::MoveTo(start_col, row))?;
    let length = characters.len();
    for i in start_col..end_col {
        print!("{}", characters[i as usize % length]);
    }
    stdout.flush()?;
    Ok(())
}

fn draw_circle(stdout: &mut std::io::Stdout, origin_x: u16, origin_y: u16, radius: u16, character: char, thickness: u16) -> Result<()> {
    if thickness < 1 {
        return Ok(());
    }
    for it in 0..thickness {
        let r = radius + it;
        for i in (origin_y - r)..(origin_y + r + 1) {
            for j in ((origin_x - r*2)..(origin_x + r*2 + 1)).step_by(2) {
                let x: i32 = j as i32 - origin_x as i32;
                let y: i32 = i as i32 - origin_y as i32;
                if on_circle((x / 2) as f32, y as f32, r) {
                    stdout.queue(cursor::MoveTo(j, i))?;
                    print!("{}", character);
                }
            }
        }
    }
    stdout.flush()?;
    Ok(())
}

fn on_circle(x: f32, y: f32, r: u16) -> bool {
    (x*x + y*y).sqrt() as i32 == r as i32
}
