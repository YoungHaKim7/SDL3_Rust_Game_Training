use sdl3::pixels::{Color, PixelFormatEnum};
use sdl3::rect::Rect;
use sdl3::render::{Renderer, RendererFlip};
use sdl3::sys::{SDL_Init, SDL_Quit};
use sdl3::video::{Window, WindowPos};
use std::time::Duration;

const SNAKE_GAME_WIDTH: i32 = 40;
const SNAKE_GAME_HEIGHT: i32 = 30;
const CELL_SIZE: i32 = 15;

struct AppState {
    window: Window,
    renderer: Renderer,
}

struct SnakeContext {
    head_x: i32,
    head_y: i32,
    body: Vec<(i32, i32)>,
    direction: (i32, i32),
    score: i32,
    game_over: bool,
    new_food: bool,
}

impl SnakeContext {
    fn new() -> Self {
        Self {
            head_x: SNAKE_GAME_WIDTH / 2,
            head_y: SNAKE_GAME_HEIGHT / 2,
            body: vec![],
            direction: (0, -1),
            score: 0,
            game_over: false,
            new_food: true,
        }
    }

    fn move_snake(&mut self) {
        // Implementation of snake movement logic
    }

    fn can_move_to(&self, x: i32, y: i32) -> bool {
        // Implementation of collision detection
        true
    }

    fn grow_body(&mut self) {
        // Implementation of body growth
    }

    fn check_food(&mut self) {
        // Implementation of food checking
    }

    fn generate_food(&mut self) {
        // Implementation of food generation
    }

    fn reset_game(&mut self) {
        // Implementation of game reset
    }
}

fn handle_key_event(scancode: sdl3::sys::SDL_Scancode, context: &mut SnakeContext) -> i32 {
    match scancode {
        sdl3::sys::SDL_SCANCODE_ESCAPE => 0,
        sdl3::sys::SDL_SCANCODE_W | sdl3::sys::SDL_SCANCODE_UP => {
            context.direction = (0, -1);
            1
        }
        sdl3::sys::SDL_SCANCODE_A | sdl3::sys::SDL_SCANCODE_LEFT => {
            context.direction = (-1, 0);
            1
        }
        sdl3::sys::SDL_SCANCODE_S | sdl3::sys::SDL_SCANCODE_DOWN => {
            context.direction = (0, 1);
            1
        }
        sdl3::sys::SDL_SCANCODE_D | sdl3::sys::SDL_SCANCODE_RIGHT => {
            context.direction = (1, 0);
            1
        }
        _ => 1,
    }
}

fn snake_run(context: &mut SnakeContext, renderer: &mut Renderer) -> bool {
    if context.game_over {
        context.reset_game();
    }

    context.move_snake();

    // Update score and check for food
    context.check_food();

    // Render the game
    renderer.set_draw_color(Color::RGB(0, 0, 0));
    renderer.clear();

    // Draw snake body
    renderer.set_draw_color(Color::RGB(255, 255, 255));
    for (x, y) in &context.body {
        let rect = Rect::new(
            x * CELL_SIZE,
            y * CELL_SIZE,
            CELL_SIZE as u32,
            CELL_SIZE as u32,
        );
        renderer.fill_rect(rect).unwrap();
    }

    // Draw snake head
    renderer.set_draw_color(Color::RGB(0, 255, 0));
    let head_rect = Rect::new(
        context.head_x * CELL_SIZE,
        context.head_y * CELL_SIZE,
        CELL_SIZE as u32,
        CELL_SIZE as u32,
    );
    renderer.fill_rect(head_rect).unwrap();

    // Update the screen
    renderer.present();

    true
}

fn main() {
    let sdl_context = sdl3::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    // Create window and renderer
    let window = video_subsystem
        .window("Snake Game", 800, 600)
        .position(WindowPos::Centered)
        .build()
        .unwrap();
    let mut renderer = window.renderer().accelerated().build().unwrap();

    // Set logical size for the renderer to match the grid size
    renderer
        .set_logical_size(SNAKE_GAME_WIDTH * CELL_SIZE, SNAKE_GAME_HEIGHT * CELL_SIZE)
        .unwrap();

    // Initialize game state
    let mut snake_context = SnakeContext::new();
    let mut app_state = AppState { window, renderer };

    'main_loop: loop {
        // Handle events
        for event in sdl3::event::poll() {
            match event {
                sdl3::event::Event::Quit { .. } => break 'main_loop,
                sdl3::event::Event::KeyDown { scancode, .. } => {
                    if handle_key_event(scancode, &mut snake_context) == 0 {
                        break 'main_loop;
                    }
                }
                _ => {}
            }
        }

        // Update and render
        if !snake_run(&mut snake_context, &mut app_state.renderer) {
            break 'main_loop;
        }

        // Cap frame rate
        std::thread::sleep(Duration::from_millis(100));
    }

    // Cleanup
    SDL_Quit();
}
