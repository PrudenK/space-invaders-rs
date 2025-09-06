use crossterm::{event};
use std::time::{Duration, Instant};
use crossterm::event::{Event, KeyCode};
use crate::board::game_state::{GameState};
use crate::player::bullet::shot_bullet;
use crate::player::movements::move_player_horizontally;

pub fn player_controls(
    game: &mut GameState,
    last_bullet_shooted: &mut Instant,
) -> bool {

    if event::poll(Duration::from_millis(1)).unwrap() {
        if let Event::Key(key_event) = event::read().unwrap() {
            match key_event.code {
                KeyCode::Left => {
                    move_player_horizontally(game, -1);
                }
                KeyCode::Right => {
                    move_player_horizontally(game, 1);
                }
                KeyCode::Up => {
                    shot_bullet(game, last_bullet_shooted);
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
