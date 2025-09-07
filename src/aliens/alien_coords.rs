use crate::aliens::alien_type::AlienType;

#[derive(PartialEq, Copy, Clone)]
pub struct AlienData {
    pub x: u16,
    pub y: u16,
    pub alien_type: AlienType
}

impl AlienData {
    pub fn new(x: u16, y: u16, alien_type: AlienType) -> Self{
        AlienData {x, y, alien_type}
    }
}
