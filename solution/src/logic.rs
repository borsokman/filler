use crate::models::{Map, Piece, Player};
use crate::input::get_positions;
use std::fs::OpenOptions;
use std::io::Write;

fn log_logic(msg: &str) {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("/tmp/bot_logic.log")
        .unwrap();
    writeln!(file, "{}", msg).ok();
}

pub fn can_place(
    map: &Map,
    piece: &Piece,
    player: &Player,
    board_y: usize,
    board_x: usize,
) -> bool {
    let mut overlap_count = 0;
    log_logic(&format!("can_place called: board_y={}, board_x={}, piece={}x{}", board_y, board_x, piece.width, piece.height));

    for py in 0..piece.height {
        for px in 0..piece.width {
            if piece.shape[py][px] != 'O' {
                continue;
            }
            let y = board_y + py;
            let x = board_x + px;

            // Check bounds
            if y >= map.height || x >= map.width {
                log_logic(&format!("  Out of bounds at ({}, {})", y, x));
                return false;
            }

            let cell = map.grid[y][x];
            log_logic(&format!("  Checking ({}, {}) = '{}'", y, x, cell));

            // Check overlap with own territory
            if cell == player.p1 || cell == player.p1_alt {
                overlap_count += 1;
                log_logic(&format!("    Overlap with own territory, count={}", overlap_count));
            }
            // Check overlap with opponent
            else if cell == player.p2 || cell == player.p2_alt {
                log_logic("    Overlap with enemy!");
                return false;
            }
        }
    }

    let result = overlap_count == 1;
    log_logic(&format!("  Result: overlap_count={}, returning {}", overlap_count, result));
    result
}

fn count_adjacent_enemy_cells(map: &Map, piece: &Piece, player: &Player, board_y: usize, board_x: usize) -> usize {
    let mut count = 0;
    for py in 0..piece.height {
        for px in 0..piece.width {
            if piece.shape[py][px] != 'O' {
                continue;
            }
            let y = board_y + py;
            let x = board_x + px;
            let adjacents = [
                (y.wrapping_sub(1), x),
                (y + 1, x),
                (y, x.wrapping_sub(1)),
                (y, x + 1),
            ];
            for &(ay, ax) in &adjacents {
                if ay < map.height && ax < map.width {
                    let cell = map.grid[ay][ax];
                    if cell == player.p2 || cell == player.p2_alt {
                        count += 1;
                    }
                }
            }
        }
    }
    count
}

fn count_new_cells(piece: &Piece) -> usize {
    let mut count = 0;
    for py in 0..piece.height {
        for px in 0..piece.width {
            if piece.shape[py][px] == 'O' {
                count += 1;
            }
        }
    }
    count
}

pub fn find_best_move(map: &Map, piece: &Piece, player: &Player) -> (usize, usize) {
    // Handle pieces that are too large for the map to prevent panics.
    if piece.height > map.height || piece.width > map.width {
        return (0, 0);
    }

    let (_, enemy_positions) = get_positions(map, player);
    
    // Store all valid moves with their scores
    let mut all_moves: Vec<((usize, usize), usize, usize, usize)> = Vec::new(); 
    // Format: ((y, x), touch_count, distance, new_cells)

    for y in 0..=map.height - piece.height {
        for x in 0..=map.width - piece.width {
            if can_place(map, piece, player, y, x) {
                let touch_count = count_adjacent_enemy_cells(map, piece, player, y, x);
                
                // Calculate minimum distance to enemy
                let mut local_min_dist = usize::MAX;
                for py in 0..piece.height {
                    for px in 0..piece.width {
                        if piece.shape[py][px] == 'O' {
                            let piece_y = y + py;
                            let piece_x = x + px;
                            for &(ey, ex) in &enemy_positions {
                                let dist = ((piece_y as isize - ey as isize).abs()
                                    + (piece_x as isize - ex as isize).abs()) as usize;
                                if dist < local_min_dist {
                                    local_min_dist = dist;
                                }
                            }
                        }
                    }
                }
                
                let new_cells = count_new_cells(piece);
                all_moves.push(((y, x), touch_count, local_min_dist, new_cells));
            }
        }
    }

    if all_moves.is_empty() {
        return (0, 0);
    }

    // Sort by priority:
    // 1. Maximum touch count (blocking enemy)
    // 2. Maximum new cells (if touching)
    // 3. Minimum distance to enemy (if not touching)
    // 4. Top-left position (tie-breaker)
    all_moves.sort_by(|a, b| {
        // First: prioritize touching the enemy
        match b.1.cmp(&a.1) {
            std::cmp::Ordering::Equal => {
                // If both touch equally (including both = 0)
                if a.1 > 0 {
                    // Both are touching: prefer more new cells
                    match b.3.cmp(&a.3) {
                        std::cmp::Ordering::Equal => {
                            // Same cells: prefer top-left
                            a.0.cmp(&b.0)
                        }
                        other => other,
                    }
                } else {
                    // Neither is touching: prefer closer to enemy
                    match a.2.cmp(&b.2) {
                        std::cmp::Ordering::Equal => {
                            // Same distance: prefer more new cells
                            match b.3.cmp(&a.3) {
                                std::cmp::Ordering::Equal => {
                                    // Same cells: prefer top-left
                                    a.0.cmp(&b.0)
                                }
                                other => other,
                            }
                        }
                        other => other,
                    }
                }
            }
            other => other,
        }
    });

    all_moves[0].0
}