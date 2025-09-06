use crossterm::{event, execute, terminal::{Clear, ClearType}, cursor::MoveTo};
use std::io::stdout;
use crossterm::event::{Event, KeyCode};
use crate::board::game_state::GameState;

pub fn movement_loop(game: &mut GameState) {
    execute!(stdout(), MoveTo(0,0), Clear(ClearType::All)).unwrap();
    game.print_format_board();

    loop {
        if let Event::Key(key_event) = event::read().unwrap() {
            match key_event.code {
                KeyCode::Left => {
                    // actualizar posición del jugador aquí

                    // limpiar terminal antes de repintar
                    execute!(stdout(), MoveTo(0,0), Clear(ClearType::All)).unwrap();
                    game.print_format_board();
                }
                KeyCode::Right => {
                    // actualizar posición del jugador aquí

                    execute!(stdout(), MoveTo(0,0), Clear(ClearType::All)).unwrap();
                    game.print_format_board();
                }
                KeyCode::Esc => {
                    println!("¡Juego terminado!");
                    break
                },
                _ => {

                }
            }
        }
    }
}
