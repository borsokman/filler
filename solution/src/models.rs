#[derive(Debug)]
pub struct Player {
    pub p1: char,
    pub p1_alt: char,
    pub p2: char,
    pub p2_alt: char,
}

pub struct Map {
    pub width: usize,
    pub height: usize,
    pub grid: Vec<Vec<char>>,
}

pub struct Piece {
    pub width: usize,
    pub height: usize,
    pub shape: Vec<Vec<char>>,
    pub offset_x: usize,
    pub offset_y: usize,
    pub original_width: usize,
    pub original_height: usize,
}
