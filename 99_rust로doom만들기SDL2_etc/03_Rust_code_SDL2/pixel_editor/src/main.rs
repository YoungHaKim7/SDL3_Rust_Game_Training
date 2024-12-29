extern crate sdl2;

use sdl2::{
    event::Event,
    mouse::MouseButton,
    pixels::Color,
    render::UpdateTextureError,
    render::{Texture, TextureCreator},
    video::WindowContext,
};

use std::time::Duration;

const SCREEN_WIDTH: u32 = 640;
const SCREEN_HEIGHT: u32 = 480;
const INK_SIZE: i32 = 5;

fn create_texture<'a>(
    texture_creator: &'a TextureCreator<WindowContext>,
) -> Result<Texture<'a>, String> {
    texture_creator
        .create_texture_target(None, SCREEN_WIDTH, SCREEN_HEIGHT)
        .map_err(|e| e.to_string())
}

fn update_texture(texture: &mut Texture, pixels: &[u8]) -> Result<(), UpdateTextureError> {
    texture.update(None, pixels, (SCREEN_WIDTH * 4) as usize)
}

fn render(
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    texture: &Texture,
) -> Result<(), String> {
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.copy(texture, None, None)?;
    canvas.present();
    Ok(())
}

fn draw_pixels(pixels: &mut [u8], mouse_x: i32, mouse_y: i32) {
    for y in (mouse_y - INK_SIZE / 2)..=(mouse_y + INK_SIZE / 2) {
        for x in (mouse_x - INK_SIZE / 2)..=(mouse_x + INK_SIZE / 2) {
            if x >= 0 && x < SCREEN_WIDTH as i32 && y >= 0 && y < SCREEN_HEIGHT as i32 {
                let offset = ((y as u32 * SCREEN_WIDTH + x as u32) * 4) as usize;
                pixels[offset] = 0;
                pixels[offset + 1] = 0;
                pixels[offset + 2] = 0;
                pixels[offset + 3] = 255;
            }
        }
    }
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("SDL2 Pixel Drawing", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let texture_creator: TextureCreator<WindowContext> = canvas.texture_creator();
    let mut texture = create_texture(&texture_creator)?;

    let mut pixels = vec![255u8; (SCREEN_WIDTH * SCREEN_HEIGHT * 4) as usize];

    let mut event_pump = sdl_context.event_pump()?;
    let mut left_mouse_button_down = false;
    let mut quit = false;

    while !quit {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => quit = true,
                Event::MouseButtonUp { mouse_btn, .. } => {
                    if mouse_btn == MouseButton::Left {
                        left_mouse_button_down = false;
                    }
                }
                Event::MouseButtonDown { mouse_btn, .. } => {
                    if mouse_btn == MouseButton::Left {
                        left_mouse_button_down = true;
                    }
                }
                Event::MouseMotion { x, y, .. } => {
                    if left_mouse_button_down {
                        draw_pixels(&mut pixels, x, y);
                    }
                }
                _ => {}
            }
        }

        let _ = update_texture(&mut texture, &pixels);
        render(&mut canvas, &texture)?;
        std::thread::sleep(Duration::from_millis(16));
    }

    Ok(())
}
