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

impl AlienType {
    pub fn score(&self) -> i32 {
        match self {
            AlienType::Row1 => 100,
            AlienType::Row2 => 80,
            AlienType::Row3 => 70,
            AlienType::Row4 => 60,
            AlienType::Row5 => 50,
            AlienType::Row6 => 40,
            AlienType::Row7 => 30,
            AlienType::Row8 => 20,
            AlienType::Row9 => 10,
        }
    }
}
