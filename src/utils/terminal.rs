use crossterm::{execute, terminal::{Clear, ClearType}, cursor::MoveTo};
use std::io::stdout;

pub fn clear_terminal(){
    execute!(stdout(), MoveTo(0,0), Clear(ClearType::All)).unwrap();
}