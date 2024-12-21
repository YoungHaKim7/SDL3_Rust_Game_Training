use sdl3::{
    render::{BlendMode, Canvas, Texture, TextureCreator},
    video::{Window, WindowContext},
    Sdl,
};

struct AppState<'a> {
    canvas: Canvas<Window>,
    texture_creator: TextureCreator<WindowContext>,
    render_target: Texture<'a>,
    pressure: f32,
    previous_touch_x: Option<f32>,
    previous_touch_y: Option<f32>,
}

impl<'a> Copy for AppState<'a> {}

impl AppState<'_> {
    pub fn new(sdl: &Sdl) -> Self {
        let video_subsystem = sdl.video().unwrap();

        // Create the window and canvas
        let window = video_subsystem
            .window("Example Pen Drawing Lines", 640, 480)
            .position_centered()
            .build()
            .expect("Failed to create window");

        let mut canvas = window.into_canvas();
        let texture_creator = canvas.texture_creator();

        // Create render target texture
        let mut render_target = texture_creator
            .create_texture_target(None, 640, 480)
            .expect("Failed to create render target");

        // Initialize the render target with a gray background
        canvas.with_texture_canvas(&mut render_target, |texture_canvas| {
            texture_canvas.set_draw_color((100, 100, 100, 255));
            texture_canvas.clear();
        });

        // Set blend mode for the canvas
        canvas.set_blend_mode(BlendMode::Blend);

        AppState {
            canvas,
            texture_creator,
            render_target,
            pressure: 0.0,
            previous_touch_x: None,
            previous_touch_y: None,
        }
    }

    // ... rest of the implementation remains the same
}

fn main() {
    let sdl_context = sdl3::init().expect("Failed to initialize SDL3");
    let mut app_state = AppState::new(&sdl_context);
    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop {
        for event in event_pump.poll_iter() {
            if !app_state.handle_event(&event) {
                break 'running;
            }
        }
        app_state.render();
    }
}
