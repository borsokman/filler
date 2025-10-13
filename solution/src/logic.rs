use crate::models::Player;

pub fn is_valid_placement(
    anfield: &Vec<Vec<char>>,
    piece_cells: &Vec<(usize, usize)>,
    top_left_x: isize,
    top_left_y: isize,
    player: &Player,
) -> bool {
    let mut overlap_count = 0;

    for (px, py) in piece_cells {
        let x = top_left_x + *px as isize;
        let y = top_left_y + *py as isize;

        if x < 0 || y < 0 || y as usize >= anfield.len() || x as usize >= anfield[0].len() {
            return false;
        }

        let cell = anfield[y as usize][x as usize];
        if cell == player.opponent || cell == player.opponent_alt { return false; }
        if cell == player.me || cell == player.me_alt { overlap_count += 1; }
    }

    overlap_count == 1
}

pub fn find_best_move(
    anfield: &Vec<Vec<char>>,
    piece_cells: &Vec<(usize, usize)>,
    player: &Player,
) -> Option<(usize, usize)> {
    for y in 0..anfield.len() {
        for x in 0..anfield[0].len() {
            if is_valid_placement(anfield, piece_cells, x as isize, y as isize, player) {
                return Some((x, y));
            }
        }
    }
    None
}
