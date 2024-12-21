extern crate sdl3;

use sdl3::{
    event::Event, keyboard::Scancode, pixels::Color, rect::Rect, render::Canvas, video::Window, Sdl,
};
use std::time::{Duration, Instant};

const STEP_RATE_IN_MILLISECONDS: u64 = 125;
const SNAKE_BLOCK_SIZE_IN_PIXELS: u32 = 24;
const SNAKE_GAME_WIDTH: u32 = 24;
const SNAKE_GAME_HEIGHT: u32 = 18;
const SNAKE_MATRIX_SIZE: usize = (SNAKE_GAME_WIDTH * SNAKE_GAME_HEIGHT) as usize;
const SNAKE_WINDOW_WIDTH: u32 = SNAKE_BLOCK_SIZE_IN_PIXELS * SNAKE_GAME_WIDTH;
const SNAKE_WINDOW_HEIGHT: u32 = SNAKE_BLOCK_SIZE_IN_PIXELS * SNAKE_GAME_HEIGHT;

const SNAKE_CELL_MAX_BITS: u8 = 3;
const THREE_BITS: u16 = 0x7; // 3 bits mask

fn shift(x: u32, y: u32) -> usize {
    ((x + y * SNAKE_GAME_WIDTH) as usize) * SNAKE_CELL_MAX_BITS as usize
}

#[derive(Copy, Clone, PartialEq)]
enum SnakeCell {
    Nothing,
    SRight,
    SUp,
    SLeft,
    SDown,
    Food,
}

#[derive(Copy, Clone, PartialEq)]
enum SnakeDirection {
    Right,
    Up,
    Left,
    Down,
}

struct SnakeContext {
    cells: [u8; (SNAKE_MATRIX_SIZE * SNAKE_CELL_MAX_BITS as usize + 7) / 8],
    head_xpos: i32,
    head_ypos: i32,
    tail_xpos: i32,
    tail_ypos: i32,
    next_dir: SnakeDirection,
    inhibit_tail_step: u8,
    occupied_cells: u32,
}

impl SnakeContext {
    fn new() -> Self {
        Self {
            cells: [0; (SNAKE_MATRIX_SIZE * SNAKE_CELL_MAX_BITS as usize + 7) / 8],
            head_xpos: (SNAKE_GAME_WIDTH / 2) as i32,
            head_ypos: (SNAKE_GAME_HEIGHT / 2) as i32,
            tail_xpos: (SNAKE_GAME_WIDTH / 2) as i32,
            tail_ypos: (SNAKE_GAME_HEIGHT / 2) as i32,
            next_dir: SnakeDirection::Right,
            inhibit_tail_step: 1,
            occupied_cells: 0,
        }
    }

    fn snake_cell_at(&self, x: i32, y: i32) -> SnakeCell {
        let shift = shift(x as u32, y as u32);
        let byte_index = shift / 8;
        let bit_offset = (shift % 8) as u8;

        let range = ((self.cells[byte_index] as u16) >> bit_offset) & THREE_BITS;
        match range {
            1 => SnakeCell::SRight,
            2 => SnakeCell::SUp,
            3 => SnakeCell::SLeft,
            4 => SnakeCell::SDown,
            5 => SnakeCell::Food,
            _ => SnakeCell::Nothing,
        }
    }

    fn put_cell_at(&mut self, x: i32, y: i32, cell: SnakeCell) {
        let shift = shift(x as u32, y as u32);
        let byte_index = shift / 8;
        let bit_offset = (shift % 8) as u8;

        let mask = !(THREE_BITS << bit_offset);
        let value = (cell as u16 & THREE_BITS) << bit_offset;

        let current = self.cells[byte_index] as u16;
        self.cells[byte_index] = ((current & mask) | value) as u8;
    }

    fn initialize(&mut self) {
        self.cells.fill(0);
        self.head_xpos = (SNAKE_GAME_WIDTH / 2) as i32;
        self.head_ypos = (SNAKE_GAME_HEIGHT / 2) as i32;
        self.tail_xpos = self.head_xpos;
        self.tail_ypos = self.head_ypos;
        self.next_dir = SnakeDirection::Right;
        self.inhibit_tail_step = 1;
        self.occupied_cells = 0;
        self.put_cell_at(self.tail_xpos, self.tail_ypos, SnakeCell::SRight);
    }

