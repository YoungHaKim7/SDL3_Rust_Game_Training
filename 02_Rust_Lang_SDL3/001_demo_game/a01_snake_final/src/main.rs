use sdl3::{
    event::Event, keyboard::Scancode, pixels::Color, rect::Rect, render::Canvas, video::Window,
    EventPump, Sdl,
};
use std::time::{Duration, Instant};

const STEP_RATE_IN_MILLISECONDS: u64 = 125;
const SNAKE_BLOCK_SIZE_IN_PIXELS: u32 = 24;
const SNAKE_GAME_WIDTH: usize = 24;
const SNAKE_GAME_HEIGHT: usize = 18;
const SNAKE_MATRIX_SIZE: usize = SNAKE_GAME_WIDTH * SNAKE_GAME_HEIGHT;

#[derive(Clone, Copy, PartialEq)]
enum SnakeCell {
    Nothing,
    SRight,
    SUp,
    SLeft,
    SDown,
    Food,
}

impl SnakeCell {
    fn from_u8(value: u8) -> Self {
        match value {
            0 => SnakeCell::Nothing,
            1 => SnakeCell::SRight,
            2 => SnakeCell::SUp,
            3 => SnakeCell::SLeft,
            4 => SnakeCell::SDown,
            5 => SnakeCell::Food,
            _ => SnakeCell::Nothing,
        }
    }

