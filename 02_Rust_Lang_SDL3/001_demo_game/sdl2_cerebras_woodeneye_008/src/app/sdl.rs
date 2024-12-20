use sdl2::init;
use sdl2::render::Canvas;
use sdl2::render::Renderer;
use sdl2::render::Texture;
use sdl2::video::Window;

pub fn init_sdl() -> (Renderer, Canvas<Window>) {
    let sdl_context = init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("SDL App", 800, 600)
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    (canvas.renderer().unwrap(), canvas)
}
