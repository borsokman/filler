mod input;
mod models;
mod logic;

use input::{get_player_info, read_map, read_piece};
use logic::find_best_move;

fn main() {
    
    let player = match get_player_info() {
        Some(p) => {
            p
        },
        None => {
            return;
        }
    };

    loop {
        let map = match read_map() {
            Some(m) => {
                m
            },
            None => {
                break;
            }
        };

        let piece = match read_piece() {
            Some(p) => {
                p
            },
            None => {
                break;
            }
        };

        let (y, x) = find_best_move(&map, &piece, &player);
        // Convert from trimmed piece coordinates to untrimmed piece coordinates
        let actual_y = y as i32 - piece.offset_y as i32;
        let actual_x = x as i32- piece.offset_x as i32;
        
        // Output format: X Y (column row)
        println!("{} {}", actual_x, actual_y);
    }
}