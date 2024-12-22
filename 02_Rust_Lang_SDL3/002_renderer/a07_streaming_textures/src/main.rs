extern crate sdl3;

use sdl3::event::Event;
use sdl3::pixels::PixelFormatEnum;
use sdl3::rect::Rect;
use sdl3::render::FRect;
use sdl3::render::{Canvas, Texture};
use sdl3::surface::Surface;
use sdl3::video::Window;
use sdl3::Sdl;
use sdl3_sys::pixels::SDL_PixelFormat;
use std::error::Error;

const TEXTURE_SIZE: u32 = 150;
const WINDOW_WIDTH: u32 = 640;
const WINDOW_HEIGHT: u32 = 480;

struct AppState<'a> {
    canvas: Canvas<Window>,
    texture: Texture<'a>,
    running: bool,
}

impl<'a> AppState<'a> {
    fn new(sdl_context: &'a Sdl) -> Result<Self, Box<dyn Error>> {
        sdl_context.set_metadata(
            "Example Renderer Streaming Textures",
            "1.0",
            "com.example.renderer-streaming-textures",
        );

        let video_subsystem = sdl_context.video()?;
        let window = video_subsystem
            .window(
                "examples/renderer/streaming-textures",
                WINDOW_WIDTH,
                WINDOW_HEIGHT,
            )
            .position_centered()
            .build()?;

        let canvas = window.into_canvas();
        let texture_creator = canvas.texture_creator();
        let texture = texture_creator.create_texture(
            SDL_PixelFormat::RGB332;
            sdl3::render::TextureAccess::Streaming,
            TEXTURE_SIZE,
            TEXTURE_SIZE,
        )?;

        Ok(Self {
            canvas,
            texture,
            running: true,
        })
    }

    fn handle_event(&mut self, event: Event) {
        if let Event::Quit { .. } = event {
            self.running = false;
        }
    }

    fn iterate(&mut self) {
        let now = sdl3::timer::ticks() as u64;

        // Calculate color and position for the moving strip
        let direction = if (now % 2000) >= 1000 { 1.0 } else { -1.0 };
        let scale = (((now % 1000) as f32 - 500.0) / 500.0) * direction;

        // Lock the texture to update it
        if let Ok(mut surface) = self.texture.lock_to_surface(None) {
            let format = surface.pixel_format_enum();
            surface.fill_rect(None, format.map_rgb(0, 0, 0)).unwrap();

            let strip_height = TEXTURE_SIZE / 10;
            let y_position = ((TEXTURE_SIZE - strip_height) as f32 * ((scale + 1.0) / 2.0)) as i32;

            let strip_rect = Rect::new(0, y_position, TEXTURE_SIZE, strip_height);
            surface
                .fill_rect(Some(strip_rect), format.map_rgb(0, 255, 0))
                .unwrap();
        }

        // Clear the screen
        self.canvas
            .set_draw_color(sdl3::pixels::Color::RGB(66, 66, 66));
        self.canvas.clear();

        // Draw the updated texture
        let dst_rect = FRect::new(
            (WINDOW_WIDTH - TEXTURE_SIZE) as f32 / 2.0,
            (WINDOW_HEIGHT - TEXTURE_SIZE) as f32 / 2.0,
            TEXTURE_SIZE as f32,
            TEXTURE_SIZE as f32,
        );

        self.canvas
            .copy(&self.texture, None, Some(dst_rect))
            .unwrap();
        self.canvas.present();
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let sdl_context = sdl3::init()?;
    let mut app_state = AppState::new(&sdl_context)?;
    let mut event_pump = sdl_context.event_pump()?;

    while app_state.running {
        for event in event_pump.poll_iter() {
            app_state.handle_event(event);
        }

        app_state.iterate();
    }

    Ok(())
}
