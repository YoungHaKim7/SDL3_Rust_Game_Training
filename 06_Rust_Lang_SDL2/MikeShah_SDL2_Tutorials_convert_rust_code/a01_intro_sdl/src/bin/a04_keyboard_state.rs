extern crate sdl2;

use sdl2::{event::Event, keyboard::Keycode};

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let _window = video_subsystem
        .window("SDL2 Keyboard test", 800, 600)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut events = sdl_context.event_pump()?;

    'event_loop: loop {
        for event in events.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    println!("Quit");
                    break 'event_loop;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    println!("Escape");
                    break 'event_loop;
                }
                Event::KeyDown { .. } => {
                    println!("KeyDown Triggered");
                }
                Event::KeyUp { .. } => {
                    println!("KeyUp Triggered");
                }
                _ => {}
            }
        }
    }

    Ok(())
}
