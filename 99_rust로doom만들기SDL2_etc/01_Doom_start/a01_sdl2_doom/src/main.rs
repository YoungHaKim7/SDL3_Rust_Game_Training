use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use std::time::{Duration, Instant};

const RES: u32 = 1; // 0=160x120, 1=360x240, 4=640x480
const SW: u32 = 160 * RES; // Screen width
const SH: u32 = 120 * RES; // Screen height
const SW2: u32 = SW / 2; // Half of screen width
const SH2: u32 = SH / 2; // Half of screen height
const PIXEL_SCALE: u32 = 4 / RES; // OpenGL pixel scale
const GLSW: u32 = SW * PIXEL_SCALE; // OpenGL window width
const GLSH: u32 = SH * PIXEL_SCALE; // OpenGL window height

struct Keys {
    w: bool,
    s: bool,
    a: bool,
    d: bool,
    sl: bool,
    sr: bool,
    m: bool,
}

impl Keys {
    fn new() -> Self {
        Self {
            w: false,
            s: false,
            a: false,
            d: false,
            sl: false,
            sr: false,
            m: false,
        }
    }
}

fn pixel(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, x: i32, y: i32, c: u8) {
    let color = match c {
        0 => Color::RGB(255, 255, 0), // Yellow
        1 => Color::RGB(160, 160, 0), // Yellow darker
        2 => Color::RGB(0, 255, 0),   // Green
        3 => Color::RGB(0, 160, 0),   // Green darker
        4 => Color::RGB(0, 255, 255), // Cyan
        5 => Color::RGB(0, 160, 160), // Cyan darker
        6 => Color::RGB(160, 100, 0), // Brown
        7 => Color::RGB(110, 50, 0),  // Brown darker
        _ => Color::RGB(0, 60, 130),  // Background
    };
    canvas.set_draw_color(color);
    canvas
        .draw_point(Point::new(x * PIXEL_SCALE as i32, y * PIXEL_SCALE as i32))
        .unwrap();
}

fn move_player(keys: &Keys) {
    if keys.a && !keys.m {
        println!("left");
    }
    if keys.d && !keys.m {
        println!("right");
    }
    if keys.w && !keys.m {
        println!("up");
    }
    if keys.s && !keys.m {
        println!("down");
    }
    if keys.sr {
        println!("strafe left");
    }
    if keys.sl {
        println!("strafe right");
    }
    if keys.a && keys.m {
        println!("look up");
    }
    if keys.d && keys.m {
        println!("look down");
    }
    if keys.w && keys.m {
        println!("move up");
    }
    if keys.s && keys.m {
        println!("move down");
    }
}

fn clear_background(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
    canvas.set_draw_color(Color::RGB(0, 60, 130));
    canvas.clear();
}

fn draw_3d(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, tick: u32) {
    let mut c = 0;
    for y in 0..SH2 {
        for x in 0..SW2 {
            pixel(canvas, x as i32, y as i32, c);
            c += 1;
            if c > 8 {
                c = 0;
            }
        }
    }
    pixel(canvas, SW2 as i32, (SH2 + tick) as i32, 0);
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("Rust SDL2 Demo", GLSW, GLSH)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let mut event_pump = sdl_context.event_pump()?;

    let mut keys = Keys::new();
    let mut last_frame_time = Instant::now();
    let mut tick = 0;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(key), ..
                } => match key {
                    Keycode::W => keys.w = true,
                    Keycode::S => keys.s = true,
                    Keycode::A => keys.a = true,
                    Keycode::D => keys.d = true,
                    Keycode::M => keys.m = true,
                    Keycode::Comma => keys.sr = true,
                    Keycode::Period => keys.sl = true,
                    _ => {}
                },
                Event::KeyUp {
                    keycode: Some(key), ..
                } => match key {
                    Keycode::W => keys.w = false,
                    Keycode::S => keys.s = false,
                    Keycode::A => keys.a = false,
                    Keycode::D => keys.d = false,
                    Keycode::M => keys.m = false,
                    Keycode::Comma => keys.sr = false,
                    Keycode::Period => keys.sl = false,
                    _ => {}
                },
                _ => {}
            }
        }

        if last_frame_time.elapsed() >= Duration::from_millis(50) {
            clear_background(&mut canvas);
            move_player(&keys);
            draw_3d(&mut canvas, tick);

            canvas.present();
            tick = (tick + 1) % 20;
            last_frame_time = Instant::now();
        }

        std::thread::sleep(Duration::from_millis(1));
    }

    Ok(())
}
