use std::time::Instant;
use crate::aliens::alien_type::AlienType;
use crate::board::cell::{Cell};
use crate::game_result::result_condition::GameStatus;
use crate::utils::constants::{DIR_RIGHT, PLAYER_LIVES};

pub const WIDTH: usize = 19;
pub const HEIGHT: usize = 32;
const TOP_BRIDGE_LIST: [u16; 9] = [3, 4, 5, 8, 9, 10, 13, 14, 15];
const SIDE_BRIDGE_LIST: [u16; 6] = [3, 5, 8, 10, 13, 15];

pub struct GameState {
    pub board: [[Cell; WIDTH]; HEIGHT],
    pub last_enemy_move: Instant,
    pub last_bullet_move: Instant,
    pub last_bullet_shooted: Instant,
    pub last_alien_bullet_shooted: Instant,
    pub last_alien_bullet_move: Instant,
    pub last_random_ovni_spawned: Instant,
    pub last_random_ovni_moved: Instant,
    pub game_status: GameStatus,
    pub score: i32,
    pub enemy_dir: i8,
    pub random_ovni_dir: i8,
    pub lives: u8,
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
            last_random_ovni_spawned: Instant::now(),
            last_random_ovni_moved: Instant::now(),
            game_status: GameStatus::Continue,
            score: 0,
            enemy_dir: DIR_RIGHT,
            random_ovni_dir: DIR_RIGHT,
            lives: PLAYER_LIVES
        }
    }

    pub fn set_up_new_game(self: &mut GameState) {
        for i in 0..HEIGHT {
            for j in 0..WIDTH {
                if i > 1 && i < 11{
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

                    if j % 2 == 0 && j < 16 && j > 0{
                        self.board[i][j] = Cell::Alien(alien_type);
                    }else{
                        self.board[i][j] = Cell::Empty;
                    }
                }else{
                    if i == HEIGHT - 5 {
                        self.set_bridg_cell(i, j, &TOP_BRIDGE_LIST)
                    }else if i == HEIGHT - 4{
                        self.set_bridg_cell(i, j, &SIDE_BRIDGE_LIST)
                    }else{
                        self.board[i][j] = Cell::Empty;
                    }

                }

                if i == 0 || j == 0 || i == HEIGHT -1 || j == WIDTH -1{
                    self.board[i][j] = Cell::Border;
                }
            }
        }

        self.board[HEIGHT-2][WIDTH/2] = Cell::Player;
    }

    fn set_bridg_cell(&mut self, i: usize, j: usize, bridge_list: &[u16]){
        if bridge_list.contains(&(j as u16)){
            self.board[i][j] = Cell::Bridge(4);
        }else{
            self.board[i][j] = Cell::Empty;
        }
    }

    pub fn restart_game(&mut self) {
        self.set_up_new_game();
        self.last_enemy_move = Instant::now();
        self.last_bullet_move = Instant::now();
        self.last_bullet_shooted = Instant::now();
        self.last_alien_bullet_shooted = Instant::now();
        self.last_alien_bullet_move = Instant::now();
        self.last_random_ovni_spawned = Instant::now();
        self.last_random_ovni_moved = Instant::now();
        self.game_status = GameStatus::Continue;
        self.enemy_dir = DIR_RIGHT;
        self.score = 0;
        self.lives = PLAYER_LIVES
    }

    pub fn print_format_board(&self) {
        for i in 0..HEIGHT {
            for j in 0..WIDTH {
                match self.board[i][j] {
                    Cell::Empty => print!("   "),
                    Cell::Player => print!("\x1b[37m<Â>\x1b[0m"),
                    Cell::Bullet => print!("\x1b[37m | \x1b[0m"),
                    Cell::AlienBullet => print!("\x1b[31m | \x1b[0m"),
                    Cell::Border => {
                        print!("\x1b[100m   \x1b[0m");

                        if i == 3 && j == WIDTH -1{
                            print!("  Developed by PrudenK");
                        }

                        if i == 4 && j == WIDTH -1{
                            print!("  On Github: https://github.com/PrudenK/space-invaders-rs");
                        }

                        if i == 6 && j == WIDTH -1{
                            print!("  Score : {}", self.score);
                        }

                        if i == 8 && j == WIDTH -1{
                            print!("  Lives:");
                            for _ in 0..self.lives {
                                print!("\x1b[37m ♥ \x1b[0m");
                            }
                        }

                        if i == 10 && j == WIDTH -1{
                            print!("  Press r for new game");
                        }

                        if i == 12 && j == WIDTH -1{
                            print!("  Press Esc to exit");
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

                    Cell::Bridge(hp) => match hp {
                        4 => print!("\x1b[48;5;46m   \x1b[0m"),
                        3 => print!("\x1b[48;5;118m   \x1b[0m"),
                        2 => print!("\x1b[48;5;190m   \x1b[0m"),
                        1 => print!("\x1b[48;5;208m   \x1b[0m"),
                        0 => print!("\x1b[48;5;196m   \x1b[0m"),
                        _ => print!("   "),
                    },

                    Cell::RandomOvni => print!("\x1b[41m[O]\x1b[0m"),
                }
            }
            print!("\r\n");
        }
    }
}