use sdl2::event::Event;
use sdl2::image::{InitFlag, LoadTexture};
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

fn main() -> Result<(), String> {
    // Initialize SDL2
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    // Create a window
    let window = video_subsystem
        .window("Rust SDL2 Window", 1280, 960)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    // Create a renderer
    let mut canvas = window
        .into_canvas()
        .accelerated()
        .build()
        .map_err(|e| e.to_string())?;

    // Initialize SDL2_image for loading textures
    let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG)?;

    // Load the texture from an image file
    let texture_creator = canvas.texture_creator();
    let texture = texture_creator
        .load_texture("./assets/kong.bmp")
        .map_err(|e| e.to_string())?;

    // Define a rectangle for rendering the texture
    let rectangle = Rect::new(50, 100, 200, 200);

    // Start the main loop
    let mut event_pump = sdl_context.event_pump()?;
    let mut game_is_running = true;

    while game_is_running {
        // Handle input
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => game_is_running = false,
                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => game_is_running = false,
                _ => {}
            }
        }

        // Clear and draw the screen
        canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
        canvas.clear();

        // Draw a white line
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.draw_line((5, 5), (100, 120))?;

        // Draw the texture onto the rectangle
        canvas.copy(&texture, None, rectangle)?;

        // Display the updated screen
        canvas.present();
    }

    Ok(())
}
