use std::thread;
use std::time::{Duration, Instant};
use crate::board::game_state::GameState;
use crate::aliens::alien_movement::alien_movement;
use crate::board::cell::Cell;
use crate::player::player_controls::{get_bullet_coords, is_a_bullet_active, player_controls};
use crate::utils;

pub fn game_loop(game: &mut GameState) {
    utils::terminal::clear_terminal();
    game.print_format_board();

    let mut enemy_dir: i8 = 1;
    let mut last_enemy_move = Instant::now();
    let mut last_bullet_move = Instant::now();

    loop {
        let end_game = player_controls(game);

        if end_game { break }

        if last_enemy_move.elapsed() >= Duration::from_millis(600) {
            let went_down = alien_movement(game, enemy_dir);
            if went_down {
                enemy_dir *= -1;
            }
            last_enemy_move = Instant::now();
        }

        if is_a_bullet_active(game) {
            if last_bullet_move.elapsed() >= Duration::from_millis(30) {
                let (i_index, j_index) = get_bullet_coords(game);

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

        utils::terminal::clear_terminal();
        game.print_format_board();

        thread::sleep(Duration::from_millis(30));
    }
}