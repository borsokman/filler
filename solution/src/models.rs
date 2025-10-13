#[derive(Debug)]
pub struct Player {
    pub me: char,
    pub me_alt: char,
    pub opponent: char,
    pub opponent_alt: char,
}

#[derive(Debug)]
pub struct Piece {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<(usize, usize)>, 
}

