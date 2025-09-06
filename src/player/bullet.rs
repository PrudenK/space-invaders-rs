use std::time::{Duration, Instant};
use crate::board::cell::Cell;
use crate::board::game_state::{GameState, HEIGHT, WIDTH};

const BULLET_COOLDOWN: u64 = 500;
const ERROR_NUMBER: usize = 999;


pub fn shot_bullet(game: &mut GameState) {
    if !is_a_bullet_active(game) && game.last_bullet_shooted.elapsed() >= Duration::from_millis(BULLET_COOLDOWN) {
        game.last_bullet_shooted = Instant::now();

        if let Some(j_player_index) = game.board[HEIGHT -2].iter().position(|&c| c == Cell::Player) {
            game.board[HEIGHT-3][j_player_index] = Cell::Bullet;
        }
    }
}

pub fn manage_bullet_on_loop(game: &mut GameState) {
    if is_a_bullet_active(game) {
        if game.last_bullet_move.elapsed() >= Duration::from_millis(30) {
            let (i_index, j_index) = get_bullet_coords(game);

            game.last_bullet_move = Instant::now();

            if i_index != ERROR_NUMBER && j_index != ERROR_NUMBER {
                game.board[i_index][j_index] = Cell::Empty;

                match game.board[i_index -1][j_index] {
                    Cell::Border => {},
                    Cell::Alien => {
                        game.board[i_index - 1][j_index] = Cell::Empty
                    },
                    _ => game.board[i_index - 1][j_index] = Cell::Bullet
                }
            }
        }
    }
}


fn is_a_bullet_active(game: &GameState) -> bool {
    for i in 0..HEIGHT {
        for j in 0..WIDTH {
            if game.board[i][j] == Cell::Bullet{
                return true;
            }
        }
    }

    false
}

fn get_bullet_coords(game: &GameState) -> (usize, usize) {
    for i in 0..HEIGHT {
        for j in 0..WIDTH {
            if game.board[i][j] == Cell::Bullet{
                return (i , j);
            }
        }
    }

    (ERROR_NUMBER, ERROR_NUMBER)
}