use board::game_state::GameState;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

mod board;
mod game;
mod utils;
mod aliens;
mod player;

fn main() {
    enable_raw_mode().unwrap();

    let mut game_state = GameState::new();

    game_state.set_up_new_game();
    game_state.print_format_board();



    game::game_loop(&mut game_state);




    disable_raw_mode().unwrap();
}
