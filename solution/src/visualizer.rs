use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Duration;

#[derive(Clone, Debug)]
struct GameState {
    map: Vec<Vec<char>>,
    height: usize,
    width: usize,
}

fn parse_log_file(filename: &str) -> Vec<GameState> {
    let file = File::open(filename).expect("Cannot open log file");
    let reader = BufReader::new(file);
    let mut states = Vec::new();
    let lines: Vec<String> = reader.lines().filter_map(Result::ok).collect();
    
    let mut i = 0;
    while i < lines.len() {
        let line = &lines[i];
        if line.starts_with("Anfield") && line.contains(':') {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 3 {
                let height: usize = parts[2].trim_end_matches(':').parse().unwrap_or(0);
                let width: usize = parts[1].parse().unwrap_or(0);
                if height == 0 || width == 0 {
                    i += 1;
                    continue;
                }
                i += 2; // Skip column numbers line and move to first map line
                let mut map = Vec::new();
                for _ in 0..height {
                    if i >= lines.len() { break; }
                    let map_line = &lines[i];
                    if map_line.len() >= 4 && map_line.chars().take(3).all(|c| c.is_numeric()) {
                        let row_str = &map_line[4..];
                        let cells: Vec<char> = row_str.chars().collect();
                        if cells.len() == width {
                            map.push(cells);
                        }
                    }
                    i += 1;
                }
                if map.len() == height && !map.is_empty() {
                    states.push(GameState { map, height, width });
                }
            }
        }
        i += 1;
    }
    states
}

fn draw_state(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, state: &GameState, cell_size: u32, ) {
    canvas.set_draw_color(Color::RGB(20, 20, 30));
    canvas.clear();

    let pad = 1u32;
    for y in 0..state.height {
        for x in 0..state.width {
            let rect = Rect::new(
                (x as u32 * cell_size) as i32,
                (y as u32 * cell_size) as i32,
                cell_size.saturating_sub(pad),
                cell_size.saturating_sub(pad),
            );

            let color = match state.map[y][x] {
                '@' => Color::RGB(100, 150, 255),
                'a' => Color::RGB(80, 120, 200),
                '$' => Color::RGB(255, 100, 100),
                's' => Color::RGB(200, 80, 80),
                _ => Color::RGB(40, 40, 50),
            };
            canvas.set_draw_color(color);
            let _ = canvas.fill_rect(rect);
        }
    }
    canvas.present();
}

fn main() {

    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <log_file>", args[0]);
        std::process::exit(1);
    }

    let log_file = &args[1];
    let states = parse_log_file(log_file);
    if states.is_empty() {
        eprintln!("No game states found in log file");
        std::process::exit(1);
    }

    let first_state = &states[0];
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let bounds = video_subsystem.display_bounds(0).unwrap();
    let screen_w = bounds.width() as u32;
    let screen_h = bounds.height() as u32;

    let margin = 80u32;
    let avail_w = screen_w.saturating_sub(margin);
    let avail_h = screen_h.saturating_sub(margin);

    let cell_w = avail_w / first_state.width as u32;
    let cell_h = avail_h / first_state.height as u32;
    let cell_size = cell_w.min(cell_h).clamp(2, 60);

    let window_width = first_state.width as u32 * cell_size;
    let window_height = first_state.height as u32 * cell_size;

    let window = video_subsystem
        .window("Filler Visualizer", window_width, window_height)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut current_turn = 0;
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::KeyDown { keycode: Some(key), .. } => match key {
                    Keycode::Escape | Keycode::Q => break 'running,
                    Keycode::Right => {
                        if current_turn < states.len() - 1 {
                            current_turn += 1;
                        }
                    }
                    Keycode::Left => {
                        if current_turn > 0 {
                            current_turn -= 1;
                        }
                    }
                    Keycode::Up => {
                        current_turn = states.len() - 1;
                    }
                    Keycode::Down => {
                        current_turn = 0;
                    }
                    _ => {}
                },
                _ => {}
            }
            if let Event::Quit { .. } = event {
                break 'running;
            }
        }
        
        draw_state(&mut canvas, &states[current_turn], cell_size);
        std::thread::sleep(Duration::from_millis(16));
    }
}