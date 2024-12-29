extern crate sdl2;

use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::time::Duration;

fn main() -> Result<(), String> {
    // Initialize the SDL2 library
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    // Create a window
    let window = video_subsystem
        .window("Rust SDL2 Window", 640, 480)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    // Create a renderer
    let mut canvas = window
        .into_canvas()
        .accelerated()
        .build()
        .map_err(|e| e.to_string())?;

    // Define a rectangle
    let rectangle = Rect::new(50, 100, 20, 20);

    // Main application loop
    let mut event_pump = sdl_context.event_pump()?;
    let mut game_is_running = true;

    while game_is_running {
        // Handle input
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => game_is_running = false,
                _ => {}
            }
        }

        // Clear the screen with a black color
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        // Draw a white line
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.draw_line((5, 5), (100, 120))?;

        // Draw the rectangle
        canvas.draw_rect(rectangle)?;

        // Present the updated canvas
        canvas.present();

        // Limit the frame rate to ~60 FPS
        ::std::thread::sleep(Duration::from_millis(16));
    }

    Ok(())
}
