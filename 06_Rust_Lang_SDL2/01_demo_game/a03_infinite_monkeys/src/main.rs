use rand::Rng;
use sdl2::{
    event::Event,
    keyboard::{Keycode, Scancode},
    pixels::Color,
    rect::Rect,
    render::Canvas,
    video::Window,
    Sdl,
};

// Constants
const MAX_MONKEY_SCANCODE: Scancode = Scancode::F12;
const MIN_MONKEY_SCANCODE: Scancode = Scancode::A;
const DEFAULT_TEXT: &str = "..."; // Replace with Jabberwocky text
const FONT_CHARACTER_SIZE: u32 = 8;

// Struct Definitions
// struct Line {
//     text: Vec<u32>,
//     length: usize,
// }

// struct AppState {
//     cols: usize,
//     monkey_chars: Vec<char>,
// }

// Define the AppState struct with the required fields
struct AppState {
    cols: usize,
    rows: usize,
    lines: Vec<Vec<char>>,
    monkey_chars: Vec<char>,
    text: String,
    progress: usize,
    canvas: sdl2::render::Canvas<sdl2::video::Window>,
}

impl AppState {
    fn new(
        cols: usize,
        rows: usize,
        text: String,
        canvas: sdl2::render::Canvas<sdl2::video::Window>,
    ) -> Self {
        Self {
            cols,
            rows,
            lines: vec![vec![' '; cols]; rows],
            monkey_chars: vec![' '; cols],
            text,
            progress: 0,
            canvas,
        }
    }
}

// Define the Line struct
struct Line {
    text: Vec<char>,
    length: usize,
}

impl Line {
    fn new(text: Vec<char>) -> Self {
        Self {
            text,
            length: text.len(),
        }
    }
}

// fn main() {
//     // Initialize the SDL2 library
//     let sdl_context = sdl2::init().unwrap();
//     let video_subsystem = sdl_context.video().unwrap();

//     // Create a window and canvas
//     let window = video_subsystem
//         .window("Monkey Typing", 800, 600)
//         .build()
//         .unwrap();
//     let mut canvas = window.into_canvas().build().unwrap();

//     // Create an AppState instance
//     let mut app_state = AppState::new(80, 24, "Hello, World!".to_string(), canvas);

//     // Main loop
//     loop {
//         // Handle events
//         for event in sdl_context.event_pump().unwrap().poll_iter() {
//             match event {
//                 sdl2::event::Event::KeyDown { keycode, .. } => {
//                     // Convert the Keycode to a Scancode
//                     let scancode = sdl2::keyboard::Scancode::from_keycode(keycode).unwrap();

//                     // Check if the Scancode is within the valid range
//                     if let Some(scancode) = scancode {
//                         // Convert the Scancode to a Keycode
//                         let keycode = sdl2::keyboard::Keycode::from_scancode(scancode).unwrap();

//                         // Get the character corresponding to the Keycode
//                         let ch = match keycode {
//                             sdl2::keyboard::Keycode::Space => ' ',
//                             _ => {
//                                 // Convert the Keycode to a character
//                                 let ch = keycode as u8 as char;
//                                 ch
//                             }
//                         };

//                         // Update the AppState
//                         app_state.monkey_chars[app_state.progress % app_state.cols] = ch;
//                         app_state.progress += 1;
//                     }
//                 }
//                 _ => {}
//             }
//         }

//         // Update the canvas
//         canvas.clear();
//         canvas.present();
//     }
// }

// Helper Functions
fn free_lines(app_state: &mut AppState) {
    app_state.lines.clear();
    app_state.monkey_chars.text.clear();
}

