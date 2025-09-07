use crate::aliens::alien_type::AlienType;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    Empty,
    Player,
    Alien(AlienType),
    Bullet,
    Border,
    AlienBullet,
    RandomOvni,
    Bridge(u8)
}