use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::time::{Duration, Instant};

const SCREEN_WIDTH: u32 = 640;
const SCREEN_HEIGHT: u32 = 480;
const CELL_SIZE: i32 = 20; // Size of the grid cells
const PLAYER_SIZE: i32 = 10; // Size of the player square
const PLAYER_SPEED: f32 = 100.0; // Player movement speed (pixels per second)
const TURN_SPEED: f32 = 3.0; // Player turning speed (radians per second)

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem
        .window("Doom Movement", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;
    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let mut event_pump = sdl_context.event_pump()?;

    let mut player_pos: (f32, f32) = (320.0, 240.0); // Player's initial position (center of the screen)
    let mut player_angle: f32 = 0.0; // Player's facing angle (in radians)

    let mut keys = [false; 4]; // W, A, S, D
    let mut last_time = Instant::now();

    'running: loop {
        let current_time = Instant::now();
        let delta_time = current_time.duration_since(last_time).as_secs_f32();
        last_time = current_time;

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::W),
                    ..
                } => keys[0] = true,
                Event::KeyDown {
                    keycode: Some(Keycode::A),
                    ..
                } => keys[1] = true,
                Event::KeyDown {
                    keycode: Some(Keycode::S),
                    ..
                } => keys[2] = true,
                Event::KeyDown {
                    keycode: Some(Keycode::D),
                    ..
                } => keys[3] = true,
                Event::KeyUp {
                    keycode: Some(Keycode::W),
                    ..
                } => keys[0] = false,
                Event::KeyUp {
                    keycode: Some(Keycode::A),
                    ..
                } => keys[1] = false,
                Event::KeyUp {
                    keycode: Some(Keycode::S),
                    ..
                } => keys[2] = false,
                Event::KeyUp {
                    keycode: Some(Keycode::D),
                    ..
                } => keys[3] = false,
                _ => {}
            }
        }

        // Movement logic
        let mut dx = 0.0;
        let mut dy = 0.0;

        if keys[3] {
            // Move up
            dx += player_angle.cos();
            dy += player_angle.sin();
        }
        if keys[1] {
            // Move down
            dx -= player_angle.cos();
            dy -= player_angle.sin();
        }
        if keys[2] {
            // Strafe up
            dx -= player_angle.sin();
            dy += player_angle.cos();
        }
        if keys[0] {
            // Strafe down
            dx += player_angle.sin();
            dy -= player_angle.cos();
        }

        // Normalize direction vector and scale by speed
        let length = (dx * dx + dy * dy).sqrt();
        if length > 0.0 {
            dx /= length;
            dy /= length;
        }

        player_pos.0 += dx * PLAYER_SPEED * delta_time;
        player_pos.1 += dy * PLAYER_SPEED * delta_time;

        // Render the scene
        canvas.set_draw_color(Color::BLACK);
        canvas.clear();

        // Draw grid
        canvas.set_draw_color(Color::GRAY);
        for x in (0..SCREEN_WIDTH as i32).step_by(CELL_SIZE as usize) {
            canvas.draw_line((x, 0), (x, SCREEN_HEIGHT as i32))?;
        }
        for y in (0..SCREEN_HEIGHT as i32).step_by(CELL_SIZE as usize) {
            canvas.draw_line((0, y), (SCREEN_WIDTH as i32, y))?;
        }

        // Draw player
        canvas.set_draw_color(Color::BLUE);
        canvas.fill_rect(Rect::new(
            player_pos.0 as i32 - PLAYER_SIZE / 2,
            player_pos.1 as i32 - PLAYER_SIZE / 2,
            PLAYER_SIZE as u32,
            PLAYER_SIZE as u32,
        ))?;

        canvas.present();

        // Sleep to maintain a stable frame rate
        ::std::thread::sleep(Duration::from_millis(16));
    }

    Ok(())
}
