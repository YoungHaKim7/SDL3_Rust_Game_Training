extern crate sdl2;

use std::fs::File;

use sdl2::event::Event;

use sdl2::image::{InitFlag, LoadTexture};
use sdl2::keyboard::Keycode;

fn main() -> Result<(), String> {
    // 파일 경로 테스트
    let file_path = "../assets/3D-Window_BBB_Blender_2.jpg";
    if File::open(file_path).is_err() {
        return Err(format!("파일을 열 수 없습니다: {}", file_path));
    }

    // SDL2 초기화
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG)?;
    let window = video_subsystem
        .window("rust-sdl2 demo: Video", 800, 600)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window
        .into_canvas()
        .software()
        .build()
        .map_err(|e| e.to_string())?;
    let texture_creator = canvas.texture_creator();
    let texture = texture_creator.load_texture(file_path)?;

    canvas.copy(&texture, None, None)?;
    canvas.present();

    'mainloop: loop {
        for event in sdl_context.event_pump()?.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Option::Some(Keycode::Escape),
                    ..
                } => break 'mainloop,
                _ => {}
            }
        }
    }

    Ok(())
}