    fn to_u8(self) -> u8 {
        match self {
            SnakeCell::Nothing => 0,
            SnakeCell::SRight => 1,
            SnakeCell::SUp => 2,
            SnakeCell::SLeft => 3,
            SnakeCell::SDown => 4,
            SnakeCell::Food => 5,
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
enum SnakeDirection {
    Right,
    Up,
    Left,
    Down,
}

struct SnakeContext {
    cells: Vec<u8>,
    head_xpos: i8,
    head_ypos: i8,
    tail_xpos: i8,
    tail_ypos: i8,
    next_dir: SnakeDirection,
    inhibit_tail_step: u8,
    occupied_cells: usize,
}

impl SnakeContext {
    fn new() -> Self {
        Self {
            cells: vec![0; SNAKE_MATRIX_SIZE],
            head_xpos: (SNAKE_GAME_WIDTH / 2) as i8,
            head_ypos: (SNAKE_GAME_HEIGHT / 2) as i8,
            tail_xpos: (SNAKE_GAME_WIDTH / 2) as i8,
            tail_ypos: (SNAKE_GAME_HEIGHT / 2) as i8,
            next_dir: SnakeDirection::Right,
            inhibit_tail_step: 0,
            occupied_cells: 0,
        }
    }

    fn initialize(&mut self) {
        self.cells.fill(0);
        self.head_xpos = (SNAKE_GAME_WIDTH / 2) as i8;
        self.head_ypos = (SNAKE_GAME_HEIGHT / 2) as i8;
        self.tail_xpos = (SNAKE_GAME_WIDTH / 2) as i8;
        self.tail_ypos = (SNAKE_GAME_HEIGHT / 2) as i8;
        self.next_dir = SnakeDirection::Right;
        self.inhibit_tail_step = 0;
        self.occupied_cells = 4;

        self.set_cell(self.tail_xpos, self.tail_ypos, SnakeCell::SRight);

        for _ in 0..4 {
            self.add_food();
        }
    }

    fn step(&mut self) {
        // Calculate new head position
        let (mut new_head_x, mut new_head_y) = (self.head_xpos, self.head_ypos);
        match self.next_dir {
            SnakeDirection::Right => new_head_x += 1,
            SnakeDirection::Left => new_head_x -= 1,
            SnakeDirection::Up => new_head_y -= 1,
            SnakeDirection::Down => new_head_y += 1,
        }

        // Wrap around if the snake goes out of bounds
        new_head_x = (new_head_x + SNAKE_GAME_WIDTH as i8) % SNAKE_GAME_WIDTH as i8;
        new_head_y = (new_head_y + SNAKE_GAME_HEIGHT as i8) % SNAKE_GAME_HEIGHT as i8;

        // Check what is at the new head position
        let next_cell = self.get_cell(new_head_x, new_head_y);

        // Handle collisions
        match next_cell {
            SnakeCell::Nothing => {
                // Move the tail if no growth is needed
                if self.inhibit_tail_step == 0 {
                    let tail_dir = self.get_cell(self.tail_xpos, self.tail_ypos);
                    self.set_cell(self.tail_xpos, self.tail_ypos, SnakeCell::Nothing);

                    match tail_dir {
                        SnakeCell::SRight => self.tail_xpos += 1,
                        SnakeCell::SLeft => self.tail_xpos -= 1,
                        SnakeCell::SUp => self.tail_ypos -= 1,
                        SnakeCell::SDown => self.tail_ypos += 1,
                        _ => {}
                    }
                } else {
                    self.inhibit_tail_step -= 1;
                }
            }
            SnakeCell::Food => {
                // Grow the snake when eating food
                self.inhibit_tail_step += 1;
                self.occupied_cells += 1;
                self.add_food();
            }
            _ => {
                // Snake collides with itself (game over logic could go here)
                self.initialize();
                return;
            }
        }

        // Move the head to the new position
        self.set_cell(
            self.head_xpos,
            self.head_ypos,
            match self.next_dir {
                SnakeDirection::Right => SnakeCell::SRight,
                SnakeDirection::Left => SnakeCell::SLeft,
                SnakeDirection::Up => SnakeCell::SUp,
                SnakeDirection::Down => SnakeCell::SDown,
            },
        );

        self.head_xpos = new_head_x;
        self.head_ypos = new_head_y;
    }

    fn set_cell(&mut self, x: i8, y: i8, cell: SnakeCell) {
        let index = (y as usize * SNAKE_GAME_WIDTH + x as usize) % SNAKE_MATRIX_SIZE;
        self.cells[index] = cell.to_u8();
    }

    fn get_cell(&self, x: i8, y: i8) -> SnakeCell {
        let index = (y as usize * SNAKE_GAME_WIDTH + x as usize) % SNAKE_MATRIX_SIZE;
        SnakeCell::from_u8(self.cells[index])
    }

    fn add_food(&mut self) {
        loop {
            let x = rand::random::<usize>() % SNAKE_GAME_WIDTH;
            let y = rand::random::<usize>() % SNAKE_GAME_HEIGHT;

            if self.get_cell(x as i8, y as i8) == SnakeCell::Nothing {
                self.set_cell(x as i8, y as i8, SnakeCell::Food);
                break;
            }
        }
    }
}

struct AppState {
    canvas: Canvas<Window>,
    event_pump: EventPump,
    snake_ctx: SnakeContext,
    last_step: Instant,
}

impl AppState {
    fn new(sdl: &Sdl) -> Self {
        let video_subsystem = sdl.video().unwrap();
        let window = video_subsystem
            .window(
                "Snake Game",
                SNAKE_GAME_WIDTH as u32 * SNAKE_BLOCK_SIZE_IN_PIXELS,
                SNAKE_GAME_HEIGHT as u32 * SNAKE_BLOCK_SIZE_IN_PIXELS,
            )
            .position_centered()
            .build()
            .unwrap();

        let canvas = window.into_canvas();
        let event_pump = sdl.event_pump().unwrap();
        let snake_ctx = SnakeContext::new();

        AppState {
            canvas,
            event_pump,
            snake_ctx,
            last_step: Instant::now(),
        }
    }
    pub fn game_loop(&mut self) {
        let mut last_step = Instant::now();

        'running: loop {
            // Collect all events first to avoid borrowing issues
            let events: Vec<_> = self.event_pump.poll_iter().collect();

            for event in events {
                match event {
                    Event::Quit { .. } => break 'running,
                    Event::KeyDown {
                        scancode: Some(key),
                        ..
                    } => {
                        self.handle_key_event(key);
                    }
                    _ => {}
                }
            }

            // Game step logic
            if last_step.elapsed() >= Duration::from_millis(STEP_RATE_IN_MILLISECONDS) {
                self.snake_ctx.step();

                // Reset the timer
                last_step = Instant::now();
            }

            // Render the game
            self.render();
        }
    }
    // fn run(&mut self) {
    //     'running: loop {
    //         for event in self.event_pump.poll_iter() {
    //             match event {
    //                 Event::Quit { .. } => break 'running,
    //                 Event::KeyDown {
    //                     scancode: Some(key),
    //                     ..
    //                 } => {
    //                     self.handle_key_event(key);
    //                 }
    //                 _ => {}
    //             }
    //         }

    //         if self.last_step.elapsed() >= Duration::from_millis(STEP_RATE_IN_MILLISECONDS) {
    //             self.snake_ctx.initialize(); // Call your logic
    //             self.last_step = Instant::now();
    //         }

    //         self.render();
    //     }
    // }

    fn handle_key_event(&mut self, key: Scancode) {
        match key {
            Scancode::Right => self.snake_ctx.next_dir = SnakeDirection::Right,
            Scancode::Up => self.snake_ctx.next_dir = SnakeDirection::Up,
            Scancode::Left => self.snake_ctx.next_dir = SnakeDirection::Left,
            Scancode::Down => self.snake_ctx.next_dir = SnakeDirection::Down,
            _ => {}
        }
    }

    fn render(&mut self) {
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();

        for y in 0..SNAKE_GAME_HEIGHT {
            for x in 0..SNAKE_GAME_WIDTH {
                let cell = self.snake_ctx.get_cell(x as i8, y as i8);

                if cell != SnakeCell::Nothing {
                    let color = match cell {
                        SnakeCell::Food => Color::RGB(80, 80, 255),
                        _ => Color::RGB(0, 128, 0),
                    };

                    self.canvas.set_draw_color(color);
                    self.canvas
                        .fill_rect(Rect::new(
                            (x as u32 * SNAKE_BLOCK_SIZE_IN_PIXELS) as i32,
                            (y as u32 * SNAKE_BLOCK_SIZE_IN_PIXELS) as i32,
                            SNAKE_BLOCK_SIZE_IN_PIXELS,
                            SNAKE_BLOCK_SIZE_IN_PIXELS,
                        ))
                        .unwrap();
                }
            }
        }

        self.canvas.present();
    }
}

fn main() {
    let sdl_context = sdl3::init().unwrap();
    let mut app_state = AppState::new(&sdl_context);
    app_state.game_loop();
}
