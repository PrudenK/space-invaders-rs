use crate::board::cell::Cell;
use crate::board::game_state::{GameState, HEIGHT, WIDTH};
use crate::utils::constants::ERROR_NUMBER;

pub fn is_cell_active<F>(game: &GameState, pred: F) -> bool where F: Fn(&Cell) -> bool,{
    for i in 0..HEIGHT {
        for j in 0..WIDTH {
            if pred(&game.board[i][j]) {
                return true;
            }
        }
    }
    false
}

pub fn get_cell_coords(game: &GameState, cell_type: Cell) -> (usize, usize) {
    for i in 0..HEIGHT {
        for j in 0..WIDTH {
            if game.board[i][j] == cell_type {
                return (i , j);
            }
        }
    }

    (ERROR_NUMBER, ERROR_NUMBER)
}