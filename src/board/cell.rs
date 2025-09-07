

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    Empty,
    Player,
    Alien(AlienType),
    Bullet,
    Border,
    AlienBullet
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum AlienType {
    Row1,
    Row2,
    Row3,
    Row4,
    Row5,
    Row6,
    Row7,
    Row8,
    Row9,
}