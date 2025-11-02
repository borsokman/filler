use crate::models::{Map, Piece, Player};
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

// Scans the grid after parsing to find all cells belonging to you and your opponent.
pub fn get_positions(map: &Map, player: &Player) -> (Vec<(usize, usize)>, Vec<(usize, usize)>) {
    let mut my_positions = Vec::new();
    let mut enemy_positions = Vec::new();
    for y in 0..map.height {
        for x in 0..map.width {
            let cell = map.grid[y][x];
            if cell == player.p1 || cell == player.p1_alt {
                my_positions.push((y, x));
            } else if cell == player.p2 || cell == player.p2_alt {
                enemy_positions.push((y, x));
            }
        }
    }
    (my_positions, enemy_positions)
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

pub fn shortest_distance_between_players(map: &Map, player: &Player) -> usize {
    let (my_positions, enemy_positions) = get_positions(map, player);

    if my_positions.is_empty() || enemy_positions.is_empty() {
        return usize::MAX;
    }

    let mut min_distance = usize::MAX;

    for &(my_y, my_x) in &my_positions {
        for &(enemy_y, enemy_x) in &enemy_positions {
            let dist = ((my_y as isize - enemy_y as isize).abs() + (my_x as isize - enemy_x as isize).abs()) as usize;
            if dist < min_distance {
                min_distance = dist;
            }
        }
    }
    min_distance
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
    let current_global_min = shortest_distance_between_players(map, player);
    let mut all_moves: Vec<((usize, usize), usize, usize, usize)> = Vec::new(); 
  
    for y in 0..=map.height - piece.height {
        for x in 0..=map.width - piece.width {
            if can_place(map, piece, player, y, x) {
                let touch_count = count_adjacent_enemy_cells(map, piece, player, y, x);
                
                // Calculate minimum distance from this piece placement to enemy
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

    // Sort with awareness of global position
    all_moves.sort_by(|a, b| {
        // 1. Compare touch counts (higher is better)
        match b.1.cmp(&a.1) { 
            std::cmp::Ordering::Equal => {
                // 2. If both have same touch count...
                if a.1 > 0 {
                    // 3. Both are touching (touch_count > 0): prefer more cells
                    match b.3.cmp(&a.3) {
                        std::cmp::Ordering::Equal => a.0.cmp(&b.0), // Tie: prefer top-left
                        other => other,
                    }
                } else {
                    // 4. Neither is touching (touch_count == 0): prefer getting closer
                    let a_improves = a.2 < current_global_min;
                    let b_improves = b.2 < current_global_min;
                    
                    match (a_improves, b_improves) {
                        (true, false) => std::cmp::Ordering::Less,    // A improves, B doesn't → A wins
                        (false, true) => std::cmp::Ordering::Greater, // B improves, A doesn't → B wins
                        _ => {
                            // Both improve or both don't: compare distances
                            match a.2.cmp(&b.2) { // Smaller distance is better
                                std::cmp::Ordering::Equal => {
                                    // Same distance: prefer more cells
                                    match b.3.cmp(&a.3) {
                                        std::cmp::Ordering::Equal => a.0.cmp(&b.0),
                                        other => other,
                                    }
                                }
                                other => other,
                            }
                        }
                    }
                }
            }
            other => other, // Not equal? Use the touch_count comparison result
        }
    });

    all_moves[0].0
}

