use crate::models::Player;Map;Piece;

pub fn can_place(
    map: &Map,
    piece: &Piece,
    player: &Player,
    board_y: usize,
    board_x: usize,
) -> bool {
    let mut overlap_count = 0;

    for py in 0..piece.height {
        for px in 0..piece.width {
            if piece.shape[py][px] != 'O' {
                continue;
            }
            let y = board_y + py;
            let x = board_x + px;

            // Check bounds
            if y >= map.height || x >= map.width {
                return false;
            }

            let cell = map.grid[y][x];

            // Check overlap with own territory
            if cell == player.p1 || cell == player.p1_alt {
                overlap_count += 1;
            }
            // Check overlap with opponent
            else if cell == player.p2 || cell == player.p2_alt {
                return false;
            }
            // Otherwise, must be empty ('.')
        }
    }

    // Must overlap exactly one of your own cells
    overlap_count == 1
}