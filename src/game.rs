use std::thread;
use std::time::{Duration};
use crate::board::game_state::GameState;
use crate::aliens::alien_movement::{alien_move_loop};
use crate::aliens::alien_shoot::{make_alien_shoot, manage_alien_bullet_on_loop};
use crate::player::bullet::manage_bullet_on_loop;
use crate::player::player_controls::{player_controls};
use crate::utils;

pub fn game_loop(game: &mut GameState) {
    utils::terminal::clear_terminal();
    game.print_format_board();

    loop {
        let end_game = player_controls(game);

        if end_game { break }

        alien_move_loop(game);

        manage_bullet_on_loop(game);

        make_alien_shoot(game);

        manage_alien_bullet_on_loop(game);

        utils::terminal::clear_terminal();
        game.print_format_board();

        thread::sleep(Duration::from_millis(30));
    }
}