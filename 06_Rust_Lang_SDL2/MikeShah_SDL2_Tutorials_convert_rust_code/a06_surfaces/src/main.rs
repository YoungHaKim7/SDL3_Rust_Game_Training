// use std::path::Path;
// use std::time::Duration;

// use sdl2::image::{self, InitFlag, LoadTexture};
// use sdl2::{event::Event, keyboard::Keycode};

// pub fn main() -> Result<(), String> {
//     let sdl_context = sdl2::init()?;
//     let video_subsystem = sdl_context.video()?;
//     let _image_context = image::init(InitFlag::PNG | InitFlag::JPG)?;

//     let window = video_subsystem
//         .window("rust-sdl2 demo: Video", 800, 600)
//         .position_centered()
//         .opengl()
//         .build()
//         .map_err(|e| e.to_string())?;

//     let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
//     let texture_creator = canvas.texture_creator();
//     let texture =
//         texture_creator.load_texture(Path::new("../assets/3D-Window_BBB_Blender_2.jpg"))?;

//     let mut event_pump = sdl_context.event_pump()?;

//     'running: loop {
//         for event in event_pump.poll_iter() {
//             match event {
//                 Event::Quit { .. }
//                 | Event::KeyDown {
//                     keycode: Some(Keycode::Escape),
//                     ..
//                 } => break 'running,
//                 _ => {}
//             }
//         }

//         canvas.clear();
//         canvas.copy(&texture, None, None)?;
//         canvas.present();
//         ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
//     }

//     Ok(())
// }

use std::path::Path;
use std::time::Duration;

use sdl2::image::{self, InitFlag, LoadTexture};
use sdl2::{event::Event, keyboard::Keycode};

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let image_context = image::init(InitFlag::PNG | InitFlag::JPG)?;

    let window = video_subsystem
        .window("rust-sdl2 demo: Video", 800, 600)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let texture_creator = canvas.texture_creator();

    // Use an absolute or dynamically computed path for the image file
    let image_path = Path::new("../assets/3D_Window_BBB_Blender_2.jpg");
    if !image_path.exists() {
        return Err(format!(
            "Image file not found: {}",
            image_path.to_string_lossy()
        ));
    }

    // let texture = texture_creator.load_texture(image_path)?;
    let texture = texture_creator.load_texture(image_path)?;
    canvas.copy(&texture, None, None)?;
    canvas.present();

    'running: loop {
        for event in sdl_context.event_pump()?.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        // canvas.clear();
        // canvas.copy(&texture, None, None)?;
        // canvas.present();
        // ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
    }

    Ok(())
}
