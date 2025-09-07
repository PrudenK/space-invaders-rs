use std::thread;
use std::time::{Duration};
use crate::board::game_state::GameState;
use crate::aliens::alien_movement::{alien_move_loop};
use crate::aliens::alien_shoot::{make_alien_shoot, manage_alien_bullet_on_loop};
use crate::aliens::random_ovni::{manage_random_ovni_on_loop, spwan_random_ovni};
use crate::game_result::result_condition::{calculate_game_status, GameStatus};
use crate::player::bullet::manage_bullet_on_loop;
use crate::player::player_controls::{player_controls};
use crate::utils;
use crate::utils::constants::WAIT_THREAD;

pub fn game_loop(game: &mut GameState) {
    utils::terminal::clear_terminal();
    game.print_format_board();

    loop {
        let end_game = player_controls(game);

        if end_game { break }

        if game.game_status == GameStatus::Continue{
            alien_move_loop(game);

            manage_bullet_on_loop(game);

            make_alien_shoot(game);
            
            manage_alien_bullet_on_loop(game);

            spwan_random_ovni(game);
            
            manage_random_ovni_on_loop(game);

            utils::terminal::clear_terminal();
            game.print_format_board();

            game.game_status = calculate_game_status(game);
        }else{

            if game.game_status == GameStatus::Loss{
                println!("      Game Over!");
            }else if game.game_status == GameStatus::Win{
                println!("      You Win!");
            }

            game.game_status = GameStatus::Waiting
        }

        thread::sleep(Duration::from_millis(WAIT_THREAD));
    }
}