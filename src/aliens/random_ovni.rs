use std::time::{Duration, Instant};
use rand::{Rng};
use crate::board::cell::Cell;
use crate::board::game_state::{GameState, WIDTH};
use crate::utils::board_utils::{get_cell_coords, is_cell_active, ERROR_NUMBER};

const OVNI_COOLDOWN: u64 = 10000;


pub fn spwan_random_ovni(game: &mut GameState){
    if !is_cell_active(game, |c| *c == Cell::RandomOvni) && game.last_random_ovni_spawned.elapsed() >= Duration::from_millis(OVNI_COOLDOWN){
        let mut rng = rand::thread_rng();
        game.last_random_ovni_spawned = Instant::now();

        if rng.gen_range(0..3) == 1{
            let direction = if rng.gen_bool(0.5) { 1 } else { -1 };

            game.random_ovni_dir = direction;

            if direction == 1{
                game.board[1][1] = Cell::RandomOvni
            }else{
                game.board[1][WIDTH-2] = Cell::RandomOvni
            }
        }
    }
}

pub fn manage_random_ovni_on_loop(game: &mut GameState){
    if is_cell_active(game, |c| *c == Cell::RandomOvni){
            if game.last_random_ovni_moved.elapsed() >= Duration::from_millis(120){
            let (i_index, j_index) = get_cell_coords(game, Cell::RandomOvni);

            if i_index != ERROR_NUMBER && j_index != ERROR_NUMBER{
                game.last_random_ovni_moved = Instant::now();

                game.board[i_index][j_index] = Cell::Empty;

                match game.board[i_index][(j_index as isize + game.random_ovni_dir as isize) as usize] {
                    Cell::Border => {},
                    Cell::Empty => {
                        game.board[i_index][(j_index as isize + game.random_ovni_dir as isize) as usize] = Cell::RandomOvni;
                    },
                    Cell::Bullet => {
                        game.board[i_index][(j_index as isize + game.random_ovni_dir as isize) as usize] = Cell::Empty;
                        game.score += 1500
                    }
                    _ =>{}
                }
            }
        }
    }
}