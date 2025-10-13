mod models;
mod input;
mod logic;

use crate::models::*;
use crate::input::*;
use crate::logic::*;

fn main() {
    let player = get_player_info().expect("Could not get player info");

    loop {
        // 1. Read Anfield from stdin (size + lines)
        let width = 20;  // example
        let height = 15; // example
        let mut anfield_lines = vec![];
        for _ in 0..height { 
            if let Some(line) = read_line() { anfield_lines.push(line); } 
        }
        let anfield = parse_anfield(&anfield_lines, width, height);

        // 2. Read Piece from stdin
        let piece_width = 4;  // example
        let piece_height = 1; // example
        let mut piece_lines = vec![];
        for _ in 0..piece_height {
            if let Some(line) = read_line() { piece_lines.push(line); }
        }
        let piece_cells = parse_piece(&piece_lines);

        // 3. Find best move
        let mv = find_best_move(&anfield, &piece_cells, &player);

        // 4. Output move
        if let Some((x, y)) = mv {
            println!("{} {}", x, y);
        } else {
            println!("0 0");
        }
    }
}
