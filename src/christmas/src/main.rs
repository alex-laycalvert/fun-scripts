use std::io::{Write, Stdout, stdout};
use crossterm::{
    ExecutableCommand, QueueableCommand,
    cursor, Result,
    terminal::{Clear, ClearType, size},
    style::{self, Stylize}
};

// asdf

fn main() -> Result<()> {
    let mut stdout = stdout();

    clear_all(&mut stdout)?;

    let (cols, rows) = size()?;
    let ctr_col = cols / 2;
    let ctr_row = rows / 2;

    println!("cols: {} rows: {}", cols, rows);


    let tree_height = 10;
    let tree_width = 10;

    let start_row = ctr_row - tree_height / 2;
    let end_row = ctr_row + tree_height / 2;

    for i in start_row..end_row {
        let string = char_string('*', (i - start_row) as usize);
        stdout.queue(cursor::MoveTo(ctr_col - (string.len() / 2) as u16, i))?;
        print!("{}", char_string('*', (i - start_row) as usize));
    }




    stdout.flush()?;

    std::thread::sleep(std::time::Duration::from_millis(1500));

    clear_all(&mut stdout)?;

    Ok(())
}

fn char_string(character: char, amount: usize) -> String {
    let mut s = String::new();
    for _i in 0..amount {
        s.push(character);
    }
    s
}

fn clear_all(stdout: &mut Stdout) -> Result<()> {
    stdout.execute(Clear(ClearType::All))?;
    stdout.execute(cursor::MoveTo(0, 0))?;
    Ok(())
}
