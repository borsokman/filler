use std::io::{self, BufRead};
use crate::models::{Map, Piece, Player};

/// Reads a single line from standard input and trims the newline characters.
/// Returns `None` if there is no more input (end of file).
/// Skip empty lines.
pub fn read_line() -> Option<String> {
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    
    loop {
        let mut buffer = String::new();
        match handle.read_line(&mut buffer) {
            Ok(0) => {
                return None; // End of file
            }
            Ok(_) => {
                // Trim the trailing newline
                if buffer.ends_with('\n') {
                    buffer.pop();
                    if buffer.ends_with('\r') {
                        buffer.pop();
                    }
                }
                // Skip empty lines
                let trimmed = buffer.trim();
                if !trimmed.is_empty() {
                    return Some(trimmed.to_string());
                }
            }
            Err(_) => {
                return None;
            }
        }
    }
}

// Reads player info and sets the correct symbols for your bot and the opponent.
pub fn get_player_info() -> Option<Player> {
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
                    p1: '$',
                    p1_alt: 's',
                    p2: '@',
                    p2_alt: 'a',
                });
            }
        }
    }
}

pub fn read_map() -> Option<Map> {
    // Keep reading until we find the "Anfield" line
    let line = loop {
        let l = read_line()?;
        if l.contains("Anfield") {
            break l;
        }
    };
    
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() < 3 {
        return None;
    }
    let width: usize = parts[1].parse().ok()?;
    let height: usize = parts[2].trim_end_matches(':').parse().ok()?;

    // Skip the column header line
    read_line()?;

    let mut grid = Vec::with_capacity(height);
    for _ in 0..height {
        let row_line = read_line()?;
        // Skip the first 4 characters (row number and space)
        if row_line.len() < 4 {
            return None;
        }
        let row: Vec<char> = row_line.chars().skip(4).collect();
        if row.len() != width {
            return None;
        }
        grid.push(row);
    }

    Some(Map { width, height, grid })
}

// Reads the piece, trims it to its minimal bounding box, and stores the trimmed shape.
pub fn read_piece() -> Option<Piece> {
    let line = loop {
        let l = read_line()?;
        if l.starts_with("Piece") {
            break l;
        }
    };

    let parts: Vec<&str> = line.split_whitespace().collect();
    let original_width: usize = parts[1].parse().ok()?;
    let original_height: usize = parts[2].trim_end_matches(':').parse().ok()?;
    let _width: usize = parts[1].parse().ok()?;
    let height: usize = parts[2].trim_end_matches(':').parse().ok()?;

    let mut raw_shape = Vec::with_capacity(height);
    for _ in 0..height {
        let row = read_line()?.chars().collect::<Vec<char>>();
        raw_shape.push(row);
    }

    let (trimmed_shape, offset_y, offset_x) = trim_piece(&raw_shape);

    if trimmed_shape.is_empty() {
        // If the piece is empty, return a piece with 0 width and height.
        Some(Piece { width: 0, height: 0, shape: trimmed_shape, offset_x: 0, offset_y: 0,  original_width, original_height, })
    } else {
        let piece = Piece { 
            width: trimmed_shape[0].len(), 
            height: trimmed_shape.len(), 
            shape: trimmed_shape,
            offset_x,
            offset_y,
            original_width,
            original_height,
        };
        Some(piece)
    }
}

pub fn trim_piece(raw_shape: &Vec<Vec<char>>) -> (Vec<Vec<char>>, usize, usize) {
    let height = raw_shape.len();
    let width = if height > 0 { raw_shape[0].len() } else { 0 };

    // Find min/max rows and columns containing 'O'
    let mut min_row = height;
    let mut max_row = 0;
    let mut min_col = width;
    let mut max_col = 0;

    for y in 0..height {
        for x in 0..width {
            if raw_shape[y][x] == 'O' {
                if y < min_row { min_row = y; }
                if y > max_row { max_row = y; }
                if x < min_col { min_col = x; }
                if x > max_col { max_col = x; }
            }
        }
    }

    // If no 'O' found, return an empty shape
    if min_row > max_row || min_col > max_col {
        return (Vec::new(), 0, 0);
    }

    // Build trimmed shape
    let mut trimmed = Vec::new();
    for y in min_row..=max_row {
        let mut row = Vec::new();
        for x in min_col..=max_col {
            row.push(raw_shape[y][x]);
        }
        trimmed.push(row);
    }
    (trimmed, min_row, min_col)
}

