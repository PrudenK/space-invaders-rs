use std::time::Instant;
use crate::board::cell::Cell;

pub const WIDTH: usize = 13;
pub const HEIGHT: usize = 20;


pub struct GameState {
    pub board: [[Cell; WIDTH]; HEIGHT],
    pub last_enemy_move: Instant,
    pub last_bullet_move: Instant,
    pub last_bullet_shooted: Instant,
    pub last_alien_bullet_shooted: Instant,
    pub last_alien_bullet_move: Instant,
    pub enemy_dir: i8
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            board: [[Cell::Empty; WIDTH]; HEIGHT],
            last_enemy_move: Instant::now(),
            last_bullet_move: Instant::now(),
            last_bullet_shooted: Instant::now(),
            last_alien_bullet_shooted: Instant::now(),
            last_alien_bullet_move: Instant::now(),
            enemy_dir: 1
        }
    }

    pub fn set_up_new_game(self: &mut GameState) {
        for i in 0..HEIGHT {
            for j in 0..WIDTH {
                if i > 0 && i < 10{
                    if j % 2 == 0 && j < 10 && j > 0{
                        self.board[i][j] = Cell::Alien;
                    }else{
                        self.board[i][j] = Cell::Empty;
                    }
                }else{
                    self.board[i][j] = Cell::Empty;
                }

                if i == 0 || j == 0 || i == HEIGHT -1 || j == WIDTH -1{
                    self.board[i][j] = Cell::Border;
                }
            }
        }

        self.board[HEIGHT-2][WIDTH/2] = Cell::Player;
    }

    pub fn print_format_board(&self) {
        for i in 0..HEIGHT {
            for j in 0..WIDTH {
                match self.board[i][j] {
                    Cell::Empty  => print!("   "),
                    Cell::Alien  => print!("\x1b[37m~X~\x1b[0m"),
                    Cell::Player => print!("\x1b[37m<A>\x1b[0m"),
                    Cell::Bullet => print!("\x1b[37m | \x1b[0m"),
                    Cell::AlienBullet => print!("\x1b[31m | \x1b[0m"),
                    Cell::Border => print!("\x1b[100m   \x1b[0m"),
                }
            }
            print!("\r\n");
        }
    }
}