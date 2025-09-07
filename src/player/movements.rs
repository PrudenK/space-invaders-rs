use crate::board::cell::Cell;
use crate::board::game_state::{GameState, HEIGHT, WIDTH};
use crate::utils::constants::{DIR_LEFT, DIR_RIGHT};

pub fn move_player_horizontally(game: &mut GameState, direction: i8) {
    if let Some(j_player_index) = game.board[HEIGHT -2].iter().position(|&c| c == Cell::Player) {
        if direction == DIR_RIGHT{
            if j_player_index + DIR_RIGHT as usize != WIDTH -1{
                game.board[HEIGHT -2][j_player_index] = Cell::Empty;
                game.board[HEIGHT -2][j_player_index + DIR_RIGHT as usize] = Cell::Player;
            }
        }else{
            if ((j_player_index as isize) + DIR_LEFT as isize) as usize != 0{
                game.board[HEIGHT -2][j_player_index] = Cell::Empty;
                game.board[HEIGHT -2][((j_player_index as isize) + DIR_LEFT as isize) as usize] = Cell::Player;
            }
        }
    }
}