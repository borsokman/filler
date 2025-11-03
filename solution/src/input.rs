use std::io::{self, BufRead};
use crate::models::{Map, Piece, Player};
use std::fs::OpenOptions;
use std::io::Write;

fn log_input(msg: &str) {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("/tmp/bot_input.log")
        .unwrap();
    writeln!(file, "{}", msg).ok();
}

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
                log_input("EOF reached");
                return None; // End of file
            }
            Ok(n) => {
                log_input(&format!("Read {} bytes: {:?}", n, buffer));
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
                    log_input(&format!("Returning: {:?}", trimmed));
                    return Some(trimmed.to_string());
                }
                log_input("Empty line, continuing...");
                // If empty, continue the loop to read the next line
            }
            Err(e) => {
                log_input(&format!("Error reading: {:?}", e));
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
    log_input("read_map() called");
    // Keep reading until we find the "Anfield" line
    let line = loop {
        let l = read_line()?;
        log_input(&format!("Looking for Anfield, got: {:?}", l));
        if l.contains("Anfield") {
            log_input("Found Anfield line");
            break l;
        }
    };
    
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() < 3 {
        log_input("Not enough parts in Anfield line");
        return None;
    }
    let width: usize = parts[1].parse().ok()?;
    let height: usize = parts[2].trim_end_matches(':').parse().ok()?;
    log_input(&format!("Map dimensions: {}x{}", width, height));

    // Skip the column header line
    read_line()?;

    let mut grid = Vec::with_capacity(height);
    for i in 0..height {
        let row_line = read_line()?;
        log_input(&format!("Row {}: {:?}", i, row_line));
        // Skip the first 4 characters (row number and space)
        if row_line.len() < 4 {
            log_input("Row too short");
            return None;
        }
        let row: Vec<char> = row_line.chars().skip(4).collect();
        if row.len() != width {
            log_input(&format!("Row width mismatch: got {}, expected {}", row.len(), width));
            return None;
        }
        grid.push(row);
    }

    log_input("Map read successfully");
    Some(Map { width, height, grid })
}

// Reads the piece, trims it to its minimal bounding box, and stores the trimmed shape.
pub fn read_piece() -> Option<Piece> {
    log_input("read_piece() called");
    // Keep reading until we find the "Piece" line
    let line = loop {
        let l = read_line()?;
        log_input(&format!("Looking for Piece, got: {:?}", l));
        if l.starts_with("Piece") {
            log_input("Found Piece line");
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
        log_input(&format!("Raw piece row: {:?}", row));
        raw_shape.push(row);
    }

    let (trimmed_shape, offset_y, offset_x) = trim_piece(&raw_shape);
    
    log_input(&format!("Trimmed piece shape with offset_y={}, offset_x={}:", offset_y, offset_x));
    for row in &trimmed_shape {
        log_input(&format!("  {:?}", row.iter().collect::<String>()));
    }

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
        log_input(&format!("Returning piece: {}x{}", piece.width, piece.height));
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

