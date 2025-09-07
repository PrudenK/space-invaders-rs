use crate::aliens::alien_shoot::get_bottom_aliens_coords;
use crate::board::cell::Cell;
use crate::board::game_state::{GameState, HEIGHT};
use crate::utils::board_utils::is_cell_active;

#[derive(PartialEq, Eq)]
pub enum GameStatus{
    Win,
    Loss,
    Continue,
    Waiting
}

pub fn calculate_game_status(game: &GameState) -> GameStatus {
    if !is_cell_active(game, |c| matches!(c, Cell::Alien(_))) {
        return GameStatus::Win;
    }

    if !is_cell_active(game, |c| *c == Cell::Player) || has_alien_reached_player_row(game){
        return GameStatus::Loss;
    }

    GameStatus::Continue
}

fn has_alien_reached_player_row(game: &GameState) -> bool{
    get_bottom_aliens_coords(game)
        .iter()
        .any(|coord| coord.x as usize == HEIGHT - 2)
}
