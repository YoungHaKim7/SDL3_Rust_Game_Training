use sdl2::pixels::Color;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::{Point, Rect};

use std::time::Duration;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("test", 800, 600)
        .resizable()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let texture_creator = canvas.texture_creator();

    let bytes = [
        0x00, 0x00, 0x00, // black pixel
        0xff, 0x00, 0x00, // red pixel
        0x00, 0xff, 0x00, // green pixel
        0x00, 0x00, 0xff,
    ]; // blue pixel
    let mut texture = texture_creator
        .create_texture_target(Some(PixelFormatEnum::RGB24), 4, 1)
        .unwrap();
    texture.update(None, &bytes, 4 * 3).unwrap();

    canvas.set_draw_color(Color::RGB(255, 255, 255));
    canvas.clear();
    canvas.present();

    loop {
        canvas.clear();

        canvas
            .copy_ex(
                &texture,
                None,
                Some(Rect::new(0, 0, 400, 100)),
                0.,
                None,
                false,
                false,
            )
            .unwrap();

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
