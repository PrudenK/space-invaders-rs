use board::game_state::GameState;
use crossterm::terminal::{enable_raw_mode, disable_raw_mode};

mod board;
mod movement;

fn main() {
    enable_raw_mode().unwrap();

    let mut game_state = GameState::new();

    game_state.set_up_new_game();
    game_state.print_format_board();

    movement::movement_loop(&mut game_state);





    disable_raw_mode().unwrap();
}
