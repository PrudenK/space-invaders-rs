use crate::board::cell::Cell;
use crate::board::game_state::{GameState, HEIGHT, WIDTH};
use crate::aliens::alien_coords::AlienCoords;


pub fn alien_movement(board: &mut GameState, direction: i8) -> bool{
    let mut went_down = false;
    let index_to_move = last_alien_index_to_move(board, direction);

    if direction == 1{
        if index_to_move == WIDTH -2{
            alien_side_move(board, 1, true);
            went_down = true;
        }else{
            alien_side_move(board, 1, false);
        }
    }else{
        if index_to_move == 1{
            alien_side_move(board, -1, true);
            went_down = true;
        }else{
            alien_side_move(board, -1, false);
        }
    }

    went_down
}

fn last_alien_index_to_move(board: &GameState, direction: i8) -> usize {
    let mut result = if direction == 1 { 0 } else { WIDTH -1 };

    for i in 1..HEIGHT{
        for j in 1..WIDTH{
            let cell = board.board[i][j];

            if cell == Cell::Alien{
                if direction == 1{
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

fn alien_side_move(board: &mut GameState, direction: isize, down: bool){
    let mut aliens_vector: Vec<AlienCoords> = Vec::new();

    for i in 1..HEIGHT{
        for j in 1..WIDTH{
            let cell = board.board[i][j];
            if cell == Cell::Alien{
                let i_move = if down { (i + 1) as u16 } else { i as u16 };
                let j_move = if down { j as u16 } else { (j as isize + direction) as u16 };
                aliens_vector.push(AlienCoords::new(i_move, j_move));
                board.board[i][j] = Cell::Empty;
            }
        }
    }

    for i in 1..HEIGHT{
        for j in 1..WIDTH{
            if aliens_vector.contains(&AlienCoords::new(i as u16, j as u16)){
                board.board[i][j] = Cell::Alien;
            }
        }
    }
}
