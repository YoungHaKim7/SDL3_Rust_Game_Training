extern crate sdl2;

use sdl2::event::Event;
use sdl2::mouse::MouseButton;
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

    // Load a surface and set a color key
    let texture_creator = canvas.texture_creator();
    let mut surface = sdl2::surface::Surface::load_bmp("./assets/kong.bmp")?;
    surface.set_color_key(true, Color::RGB(255, 0, 255))?;
    let mut texture = texture_creator
        .create_texture_from_surface(&surface)
        .unwrap();

    // Define two rectangles
    let mut rectangle = Rect::new(50, 100, 200, 200);
    let mut rectangle2 = Rect::new(50, 100, 200, 200);

    // Main application loop
    let mut event_pump = sdl_context.event_pump()?;
    let mut game_is_running = true;

    while game_is_running {
        // Handle input
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => game_is_running = false,
                Event::MouseMotion { x, y, .. } => {
                    rectangle2.set_x(x);
                    rectangle2.set_y(y);
                }
                Event::MouseButtonDown { mouse_btn, .. } => match mouse_btn {
                    MouseButton::Left => {
                        texture.set_blend_mode(sdl2::render::BlendMode::Add);
                    }
                    MouseButton::Middle => {
                        texture.set_blend_mode(sdl2::render::BlendMode::Blend);
                    }
                    MouseButton::Right => {
                        texture.set_blend_mode(sdl2::render::BlendMode::Mod);
                    }
                    _ => {}
                },
                _ => {
                    texture.set_blend_mode(sdl2::render::BlendMode::None);
                }
            }
        }

        // Clear the screen with a blue color
        canvas.set_draw_color(Color::RGB(0, 0, 255));
        canvas.clear();

        // Draw a white line
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.draw_line((5, 5), (200, 220))?;

        // Draw the rectangles with the texture
        canvas.copy(&texture, None, rectangle)?;
        canvas.copy(&texture, None, rectangle2)?;

        // Present the updated canvas
        canvas.present();

        // Limit the frame rate to ~60 FPS
        ::std::thread::sleep(Duration::from_millis(16));
    }

    Ok(())
}
