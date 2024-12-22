extern crate sdl3;

use sdl3::event::Event;
use sdl3::gfx::primitives::*;
use sdl3::pixels::Color;
use sdl3::render::{Canvas, Renderer};
use sdl3::surface::Surface;
use sdl3::video::Window;
use sdl3::Sdl;
use std::error::Error;

const WINDOW_WIDTH: u32 = 640;
const WINDOW_HEIGHT: u32 = 480;

struct AppState<'a> {
    canvas: Canvas<Window>,
    renderer: Renderer,
    texture: Option<sdl3::render::Texture<'a>>,
    texture_width: u32,
    texture_height: u32,
    running: bool,
}

impl<'a> AppState<'a> {
    fn new(sdl_context: &'a Sdl) -> Result<Self, Box<dyn Error>> {
        let video_subsystem = sdl_context.video()?;
        let window = video_subsystem
            .window("Example Renderer Geometry", WINDOW_WIDTH, WINDOW_HEIGHT)
            .position_centered()
            .build()?;

        let mut canvas = window.into_canvas().build()?;
        let texture_creator = canvas.texture_creator();

        // Load the BMP file into a surface
        let base_path = sdl3::filesystem::base_path();
        let bmp_path = format!("{}/sample.bmp", base_path);
        let surface = Surface::load_bmp(bmp_path)?;

        let texture_width = surface.width();
        let texture_height = surface.height();

        // Convert the surface to a texture
        let texture = texture_creator.create_texture_from_surface(&surface)?;

        Ok(Self {
            canvas,
            renderer: canvas.renderer(),
            texture: Some(texture),
            texture_width,
            texture_height,
            running: true,
        })
    }

    fn handle_event(&mut self, event: Event) {
        if let Event::Quit { .. } = event {
            self.running = false;
        }
    }

    fn iterate(&mut self) {
        let now = sdl3::timer::ticks();

        // Calculate scale and size
        let direction = if (now % 2000) >= 1000 { 1.0 } else { -1.0 };
        let scale = ((now % 1000) as f32 - 500.0) / 500.0 * direction;
        let size = 200.0 + (200.0 * scale);

        // Clear the canvas
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();

        // Define vertices for a growing and shrinking triangle
        let vertices = [
            Vertex {
                position: (
                    WINDOW_WIDTH as f32 / 2.0,
                    (WINDOW_HEIGHT as f32 - size) / 2.0,
                ),
                color: Color::RGBA(255, 0, 0, 255),
                tex_coord: None,
            },
            Vertex {
                position: (
                    (WINDOW_WIDTH as f32 + size) / 2.0,
                    (WINDOW_HEIGHT as f32 + size) / 2.0,
                ),
                color: Color::RGBA(0, 255, 0, 255),
                tex_coord: None,
            },
            Vertex {
                position: (
                    (WINDOW_WIDTH as f32 - size) / 2.0,
                    (WINDOW_HEIGHT as f32 + size) / 2.0,
                ),
                color: Color::RGBA(0, 0, 255, 255),
                tex_coord: None,
            },
        ];

        // Render the triangle
        self.renderer.render_geometry(None, &vertices, &[]);

        // Define vertices for a textured triangle
        let textured_vertices = [
            Vertex {
                position: (10.0, 10.0),
                color: Color::RGBA(255, 255, 255, 255),
                tex_coord: Some((0.0, 0.0)),
            },
            Vertex {
                position: (150.0, 10.0),
                color: Color::RGBA(255, 255, 255, 255),
                tex_coord: Some((1.0, 0.0)),
            },
            Vertex {
                position: (10.0, 150.0),
                color: Color::RGBA(255, 255, 255, 255),
                tex_coord: Some((0.0, 1.0)),
            },
        ];

        // Render the textured triangle
        if let Some(ref texture) = self.texture {
            self.renderer
                .render_geometry(Some(texture), &textured_vertices, &[]);
        }

        // Define additional vertices and indices for a more complex textured polygon
        let mut extended_vertices = textured_vertices.to_vec();
        extended_vertices.push(Vertex {
            position: (600.0, 150.0),
            color: Color::RGBA(255, 255, 255, 255),
            tex_coord: Some((1.0, 1.0)),
        });

        let indices = [0, 1, 2, 1, 2, 3];

        // Render the indexed textured polygon
        if let Some(ref texture) = self.texture {
            self.renderer
                .render_geometry_indexed(Some(texture), &extended_vertices, &indices);
        }

        // Present the rendered content
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
