use crossterm::{event, execute, terminal::{Clear, ClearType}, cursor::MoveTo};
use std::io::stdout;
use crossterm::event::{Event, KeyCode};
use crate::board::cell::Cell;
use crate::board::game_state::{GameState, HEIGHT, WIDTH};

pub fn movement_loop(game: &mut GameState) {
    clear_terminal();
    game.print_format_board();

    loop {
        if let Event::Key(key_event) = event::read().unwrap() {
            match key_event.code {
                KeyCode::Left => {
                    clear_terminal();


                    side_move(game, -1);
                    game.print_format_board();
                }
                KeyCode::Right => {
                    clear_terminal();


                    side_move(game, 1);
                    game.print_format_board();
                }
                KeyCode::Esc => {
                    println!("¡Juego terminado!\n");
                    break
                },
                _ => {

                }
            }
        }
    }
}

fn clear_terminal(){
    execute!(stdout(), MoveTo(0,0), Clear(ClearType::All)).unwrap();
}

fn side_move(game: &mut GameState, direction: i8) {
    if let Some(j_player_index) = game.board[HEIGHT -2].iter().position(|&c| c == Cell::Player) {
        if direction == 1{
            if j_player_index + 1 != WIDTH -1{
                game.board[HEIGHT -2][j_player_index] = Cell::Empty;
                game.board[HEIGHT -2][j_player_index + 1] = Cell::Player;
            }
        }else{
            if j_player_index - 1 != 0{
                game.board[HEIGHT -2][j_player_index] = Cell::Empty;
                game.board[HEIGHT -2][j_player_index - 1] = Cell::Player;
            }
        }
    }
}
