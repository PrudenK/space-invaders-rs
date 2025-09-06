use crossterm::{event};
use std::time::Duration;
use crossterm::event::{Event, KeyCode};
use crate::board::cell::Cell;
use crate::board::game_state::{GameState, HEIGHT, WIDTH};

pub fn player_controls(game: &mut GameState) -> bool {
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
                    shot_bullet(game);
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

fn move_player_horizontally(game: &mut GameState, direction: i8) {
    if let Some(j_player_index) = game.board[HEIGHT -2].iter().position(|&c| c == Cell::Player) {
        if direction == 1{
            if j_player_index + 1 != WIDTH -1{
                game.board[HEIGHT -2][j_player_index] = Cell::Empty;
                game.board[HEIGHT -2][j_player_index + 1] = Cell::Player;
            }
        }else{
            if j_player_index - 1 != 0{
                game.board[HEIGHT -2][j_player_index] = Cell::Empty;
                game.board[HEIGHT -2][j_player_index - 1] = Cell::Player;
            }
        }
    }
}

fn shot_bullet(game: &mut GameState) {
    if !is_a_bullet_active(game){
        if let Some(j_player_index) = game.board[HEIGHT -2].iter().position(|&c| c == Cell::Player) {
            if game.board[HEIGHT-3][j_player_index] == Cell::Empty{
                game.board[HEIGHT-3][j_player_index] = Cell::Bullet;
            }
        }
    }
}


pub fn is_a_bullet_active(game: &GameState) -> bool {
    for i in 0..HEIGHT {
        for j in 0..WIDTH {
            if game.board[i][j] == Cell::Bullet{
                return true;
            }
        }
    }

    false
}

pub fn get_bullet_coords(game: &GameState) -> (usize, usize) {
    for i in 0..HEIGHT {
        for j in 0..WIDTH {
            if game.board[i][j] == Cell::Bullet{
                return (i , j);
            } 
        }
    }
    
    (999, 999)
}