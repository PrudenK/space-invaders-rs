use std::time::{Duration, Instant};
use crate::board::cell::Cell;
use crate::board::game_state::{GameState, HEIGHT, WIDTH};

pub fn shot_bullet(game: &mut GameState) {
    if !is_a_bullet_active(game){
        if let Some(j_player_index) = game.board[HEIGHT -2].iter().position(|&c| c == Cell::Player) {
            game.board[HEIGHT-3][j_player_index] = Cell::Bullet;
        }
    }
}

pub fn manage_bullet_on_loop(game: &mut GameState, last_bullet_move: &mut Instant) {
    if is_a_bullet_active(game) {
        if last_bullet_move.elapsed() >= Duration::from_millis(30) {
            let (i_index, j_index) = get_bullet_coords(game);

            *last_bullet_move = Instant::now();

            if i_index != 999 && j_index != 999 {
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

    (999, 999)
}