    fn update(&mut self) {
        // Update snake position based on direction
        match self.next_dir {
            SnakeDirection::Right => self.head_xpos += 1,
            SnakeDirection::Left => self.head_xpos -= 1,
            SnakeDirection::Up => self.head_ypos -= 1,
            SnakeDirection::Down => self.head_ypos += 1,
        }

        // Wrap around screen edges
        self.head_xpos = (self.head_xpos + SNAKE_GAME_WIDTH as i32) % SNAKE_GAME_WIDTH as i32;
        self.head_ypos = (self.head_ypos + SNAKE_GAME_HEIGHT as i32) % SNAKE_GAME_HEIGHT as i32;

        // Update head position
        self.put_cell_at(
            self.head_xpos,
            self.head_ypos,
            match self.next_dir {
                SnakeDirection::Right => SnakeCell::SRight,
                SnakeDirection::Left => SnakeCell::SLeft,
                SnakeDirection::Up => SnakeCell::SUp,
                SnakeDirection::Down => SnakeCell::SDown,
            },
        );

        // Update tail position
        if self.inhibit_tail_step == 0 {
            let tail_cell = self.snake_cell_at(self.tail_xpos, self.tail_ypos);
            self.put_cell_at(self.tail_xpos, self.tail_ypos, SnakeCell::Nothing);
            match tail_cell {
                SnakeCell::SRight => self.tail_xpos += 1,
                SnakeCell::SLeft => self.tail_xpos -= 1,
                SnakeCell::SUp => self.tail_ypos -= 1,
                SnakeCell::SDown => self.tail_ypos += 1,
                _ => {}
            }
            self.tail_xpos = (self.tail_xpos + SNAKE_GAME_WIDTH as i32) % SNAKE_GAME_WIDTH as i32;
            self.tail_ypos = (self.tail_ypos + SNAKE_GAME_HEIGHT as i32) % SNAKE_GAME_HEIGHT as i32;
        } else {
            self.inhibit_tail_step -= 1;
        }

        // Check for collisions and food
        if self.snake_cell_at(self.head_xpos, self.head_ypos) == SnakeCell::Food {
            self.inhibit_tail_step += 1;
            self.occupied_cells += 1;
            // Add new food
            self.add_food();
        }
    }

    fn add_food(&mut self) {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        loop {
            let x = rng.gen_range(0..SNAKE_GAME_WIDTH as i32);
            let y = rng.gen_range(0..SNAKE_GAME_HEIGHT as i32);
            if self.snake_cell_at(x, y) == SnakeCell::Nothing {
                self.put_cell_at(x, y, SnakeCell::Food);
                break;
            }
        }
    }
}

struct AppState {
    context: SnakeContext,
    last_step: Instant,
}

impl AppState {
    fn new() -> Self {
        Self {
            context: SnakeContext::new(),
            last_step: Instant::now(),
        }
    }
}

fn main() -> Result<(), String> {
    let sdl_context = sdl3::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("Snake Game", SNAKE_WINDOW_WIDTH, SNAKE_WINDOW_HEIGHT)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas();
    let mut event_pump = sdl_context.event_pump()?;

    let mut app_state = AppState::new();
    app_state.context.initialize();
    app_state.context.add_food();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::KeyDown {
                    scancode: Some(scancode),
                    ..
                } => match scancode {
                    Scancode::Escape => break 'running,
                    Scancode::Right => app_state.context.next_dir = SnakeDirection::Right,
                    Scancode::Left => app_state.context.next_dir = SnakeDirection::Left,
                    Scancode::Up => app_state.context.next_dir = SnakeDirection::Up,
                    Scancode::Down => app_state.context.next_dir = SnakeDirection::Down,
                    _ => {}
                },
                _ => {}
            }
        }

        let now = Instant::now();
        if now.duration_since(app_state.last_step)
            >= Duration::from_millis(STEP_RATE_IN_MILLISECONDS)
        {
            app_state.context.update();
            app_state.last_step = now;
        }

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        for y in 0..SNAKE_GAME_HEIGHT {
            for x in 0..SNAKE_GAME_WIDTH {
                let cell = app_state.context.snake_cell_at(x as i32, y as i32);
                let color = match cell {
                    SnakeCell::Nothing => Color::RGB(0, 0, 0),
                    SnakeCell::Food => Color::RGB(255, 0, 0),
                    _ => Color::RGB(0, 255, 0),
                };
                canvas.set_draw_color(color);
                let rect = Rect::new(
                    (x * SNAKE_BLOCK_SIZE_IN_PIXELS) as i32,
                    (y * SNAKE_BLOCK_SIZE_IN_PIXELS) as i32,
                    SNAKE_BLOCK_SIZE_IN_PIXELS,
                    SNAKE_BLOCK_SIZE_IN_PIXELS,
                );
                canvas.fill_rect(rect)?;
            }
        }

        canvas.present();

        std::thread::sleep(Duration::from_millis(16));
    }

    Ok(())
}
