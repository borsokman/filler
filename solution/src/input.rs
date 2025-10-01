use std::io::{self, BufRead};
use crate::models::Player;

pub fn get_player_info() -> Option<Player> {
    // The game engine might send multiple lines before our player assignment.
    // We loop until we find the line that starts with "$$$ exec p".
    loop {
        let line = read_line()?;

       if line.starts_with("$$$ exec p") {
            if line.contains("p1") {
                // We are Player 1
                return Some(Player {
                    me: '@',
                    me_alt: 'a',
                    opponent: '$',
                    opponent_alt: 's',
                });
            } else if line.contains("p2") {
                // We are Player 2
                return Some(Player {
                    me: '$',
                    me_alt: 's',
                    opponent: '@',
                    opponent_alt: 'a',
                });
        }
    }
  }
}

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
