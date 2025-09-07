use crossterm::{event};
use std::time::{Duration};
use crossterm::event::{Event, KeyCode};
use crate::board::game_state::{GameState};
use crate::player::bullet::shot_bullet;
use crate::player::movements::move_player_horizontally;
use crate::utils::constants::{DIR_LEFT, DIR_RIGHT};

pub fn player_controls(game: &mut GameState) -> bool {

    if event::poll(Duration::from_millis(1)).unwrap() {
        if let Event::Key(key_event) = event::read().unwrap() {
            match key_event.code {
                KeyCode::Left => {
                    move_player_horizontally(game, DIR_LEFT);
                }
                KeyCode::Right => {
                    move_player_horizontally(game, DIR_RIGHT);
                }
                KeyCode::Up => {
                    shot_bullet(game);
                }
                KeyCode::Char('r') | KeyCode::Char('R') => {
                    game.restart_game()
                }
                KeyCode::Esc => {
                    return true;
                }
                _ => {}
            }
        }
    }

    false
}
