use std::io::{self, BufRead};
use crate::models::Player;Map;

/// Reads a single line from standard input and trims the newline characters.
/// Returns `None` if there is no more input (end of file).
pub fn read_line() -> Option<String> {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    match handle.read_line(&mut buffer) {
        Ok(0) => None, // End of file, no bytes read
        Ok(_) => {
            // Trim the trailing newline
            if buffer.ends_with('\n') {
                buffer.pop();
                // Also trim the carriage return if on Windows
                if buffer.ends_with('\r') {
                    buffer.pop();
                }
            }
            Some(buffer)
        }
        Err(_) => None, // An error occurred
    }
}

pub fn get_player_info() -> Option<Player> {
    // The game engine might send multiple lines before our player assignment.
    // We loop until we find the line that starts with "$$$ exec p".
    loop {
        let line = read_line()?;

       if line.starts_with("$$$ exec p") {
            if line.contains("p1") {
                // We are Player 1
                return Some(Player {
                    p1: '@',
                    p1_alt: 'a',
                    p2: '$',
                    p2_alt: 's',
                });
            } else if line.contains("p2") {
                // We are Player 2
                return Some(Player {
                    p2: '$',
                    p2_alt: 's',
                    p1: '@',
                    p1_alt: 'a',
                });
            }
        }
    }
}

pub fn read_map() -> Option<Map> {
    // Read the "Anfield" line
    let line = read_line()?;
    let parts: Vec<&str> = line.split_whitespace().collect();
    let width: usize = parts[1].parse().ok()?;  // 20
    let height: usize = parts[2].trim_end_matches(':').parse().ok()?; // 15

    // Skip the column header line
    read_line()?;

    let mut grid = Vec::with_capacity(height);
    for _ in 0..height {
        let row_line = read_line()?;
        // Skip the first 4 characters (row number and space)
        let row: Vec<char> = row_line.chars().skip(4).collect();
        grid.push(row);
    }

    Some(Map { width, height, grid })
}

pub fn get_positions(map: &Map, player: &Player) -> (Vec<(usize, usize)>, Vec<(usize, usize)>) {
    let mut my_positions = Vec::new();
    let mut opp_positions = Vec::new();
    for y in 0..map.height {
        for x in 0..map.width {
            let cell = map.grid[y][x];
            if cell == player.p1 || cell == player.p1_alt {
                my_positions.push((y, x));
            } else if cell == player.p2 || cell == player.p2_alt {
                opp_positions.push((y, x));
            }
        }
    }
    (my_positions, opp_positions)
}

pub fn read_piece() -> Option<Piece> {
    let line = read_line()?;
    let parts: Vec<&str> = line.split_whitespace().collect();
    let width: usize = parts[1].parse().ok()?;
    let height: usize = parts[2].trim_end_matches(':').parse().ok()?;

    let mut raw_shape = Vec::with_capacity(height);
    for _ in 0..height {
        let row = read_line()?.chars().collect::<Vec<char>>();
        raw_shape.push(row);
    }

    // Trimming logic here (find min/max rows/cols with 'O')
    let trimmed_shape = trim_piece(&raw_shape);

    Some(Piece { width: trimmed_shape[0].len(), height: trimmed_shape.len(), shape: trimmed_shape })
}

