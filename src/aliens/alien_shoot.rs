use std::collections::HashMap;
use std::time::{Duration, Instant};
use rand::{Rng};
use crate::aliens::alien_coords::AlienData;
use crate::aliens::alien_type::AlienType;
use crate::board::cell::{Cell};
use crate::board::game_state::{GameState, HEIGHT, WIDTH};
use crate::utils::board_utils::{get_cell_coords, is_cell_active, ERROR_NUMBER};

const ALIEN_BULLET_COOLDOWN: u64 = 600;

pub fn make_alien_shoot(game: &mut GameState) {
    if !is_cell_active(game, |c| *c == Cell::AlienBullet) && game.last_alien_bullet_shooted.elapsed() >= Duration::from_millis(ALIEN_BULLET_COOLDOWN) {
        game.last_alien_bullet_shooted = Instant::now();

        if let Some(shooter) = calculate_alien_will_shot(game){
            game.board[(shooter.x + 1) as usize][shooter.y as usize] = Cell::AlienBullet
        }
    }
}

pub fn manage_alien_bullet_on_loop(game: &mut GameState){
    if is_cell_active(game, |c| *c == Cell::AlienBullet) {
        if game.last_alien_bullet_move.elapsed() >= Duration::from_millis(80){
            let (i_index, j_index) = get_cell_coords(game, Cell::AlienBullet);

            if i_index != ERROR_NUMBER && j_index != ERROR_NUMBER{
                game.last_alien_bullet_move = Instant::now();

                game.board[i_index][j_index] = Cell::Empty;

                match game.board[i_index + 1][j_index] {
                    Cell::Border => {},
                    Cell::Bullet => {
                        game.board[i_index + 1][j_index] = Cell::Empty
                    },
                    Cell::Bridge(hp) if hp > 1 => {
                        game.board[i_index + 1][j_index] = Cell::Bridge(hp - 1);
                    }
                    Cell::Bridge(_) => {
                        game.board[i_index + 1][j_index] = Cell::Empty;
                    }
                    Cell::Player => {
                        game.board[i_index + 1][j_index] = Cell::Empty;

                        game.lives -= 1;

                        if game.lives > 0{
                            game.board[HEIGHT-2][WIDTH/2] = Cell::Player;
                        }
                    }
                    _ => game.board[i_index + 1][j_index] = Cell::AlienBullet
                }
            }
        }
    }
}

fn calculate_alien_will_shot(game: &GameState) -> Option<AlienData> {
    let mut rng = rand::thread_rng();

    let bottom_aliens:Vec<AlienData> = get_bottom_aliens_coords(game);
    let bottom_aliens_len = bottom_aliens.len();

    let index_shoot = rng.gen_range(0..bottom_aliens_len + 1);

    bottom_aliens.get(index_shoot).copied()
}



pub fn get_bottom_aliens_coords(game: &GameState) -> Vec<AlienData> {
    let mut bottom_by_col: HashMap<u16, AlienData> = HashMap::new();

    for i in 1..HEIGHT{
        for j in 1..WIDTH{
            let cell = game.board[i][j];
            if matches!(cell, Cell::Alien(_)) {
                let coord = AlienData::new(i as u16, j as u16, AlienType::Row1);

                bottom_by_col
                    .entry(j as u16)
                    .and_modify(|existing|{
                        if coord.x > existing.x {
                            *existing = coord;
                        }
                    })
                    .or_insert(coord);
            }
        }
    }

    bottom_by_col.into_values().collect()
}

