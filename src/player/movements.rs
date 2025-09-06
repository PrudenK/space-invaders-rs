use crate::board::cell::Cell;
use crate::board::game_state::{GameState, HEIGHT, WIDTH};

pub fn move_player_horizontally(game: &mut GameState, direction: i8) {
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