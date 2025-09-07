use std::time::Instant;
use crate::aliens::alien_type::AlienType;
use crate::board::cell::{Cell};
use crate::game_result::result_condition::GameStatus;

pub const WIDTH: usize = 16;
pub const HEIGHT: usize = 24;

pub struct GameState {
    pub board: [[Cell; WIDTH]; HEIGHT],
    pub last_enemy_move: Instant,
    pub last_bullet_move: Instant,
    pub last_bullet_shooted: Instant,
    pub last_alien_bullet_shooted: Instant,
    pub last_alien_bullet_move: Instant,
    pub game_status: GameStatus,
    pub score: i32,
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
            game_status: GameStatus::Continue,
            score: 0,
            enemy_dir: 1
        }
    }

    pub fn set_up_new_game(self: &mut GameState) {
        for i in 0..HEIGHT {
            for j in 0..WIDTH {
                if i > 1 && i < 12{
                    let alien_type = match i - 1 {
                        1 => AlienType::Row1,
                        2 => AlienType::Row2,
                        3 => AlienType::Row3,
                        4 => AlienType::Row4,
                        5 => AlienType::Row5,
                        6 => AlienType::Row6,
                        7 => AlienType::Row7,
                        8 => AlienType::Row8,
                        9 => AlienType::Row9,
                        _ => AlienType::Row1,
                    };

                    if j % 2 == 0 && j < 12 && j > 0{
                        self.board[i][j] = Cell::Alien(alien_type);
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

    pub fn restart_game(&mut self) {
        self.set_up_new_game();
        self.last_enemy_move = Instant::now();
        self.last_bullet_move = Instant::now();
        self.last_bullet_shooted = Instant::now();
        self.last_alien_bullet_shooted = Instant::now();
        self.last_alien_bullet_move = Instant::now();
        self.game_status = GameStatus::Continue;
        self.enemy_dir = 1;
        self.score = 0;
    }

    pub fn print_format_board(&self) {
        for i in 0..HEIGHT {
            for j in 0..WIDTH {
                match self.board[i][j] {
                    Cell::Empty => print!("   "),
                    Cell::Player => print!("\x1b[37m<A>\x1b[0m"),
                    Cell::Bullet => print!("\x1b[37m | \x1b[0m"),
                    Cell::AlienBullet => print!("\x1b[31m | \x1b[0m"),
                    Cell::Border => {
                        print!("\x1b[100m   \x1b[0m");

                        if i == 5 && j == WIDTH -1{
                            print!("  Score : {}", self.score);
                        }

                        if i == 7 && j == WIDTH -1{
                            print!("  Press r for new game");
                        }
                    },
                    Cell::Alien(AlienType::Row1) => print!("\x1b[32m~X~\x1b[0m"),
                    Cell::Alien(AlienType::Row2) => print!("\x1b[94m-$-\x1b[0m"),
                    Cell::Alien(AlienType::Row3) => print!("\x1b[34mx0x\x1b[0m"),
                    Cell::Alien(AlienType::Row4) => print!("\x1b[35mzZz\x1b[0m"),
                    Cell::Alien(AlienType::Row5) => print!("\x1b[36m~^~\x1b[0m"),
                    Cell::Alien(AlienType::Row6) => print!("\x1b[92m-@-\x1b[0m"),
                    Cell::Alien(AlienType::Row7) => print!("\x1b[93mqOp\x1b[0m"),
                    Cell::Alien(AlienType::Row8) => print!("\x1b[33m.&.\x1b[0m"),
                    Cell::Alien(AlienType::Row9) => print!("\x1b[95mwMw\x1b[0m"),
                }
            }
            print!("\r\n");
        }
    }
}