// filepath: 
use std::fs::OpenOptions;
use std::io::Write;

mod input;
mod models;
mod logic;

use input::{get_player_info, read_map, read_piece};
use logic::find_best_move;

fn log(msg: &str) {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("/tmp/bot_debug.log")
        .unwrap();
    writeln!(file, "{}", msg).ok();
}

fn main() {
    log("Bot started");
    
    let player = match get_player_info() {
        Some(p) => {
            log(&format!("Got player info: p1={}{}, p2={}{}", p.p1, p.p1_alt, p.p2, p.p2_alt));
            p
        },
        None => {
            log("Failed to get player info");
            return;
        }
    };

    let mut turn = 0;
    loop {
        turn += 1;
        log(&format!("=== Turn {} ===", turn));
        log("Reading map...");
        
        let map = match read_map() {
            Some(m) => {
                log(&format!("Got map: {}x{}", m.width, m.height));
                m
            },
            None => {
                log(&format!("Failed to read map at turn {}, breaking", turn));
                break;
            }
        };

        log("Reading piece...");
        let piece = match read_piece() {
            Some(p) => {
                log(&format!("Got piece: {}x{} with {} cells", p.width, p.height, p.shape.iter().flatten().filter(|&&c| c == 'O').count()));
                p
            },
            None => {
                log(&format!("Failed to read piece at turn {}, breaking", turn));
                break;
            }
        };

        log("Finding best move...");
        let (y, x) = find_best_move(&map, &piece, &player);
        
        // Convert from trimmed piece coordinates to untrimmed piece coordinates
        let actual_y = y.saturating_sub(piece.offset_y);
        let actual_x = x.saturating_sub(piece.offset_x);
        
        log(&format!("Best move (trimmed): y={}, x={}", y, x));
        log(&format!("Offset: offset_y={}, offset_x={}", piece.offset_y, piece.offset_x));
        log(&format!("Best move (untrimmed): y={}, x={}", actual_y, actual_x));
        
        // Output format: X Y (column row)
        println!("{} {}", actual_x, actual_y);
        log(&format!("Output to game: {} {} (column={}, row={})", actual_x, actual_y, actual_x, actual_y));
    }
    log(&format!("Bot exiting after {} turns", turn));
}