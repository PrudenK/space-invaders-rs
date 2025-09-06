#[derive(Clone, Copy)]
pub enum Cell {
    Empty,
    Player,
    Alien,
    Bullet,
    Border
}