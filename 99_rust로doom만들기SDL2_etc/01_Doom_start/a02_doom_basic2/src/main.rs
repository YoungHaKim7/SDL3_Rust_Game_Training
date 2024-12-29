use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::f32::consts::PI;
use std::time::Instant;

const RES: i32 = 1;
const SW: i32 = 160 * RES; // Screen width
const SH: i32 = 120 * RES; // Screen height
const SW2: i32 = SW / 2; // Half screen width
const SH2: i32 = SH / 2; // Half screen height
const PIXEL_SCALE: i32 = 4 / RES; // SDL2 pixel scale
const GLSW: i32 = SW * PIXEL_SCALE; // SDL2 window width
const GLSH: i32 = SH * PIXEL_SCALE; // SDL2 window height

struct Time {
    fr1: u128,
    fr2: u128,
}

struct Keys {
    w: bool,
    s: bool,
    a: bool,
    d: bool,
    sl: bool,
    sr: bool,
    m: bool,
}

struct Math {
    cos: [f32; 360],
    sin: [f32; 360],
}

struct Player {
    x: f32,
    y: f32,
    z: f32,
    a: i32,
    l: i32,
}

impl Math {
    fn new() -> Self {
        let mut cos = [0.0; 360];
        let mut sin = [0.0; 360];
        for i in 0..360 {
            cos[i] = (i as f32 * PI / 180.0).cos();
            sin[i] = (i as f32 * PI / 180.0).sin();
        }
        Math { cos, sin }
    }
}

impl Player {
    fn new() -> Self {
        Player {
            x: 70.0,
            y: -110.0,
            z: 20.0,
            a: 0,
            l: 0,
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
        .fill_rect(Rect::new(
            x * PIXEL_SCALE + 2,
            y * PIXEL_SCALE + 2,
            PIXEL_SCALE as u32,
            PIXEL_SCALE as u32,
        ))
        .unwrap();
}

fn clear_background(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
    for y in 0..SH {
        for x in 0..SW {
            pixel(canvas, x, y, 8); // Clear background color
        }
    }
}

fn move_player(player: &mut Player, keys: &Keys, math: &Math) {
    if keys.a && !keys.m {
        player.a -= 4;
        if player.a < 0 {
            player.a += 360;
        }
    }
    if keys.d && !keys.m {
        player.a += 4;
        if player.a > 359 {
            player.a -= 360;
        }
    }

    let dx = math.sin[player.a as usize] * 10.0;
    let dy = math.cos[player.a as usize] * 10.0;

    if keys.w && !keys.m {
        player.x += dx;
        player.y += dy;
    }
    if keys.s && !keys.m {
        player.x -= dx;
        player.y -= dy;
    }
    if keys.sr {
        player.x += dy;
        player.y -= dx;
    }
    if keys.sl {
        player.x -= dy;
        player.y += dx;
    }
    if keys.a && keys.m {
        player.l -= 1;
    }
    if keys.d && keys.m {
        player.l += 1;
    }
    if keys.w && keys.m {
        player.z -= 4.0;
    }
    if keys.s && keys.m {
        player.z += 4.0;
    }
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("3DSage", GLSW as u32, GLSH as u32)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let mut event_pump = sdl_context.event_pump()?;

    let math = Math::new();
    let mut player = Player::new();
    let mut keys = Keys {
        w: false,
        s: false,
        a: false,
        d: false,
        sl: false,
        sr: false,
        m: false,
    };

    let mut time = Time {
        fr1: 0,
        fr2: Instant::now().elapsed().as_millis(),
    };

    canvas.set_scale(PIXEL_SCALE as f32, PIXEL_SCALE as f32)?;

    'running: loop {
        time.fr1 = Instant::now().elapsed().as_millis();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::W),
                    ..
                } => keys.w = true,
                Event::KeyDown {
                    keycode: Some(Keycode::S),
                    ..
                } => keys.s = true,
                Event::KeyDown {
                    keycode: Some(Keycode::A),
                    ..
                } => keys.a = true,
                Event::KeyDown {
                    keycode: Some(Keycode::D),
                    ..
                } => keys.d = true,
                Event::KeyDown {
                    keycode: Some(Keycode::M),
                    ..
                } => keys.m = true,
                Event::KeyUp {
                    keycode: Some(Keycode::W),
                    ..
                } => keys.w = false,
                Event::KeyUp {
                    keycode: Some(Keycode::S),
                    ..
                } => keys.s = false,
                Event::KeyUp {
                    keycode: Some(Keycode::A),
                    ..
                } => keys.a = false,
                Event::KeyUp {
                    keycode: Some(Keycode::D),
                    ..
                } => keys.d = false,
                _ => {}
            }
        }

        if time.fr1 - time.fr2 >= 50 {
            clear_background(&mut canvas);
            move_player(&mut player, &keys, &math);
            canvas.present();
            time.fr2 = time.fr1;
        }
    }

    Ok(())
}