fn on_window_size_changed(app_state: &mut AppState) {
    let (w, h) = app_state.canvas.output_size().unwrap();

    free_lines(app_state);

    app_state.rows = (h / FONT_CHARACTER_SIZE) as usize - 4;
    app_state.cols = (w / FONT_CHARACTER_SIZE) as usize;

    if app_state.rows > 0 && app_state.cols > 0 {
        app_state.lines = (0..app_state.rows)
            .map(|_| Line {
                text: vec![0; app_state.cols],
                length: 0,
            })
            .collect();

        app_state.monkey_chars = Line {
            text: vec![b' ' as u32; app_state.cols],
            length: app_state.cols,
        };
    }
}

fn can_monkey_type(ch: u32) -> bool {
    let scancode = sdl2::keyboard::Scancode::from_keycode(Keycode::from_u32(ch as u8));
    if let Some(scancode) = scancode {
        if scancode < MIN_MONKEY_SCANCODE || scancode > MAX_MONKEY_SCANCODE {
            return false;
        }
    } else {
        return false;
    }

    true
}

fn advance_row(app_state: &mut AppState) {
    let row = &mut app_state.lines[app_state.rows % app_state.rows];
    row.length = 0;
}

fn add_monkey_char(app_state: &mut AppState, monkey: isize, ch: u32) {
    if monkey >= 0 {
        app_state.monkey_chars.text[(monkey as usize % app_state.cols)] = ch;
    }

    if ch == '\n' as u32 {
        advance_row(app_state);
    } else {
        let line = &mut app_state.lines[app_state.rows % app_state.rows];
        line.text[line.length] = ch;
        line.length += 1;
        if line.length == app_state.cols {
            advance_row(app_state);
        }
    }
}

fn get_next_char(app_state: &mut AppState) -> Option<u32> {
    while app_state.progress < app_state.text.len() {
        let ch = app_state.text.as_bytes()[app_state.progress] as u32;
        app_state.progress += 1;
        if can_monkey_type(ch) {
            return Some(ch);
        } else {
            add_monkey_char(app_state, -1, ch);
        }
    }
    None
}

fn monkey_play() -> u32 {
    let range = (MAX_MONKEY_SCANCODE as usize - MIN_MONKEY_SCANCODE as usize + 1);
    let scancode = MIN_MONKEY_SCANCODE as usize + (rand::random::<usize>() % range);
    Keycode::from_scancode(sdl2::keyboard::Scancode::from(scancode)).unwrap_or(Keycode::Space)
        as u32
}

fn display_line(canvas: &mut Canvas<Window>, x: f32, y: f32, line: &Line) {
    // Placeholder implementation for text rendering
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init().map_err(|e| e.to_string())?;
    let video_subsystem = sdl_context.video().map_err(|e| e.to_string())?;

    let window = video_subsystem
        .window("Infinite Monkeys", 800, 600)
        .position_centered()
        .resizable()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window
        .into_canvas()
        .accelerated()
        .build()
        .map_err(|e| e.to_string())?;

    let mut rng = rand::thread_rng();
    let range = MAX_MONKEY_SCANCODE as usize - MIN_MONKEY_SCANCODE as usize + 1;

    let cols = 80; // Set a suitable value for the number of columns.
    let mut app_state = AppState {
        cols,
        monkey_chars: vec![' '; cols],
    };

    let mut event_pump = sdl_context.event_pump().map_err(|e| e.to_string())?;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                _ => {}
            }
        }

        // Simulate random monkey typing
        let scancode = MIN_MONKEY_SCANCODE as usize + (rng.gen::<usize>() % range);
        if let Some(scancode) = Scancode::from_u32(scancode as u32) {
            if scancode >= MIN_MONKEY_SCANCODE && scancode <= MAX_MONKEY_SCANCODE {
                if let Some(keycode) = Keycode::from_scancode(scancode) {
                    let ch = keycode as u8 as char;
                    let idx = rng.gen::<usize>() % app_state.cols;
                    app_state.monkey_chars[idx] = ch;
                }
            }
        }

        // Clear the canvas
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        // Draw the text or content on the canvas here

        canvas.present();
    }

    Ok(())
}
