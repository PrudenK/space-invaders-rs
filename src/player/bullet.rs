use std::time::{Duration, Instant};
use crate::board::cell::Cell;
use crate::board::game_state::{GameState, HEIGHT};
use crate::utils::board_utils::{get_cell_coords, is_cell_active};
use crate::utils::constants::{BULLET_COOLDOWN, BULLET_SPEED, DIR_LEFT, ERROR_NUMBER, MISSING_BULET_SCORE, OVNI_SCORE_VALUE};

pub fn shot_bullet(game: &mut GameState) {
    if !is_cell_active(game, |c| *c == Cell::Bullet) && game.last_bullet_shooted.elapsed() >= Duration::from_millis(BULLET_COOLDOWN) {
        game.last_bullet_shooted = Instant::now();

        if let Some(j_player_index) = game.board[HEIGHT -2].iter().position(|&c| c == Cell::Player) {
            game.board[HEIGHT-3][j_player_index] = Cell::Bullet;
        }
    }
}

pub fn manage_bullet_on_loop(game: &mut GameState) {
    if is_cell_active(game, |c| *c == Cell::Bullet) {
        if game.last_bullet_move.elapsed() >= Duration::from_millis(BULLET_SPEED) {
            let (i_index, j_index) = get_cell_coords(game, Cell::Bullet);

            if i_index != ERROR_NUMBER && j_index != ERROR_NUMBER {
                game.last_bullet_move = Instant::now();

                game.board[i_index][j_index] = Cell::Empty;

                match game.board[(i_index as isize + DIR_LEFT as isize) as usize][j_index] {
                    Cell::Border => {
                        game.score -= MISSING_BULET_SCORE;
                    },
                    Cell::RandomOvni => {
                        game.score += OVNI_SCORE_VALUE;
                        game.board[(i_index as isize + DIR_LEFT as isize) as usize][j_index] = Cell::Empty;
                    }
                    Cell::Alien(alien_type) => {
                        game.score += alien_type.score();
                        game.board[(i_index as isize + DIR_LEFT as isize) as usize][j_index] = Cell::Empty;
                    }
                    Cell::AlienBullet => {
                        game.board[(i_index as isize + DIR_LEFT as isize) as usize][j_index] = Cell::Empty;
                    },
                    Cell::Bridge(hp) if hp > 1 => {
                        game.board[(i_index as isize + DIR_LEFT as isize) as usize][j_index] = Cell::Bridge(hp - 1);
                    }
                    Cell::Bridge(_) => {
                        game.board[(i_index as isize + DIR_LEFT as isize) as usize][j_index] = Cell::Empty;
                    }
                    _ => game.board[(i_index as isize + DIR_LEFT as isize) as usize][j_index] = Cell::Bullet
                }
            }
        }
    }
}
