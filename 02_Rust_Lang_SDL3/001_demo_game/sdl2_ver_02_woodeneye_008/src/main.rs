extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::KeyboardState;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseState;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::f64::consts::PI;
use std::time::Duration;

// Constants
const MAX_PLAYER_COUNT: usize = 16;
const MAP_BOX_SCALE: f64 = 100.0;
const MAP_BOX_EDGES_LEN: usize = 32;
const SDL_PI_D: f64 = std::f64::consts::PI;

// Player struct
#[derive(Clone, Copy)]
struct Player {
    mouse: u32,
    keyboard: u32,
    pos: [f64; 3],
    vel: [f64; 3],
    yaw: u32,
    pitch: i32,
    radius: f32,
    height: f32,
    color: [u8; 3],
    wasd: u8,
}

// AppState struct
struct AppState {
    window: sdl2::video::Window,
    renderer: sdl2::render::Canvas<sdl2::video::Window>,
    player_count: usize,
    players: [Player; MAX_PLAYER_COUNT],
    edges: [[f32; 6]; MAP_BOX_EDGES_LEN],
}

// Extended metadata
const EXTENDED_METADATA: &[(&str, &str)] = &[
    (
        "SDL_PROP_APP_METADATA_URL_STRING",
        "https://examples.libsdl.org/SDL3/game/02-woodeneye-008/",
    ),
    ("SDL_PROP_APP_METADATA_CREATOR_STRING", "SDL team"),
    (
        "SDL_PROP_APP_METADATA_COPYRIGHT_STRING",
        "Placed in the public domain",
    ),
    ("SDL_PROP_APP_METADATA_TYPE_STRING", "game"),
];

// Helper functions
fn whose_mouse(mouse: u32, players: &[Player]) -> Option<usize> {
    players.iter().position(|player| player.mouse == mouse)
}

fn whose_keyboard(keyboard: u32, players: &[Player]) -> Option<usize> {
    players
        .iter()
        .position(|player| player.keyboard == keyboard)
}

// Shoot function
fn shoot(shooter: usize, players: &mut [Player]) {
    let x0 = players[shooter].pos[0];
    let y0 = players[shooter].pos[1];
    let z0 = players[shooter].pos[2];
    let bin_rad = SDL_PI_D / 2147483648.0;
    let yaw_rad = bin_rad * players[shooter].yaw as f64;
    let pitch_rad = bin_rad * players[shooter].pitch as f64;
    let cos_yaw = yaw_rad.cos();
    let sin_yaw = yaw_rad.sin();
    let cos_pitch = pitch_rad.cos();
    let sin_pitch = pitch_rad.sin();
    let vx = -sin_yaw * cos_pitch;
    let vy = sin_pitch;
    let vz = -cos_yaw * cos_pitch;

    for (i, target) in players.iter_mut().enumerate() {
        if i == shooter {
            continue;
        }
        let mut hit = 0;
        for j in 0..2 {
            let r = target.radius as f64;
            let h = target.height as f64;
            let dx = target.pos[0] - x0;
            let dy = target.pos[1] - y0 + if j == 0 { 0.0 } else { r - h };
            let dz = target.pos[2] - z0;
            let vd = vx * dx + vy * dy + vz * dz;
            let dd = dx * dx + dy * dy + dz * dz;
            let vv = vx * vx + vy * vy + vz * vz;
            let rr = r * r;
            if vd >= 0.0 && vd * vd >= vv * (dd - rr) {
                hit += 1;
            }
        }
        if hit > 0 {
            target.pos[0] = (MAP_BOX_SCALE * (rand::random::<u8>() as f64 - 128.0)) / 256.0;
            target.pos[1] = (MAP_BOX_SCALE * (rand::random::<u8>() as f64 - 128.0)) / 256.0;
            target.pos[2] = (MAP_BOX_SCALE * (rand::random::<u8>() as f64 - 128.0)) / 256.0;
        }
    }
}

// Update function
fn update(players: &mut [Player], dt_ns: u64) {
    let time = dt_ns as f64 * 1e-9;
    let rate = 6.0;
    let drag = (-time * rate).exp();
    let diff = 1.0 - drag;
    let mult = 60.0;
    let grav = 25.0;

    for player in players.iter_mut() {
        let yaw = player.yaw as f64;
        let rad = yaw * SDL_PI_D / 2147483648.0;
        let cos = rad.cos();
        let sin = rad.sin();
        let wasd = player.wasd;

        let dir_x = if wasd & 8 != 0 { 1.0 } else { 0.0 } - if wasd & 2 != 0 { 1.0 } else { 0.0 };
        let dir_z = if wasd & 4 != 0 { 1.0 } else { 0.0 } - if wasd & 1 != 0 { 1.0 } else { 0.0 };
        let norm = (dir_x * dir_x + dir_z * dir_z).sqrt();

        let acc_x = mult
            * if norm == 0.0 {
                0.0
            } else {
                (cos * dir_x + sin * dir_z) / norm
            };
        let acc_z = mult
            * if norm == 0.0 {
                0.0
            } else {
                (-sin * dir_x + cos * dir_z) / norm
            };

        player.vel[0] = player.vel[0] * drag - acc_x * diff / rate;
        player.vel[1] -= grav * time;
        player.vel[2] = player.vel[2] * drag - acc_z * diff / rate;

        player.pos[0] += (time - diff / rate) * acc_x / rate + diff * player.vel[0] / rate;
        player.pos[1] += -0.5 * grav * time * time + player.vel[1] * time;
        player.pos[2] += (time - diff / rate) * acc_z / rate + diff * player.vel[2] / rate;

        let scale = MAP_BOX_SCALE;
        let bound = scale - player.radius as f64;
        player.pos[0] = player.pos[0].clamp(-bound, bound);
        player.pos[1] = player.pos[1].clamp(player.height as f64 - scale, scale);
        player.pos[2] = player.pos[2].clamp(-bound, bound);

        if player.pos[1] < player.height as f64 - scale {
            player.vel[1] = if wasd & 16 != 0 { 8.4375 } else { 0.0 };
        }
    }
}

struct Particle {
    x: f32,
    y: f32,
    dx: f32,
    dy: f32,
}

impl Particle {
    fn new(x: f32, y: f32, dx: f32, dy: f32) -> Self {
        Self { x, y, dx, dy }
    }

    fn update(&mut self) {
        self.x += self.dx;
        self.y += self.dy;

        // Bounce off walls
        if self.x <= 0.0 || self.x >= 800.0 {
            self.dx = -self.dx;
        }
        if self.y <= 0.0 || self.y >= 600.0 {
            self.dy = -self.dy;
        }
    }

    fn draw(&self, canvas: &mut Canvas<Window>) {
        let rect = Rect::new(self.x as i32, self.y as i32, 5, 5);
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        let _ = canvas.fill_rect(rect);
    }
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("Particle Simulation", 800, 600)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let mut event_pump = sdl_context.event_pump()?;

    let mut particles = vec![
        Particle::new(400.0, 300.0, 2.0, 3.0),
        Particle::new(200.0, 150.0, -1.5, 2.5),
    ];

    'running: loop {
        // Handle events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        // Update particles
        for particle in particles.iter_mut() {
            particle.update();
        }

        // Render
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        for particle in &particles {
            particle.draw(&mut canvas);
        }
        canvas.present();

        // Control frame rate
        std::thread::sleep(Duration::from_millis(16));
    }

    Ok(())
}
