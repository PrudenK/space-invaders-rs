#[derive(PartialEq, Copy, Clone, Debug)]
pub struct AlienCoords {
    pub x: u16,
    pub y: u16
}

impl AlienCoords{
    pub fn new(x: u16, y: u16) -> Self{
        AlienCoords{x, y}
    }
}
