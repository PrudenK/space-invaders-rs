use std::time::{Duration, Instant};
use crate::board::cell::Cell;
use crate::board::game_state::{GameState, HEIGHT, WIDTH};
use crate::aliens::alien_coords::AlienData;


pub fn alien_move_loop(game: &mut GameState) {
    if game.last_enemy_move.elapsed() >= Duration::from_millis(600) {
        let went_down = alien_movement(game);
        if went_down {
            game.enemy_dir *= -1;
        }
        game.last_enemy_move = Instant::now();
    }
}

fn alien_movement(game: &mut GameState) -> bool{
    let mut went_down = false;
    let index_to_move = last_alien_index_to_move(game);

    if game.enemy_dir == 1{
        if index_to_move == WIDTH -2{
            alien_side_move(game, true);
            went_down = true;
        }else{
            alien_side_move(game, false);
        }
    }else{
        if index_to_move == 1{
            alien_side_move(game,true);
            went_down = true;
        }else{
            alien_side_move(game, false);
        }
    }

    went_down
}

fn last_alien_index_to_move(game: &GameState) -> usize {
    let mut result = if game.enemy_dir == 1 { 0 } else { WIDTH -1 };

    for i in 1..HEIGHT{
        for j in 1..WIDTH{
            let cell = game.board[i][j];

            if matches!(cell, Cell::Alien(_)) {
                if game.enemy_dir == 1{
                    if j > result{
                        result = j;
                    }
                }else{ // -1
                    if j < result{
                        result = j;
                    }
                }
            }
        }
   }
    
    result
}

fn alien_side_move(game: &mut GameState, down: bool){
    let mut aliens_vector: Vec<AlienData> = Vec::new();

    for i in 1..HEIGHT{
        for j in 1..WIDTH{
            let cell = game.board[i][j];
            if matches!(cell, Cell::Alien(_)) {
                if let Cell::Alien(alien_type) = cell {
                    let i_move = if down { (i + 1) as u16 } else { i as u16 };
                    let j_move = if down { j as u16 } else { (j as isize + game.enemy_dir as isize) as u16 };

                    aliens_vector.push(AlienData {
                        x: i_move,
                        y: j_move,
                        alien_type,
                    });

                    game.board[i][j] = Cell::Empty;
                }
            }
        }
    }

    for alien in aliens_vector {
        game.board[alien.x as usize][alien.y as usize] = Cell::Alien(alien.alien_type);
    }
}
