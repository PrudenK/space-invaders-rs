use std::thread;
use std::time::{Duration, Instant};
use crate::board::game_state::GameState;
use crate::aliens::alien_movement::alien_movement;
use crate::player::bullet::manage_bullet_on_loop;
use crate::player::player_controls::{player_controls};
use crate::utils;

pub fn game_loop(game: &mut GameState) {
    utils::terminal::clear_terminal();
    game.print_format_board();

    let mut enemy_dir: i8 = 1;
    let mut last_enemy_move = Instant::now();
    let mut last_bullet_move = Instant::now();
    let mut last_bullet_shooted = Instant::now();

    loop {
        let end_game = player_controls(game, &mut last_bullet_shooted);

        if end_game { break }

        if last_enemy_move.elapsed() >= Duration::from_millis(600) {
            let went_down = alien_movement(game, enemy_dir);
            if went_down {
                enemy_dir *= -1;
            }
            last_enemy_move = Instant::now();
        }

        manage_bullet_on_loop(game, &mut last_bullet_move);

        utils::terminal::clear_terminal();
        game.print_format_board();

        thread::sleep(Duration::from_millis(30));
    }
}