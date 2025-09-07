use crate::board::cell::AlienType;

#[derive(PartialEq, Copy, Clone)]
pub struct AlienCoords {
    pub x: u16,
    pub y: u16,
    pub alien_type: AlienType
}

impl AlienCoords{
    pub fn new(x: u16, y: u16, alien_type: AlienType) -> Self{
        AlienCoords{x, y, alien_type}
    }
}
