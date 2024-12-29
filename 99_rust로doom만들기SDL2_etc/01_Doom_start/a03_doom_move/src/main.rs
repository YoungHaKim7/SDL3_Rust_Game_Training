use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::time::Duration;

const GRID_SIZE: i32 = 20;
const CELL_SIZE: i32 = 20;
const SCREEN_WIDTH: i32 = GRID_SIZE * CELL_SIZE;
const SCREEN_HEIGHT: i32 = GRID_SIZE * CELL_SIZE;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem
        .window(
            "Grid Movement Game",
            SCREEN_WIDTH as u32,
            SCREEN_HEIGHT as u32,
        )
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;
    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let mut event_pump = sdl_context.event_pump()?;

    canvas.set_draw_color(Color::WHITE);
    canvas.clear();
    canvas.present();

    let mut player_pos = (10, 10); // Player starts at the center of the grid

    'running: loop {
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
                } => {
                    if player_pos.1 > 0 {
                        player_pos.1 -= 1;
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::S),
                    ..
                } => {
                    if player_pos.1 < GRID_SIZE - 1 {
                        player_pos.1 += 1;
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::A),
                    ..
                } => {
                    if player_pos.0 > 0 {
                        player_pos.0 -= 1;
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::D),
                    ..
                } => {
                    if player_pos.0 < GRID_SIZE - 1 {
                        player_pos.0 += 1;
                    }
                }
                _ => {}
            }
        }

        // Draw the grid and the player
        canvas.set_draw_color(Color::WHITE);
        canvas.clear();

        // Draw grid
        canvas.set_draw_color(Color::BLACK);
        for i in 0..=GRID_SIZE {
            canvas.draw_line(
                sdl2::rect::Point::new(i * CELL_SIZE, 0),
                sdl2::rect::Point::new(i * CELL_SIZE, SCREEN_HEIGHT),
            )?;
            canvas.draw_line(
                sdl2::rect::Point::new(0, i * CELL_SIZE),
                sdl2::rect::Point::new(SCREEN_WIDTH, i * CELL_SIZE),
            )?;
        }

        // Draw player
        canvas.set_draw_color(Color::BLUE);
        canvas.fill_rect(Rect::new(
            player_pos.0 * CELL_SIZE,
            player_pos.1 * CELL_SIZE,
            CELL_SIZE as u32,
            CELL_SIZE as u32,
        ))?;

        canvas.present();
        std::thread::sleep(Duration::from_millis(16)); // ~60 FPS
    }

    Ok(())
}
