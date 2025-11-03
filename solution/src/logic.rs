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

    let untrimmed_y = board_y.saturating_sub(piece.offset_y);
    let untrimmed_x = board_x.saturating_sub(piece.offset_x);
    // Check if the full untrimmed piece would go out of bounds
    if untrimmed_y + piece.original_height > map.height {
        log_logic(&format!("can_place REJECT: untrimmed_y={} + original_height={} = {} > map.height={}", 
            untrimmed_y, piece.original_height, untrimmed_y + piece.original_height, map.height));
        return false;
    }
    if untrimmed_x + piece.original_width > map.width {
        log_logic(&format!("can_place REJECT: untrimmed_x={} + original_width={} = {} > map.width={}", 
            untrimmed_x, piece.original_width, untrimmed_x + piece.original_width, map.width));
        return false;
    }
    let mut overlap_count = 0;
    log_logic(&format!("can_place called: board_y={}, board_x={}, piece={}x{}", board_y, board_x, piece.width, piece.height));

    for py in 0..piece.height {
        for px in 0..piece.width {
            if piece.shape[py][px] != 'O' {
                continue;
            }
            let y = board_y + py;
            let x = board_x + px;

            if y >= map.height || x >= map.width {
                log_logic(&format!("  Out of bounds at ({}, {})", y, x));
                return false;
            }

            let cell = map.grid[y][x];
            log_logic(&format!("  Checking ({}, {}) = '{}', piece.shape[{}][{}] = '{}'", y, x, cell, py, px, piece.shape[py][px]));

            if cell == player.p1 || cell == player.p1_alt {
                overlap_count += 1;
                if overlap_count > 1 {
                    log_logic(&format!("  Too many overlaps at ({}, {}) count={}", y, x, overlap_count));
                     return false;
                }
            }
            else if cell == player.p2 || cell == player.p2_alt {
                log_logic("    ✗ Overlap with enemy!");
                return false;
            }
        }
    }

    let result = overlap_count == 1;
    log_logic(&format!("  Final: overlap_count={}, returning {}", overlap_count, result));
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
    log_logic(&format!("\n=== FIND_BEST_MOVE START ==="));
    log_logic(&format!("Piece: {}x{} (trimmed), original: {}x{}, offset: ({}, {})",
        piece.width, piece.height, piece.original_width, piece.original_height,
        piece.offset_x, piece.offset_y));
    
    if piece.height > map.height || piece.width > map.width {
        log_logic("Piece too large for map!");
        return (0, 0);
    }

    let (my_positions, enemy_positions) = get_positions(map, player);
    log_logic(&format!("My territory: {} cells, Enemy: {} cells", my_positions.len(), enemy_positions.len()));
    
    let current_global_min = shortest_distance_between_players(map, player);
    log_logic(&format!("Current global min distance: {}", current_global_min));
    
    let mut all_moves: Vec<((usize, usize), usize, usize, usize)> = Vec::new();
    
    let max_y = map.height - piece.height;
    let max_x = map.width - piece.width;
    log_logic(&format!("Scanning positions: y=0..={}, x=0..={}", max_y, max_x));
  
    for y in 0..=max_y {
        for x in 0..=max_x {
            if can_place(map, piece, player, y, x) {
                let touch_count = count_adjacent_enemy_cells(map, piece, player, y, x);
                
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
                log_logic(&format!("✓ Valid move at ({}, {}): touch={}, dist={}, cells={}",
                    y, x, touch_count, local_min_dist, new_cells));
                all_moves.push(((y, x), touch_count, local_min_dist, new_cells));
            }
        }
    }

    log_logic(&format!("Total valid moves found: {}", all_moves.len()));

    if all_moves.is_empty() {
        log_logic("❌ NO VALID MOVES - Returning (0, 0)");
        return (0, 0);
    }

    // Sort moves
    all_moves.sort_by(|a, b| {
        match b.1.cmp(&a.1) { 
            std::cmp::Ordering::Equal => {
                if a.1 > 0 {
                    match b.3.cmp(&a.3) {
                        std::cmp::Ordering::Equal => a.0.cmp(&b.0),
                        other => other,
                    }
                } else {
                    let a_improves = a.2 < current_global_min;
                    let b_improves = b.2 < current_global_min;
                    
                    match (a_improves, b_improves) {
                        (true, false) => std::cmp::Ordering::Less,
                        (false, true) => std::cmp::Ordering::Greater,
                        _ => {
                            match a.2.cmp(&b.2) {
                                std::cmp::Ordering::Equal => {
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
            other => other,
        }
    });

    log_logic(&format!("🏆 Best move: ({}, {}) with touch={}, dist={}, cells={}",
        all_moves[0].0.0, all_moves[0].0.1, all_moves[0].1, all_moves[0].2, all_moves[0].3));
    log_logic(&format!("Top 5 moves:"));
    for (i, m) in all_moves.iter().take(5).enumerate() {
        log_logic(&format!("  {}. ({}, {}) touch={}, dist={}, cells={}",
            i+1, m.0.0, m.0.1, m.1, m.2, m.3));
    }

    all_moves[0].0
}

