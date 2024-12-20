// use sdl3::sys::SDL_KeyboardID;
// use sdl3::sys::SDL_MouseID;
use sdl3::video::{Renderer, Window};
use sdl3_sys::everything::SDL_KeyboardID;
use sdl3_sys::everything::SDL_MouseID;

const MAP_BOX_SCALE: usize = 16;
const MAP_BOX_EDGES_LEN: usize = 12 + MAP_BOX_SCALE * 2;
const MAX_PLAYER_COUNT: usize = 4;
const CIRCLE_DRAW_SIDES: usize = 32;
const CIRCLE_DRAW_SIDES_LEN: usize = CIRCLE_DRAW_SIDES + 1;

static mut DEBUG_STRING: [u8; 32] = [0; 32]; // Equivalent to `debug_string` in C

static EXTENDED_METADATA: &[(&str, &str)] = &[
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

#[derive(Debug, Clone)]
struct Player {
    mouse: SDL_MouseID,
    keyboard: SDL_KeyboardID,
    pos: [f64; 3],
    vel: [f64; 3],
    yaw: u32,
    pitch: i32,
    radius: f32,
    height: f32,
    color: [u8; 3],
    wasd: u8,
}

struct AppState {
    window: Window,
    renderer: Renderer,
    player_count: usize,
    players: [Option<Player>; MAX_PLAYER_COUNT],
    edges: [[f32; 6]; MAP_BOX_EDGES_LEN],
}

impl Default for AppState {
    fn default() -> Self {
        // Initialize with default values
    }
}

fn whose_mouse(mouse: SDL_MouseID, players: &[Option<Player>]) -> Option<usize> {
    players.iter().position(|player| match player {
        Some(player) if player.mouse == mouse => true,
        _ => false,
    })
}

fn whose_keyboard(keyboard: SDL_KeyboardID, players: &[Option<Player>]) -> Option<usize> {
    players.iter().position(|player| match player {
        Some(player) if player.keyboard == keyboard => true,
        _ => false,
    })
}

fn shoot(shooter: usize, players: &mut [Option<Player>]) {
    if let Some(shooter_player) = &players[shooter] {
        let x0 = shooter_player.pos[0];
        let y0 = shooter_player.pos[1];
        let z0 = shooter_player.pos[2];

        let bin_rad = std::f64::consts::PI / 2_147_483_648.0;
        let yaw_rad = bin_rad * shooter_player.yaw as f64;
        let pitch_rad = bin_rad * shooter_player.pitch as f64;

        let cos_yaw = yaw_rad.cos();
        let sin_yaw = yaw_rad.sin();
        let cos_pitch = pitch_rad.cos();
        let sin_pitch = pitch_rad.sin();

        let vx = -sin_yaw * cos_pitch;
        let vy = sin_pitch;
        let vz = -cos_yaw * cos_pitch;

        for (i, player) in players.iter_mut().enumerate() {
            if i == shooter {
                continue;
            }

            if let Some(target) = player {
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
                    target.pos[0] =
                        (MAP_BOX_SCALE as f64 * (rand::random::<i8>() as f64 - 128.0)) / 256.0;
                    target.pos[1] =
                        (MAP_BOX_SCALE as f64 * (rand::random::<i8>() as f64 - 128.0)) / 256.0;
                    target.pos[2] =
                        (MAP_BOX_SCALE as f64 * (rand::random::<i8>() as f64 - 128.0)) / 256.0;
                }
            }
        }
    }
}

fn update(players: &mut [Player], dt_ns: u64) {
    let time = dt_ns as f64 * 1e-9;
    let rate = 6.0;
    let drag = (-time * rate).exp();
    let diff = 1.0 - drag;
    let mult = 60.0;
    let grav = 25.0;

    for player in players.iter_mut() {
        let yaw = player.yaw as f64;
        let rad = yaw * std::f64::consts::PI / 2_147_483_648.0;
        let cos = rad.cos();
        let sin = rad.sin();
        let wasd = player.wasd;
        let dir_x = if wasd & 8 != 0 { 1.0 } else { 0.0 } - if wasd & 2 != 0 { 1.0 } else { 0.0 };
        let dir_z = if wasd & 4 != 0 { 1.0 } else { 0.0 } - if wasd & 1 != 0 { 1.0 } else { 0.0 };
        let norm = dir_x * dir_x + dir_z * dir_z;

        let acc_x = mult
            * if norm == 0.0 {
                0.0
            } else {
                (cos * dir_x + sin * dir_z) / norm.sqrt()
            };
        let acc_z = mult
            * if norm == 0.0 {
                0.0
            } else {
                (-sin * dir_x + cos * dir_z) / norm.sqrt()
            };
        let vel_x = player.vel[0];
        let vel_y = player.vel[1];
        let vel_z = player.vel[2];

        player.vel[0] -= vel_x * diff;
        player.vel[1] -= grav * time;
        player.vel[2] -= vel_z * diff;

        player.vel[0] += diff * acc_x / rate;
        player.vel[2] += diff * acc_z / rate;
        player.pos[0] += (time - diff / rate) * acc_x / rate + diff * vel_x / rate;
        player.pos[1] += -0.5 * grav * time * time + vel_y * time;
        player.pos[2] += (time - diff / rate) * acc_z / rate + diff * vel_z / rate;
        let scale = MAP_BOX_SCALE as f64;
        let bound = scale - player.radius as f64;

        let pos_x = player.pos[0].clamp(-bound, bound);
        let pos_y = player.pos[1].clamp(player.height as f64 - scale, bound);
        let pos_z = player.pos[2].clamp(-bound, bound);

        if player.pos[0] != pos_x {
            player.vel[0] = 0.0;
        }
        if player.pos[1] != pos_y {
            player.vel[1] = if wasd & 16 != 0 { 8.4375 } else { 0.0 };
        }
        if player.pos[2] != pos_z {
            player.vel[2] = 0.0;
        }

        player.pos[0] = pos_x;
        player.pos[1] = pos_y;
        player.pos[2] = pos_z;
    }
}

fn draw_circle(renderer: &mut SDL_Renderer, r: f32, x: f32, y: f32) {
    let mut points = [SDL_FPoint { x: 0.0, y: 0.0 }; CIRCLE_DRAW_SIDES_LEN];

    for i in 0..CIRCLE_DRAW_SIDES_LEN {
        let ang = 2.0 * std::f32::consts::PI * i as f32 / CIRCLE_DRAW_SIDES as f32;
        points[i].x = x + r * ang.cos();
        points[i].y = y + r * ang.sin();
    }

    renderer.render_lines(&points).unwrap();
}
fn draw_clipped_segment(
    renderer: &mut SDL_Renderer,
    ax: f32,
    ay: f32,
    az: f32,
    bx: f32,
    by: f32,
    bz: f32,
    x: f32,
    y: f32,
    z: f32,
    w: f32,
) {
    let (mut ax, mut ay, mut az, mut bx, mut by, mut bz) = (ax, ay, az, bx, by, bz);

    if az >= -w && bz >= -w {
        return;
    }

    let dx = ax - bx;
    let dy = ay - by;

    if az > -w {
        let t = (-w - bz) / (az - bz);
        ax = bx + dx * t;
        ay = by + dy * t;
        az = -w;
    } else if bz > -w {
        let t = (-w - az) / (bz - az);
        bx = ax - dx * t;
        by = ay - dy * t;
        bz = -w;
    }

    ax = -z * ax / az;
    ay = -z * ay / az;
    bx = -z * bx / bz;
    by = -z * by / bz;

    renderer
        .render_line(x + ax, y - ay, x + bx, y - by)
        .unwrap();
}

fn draw(renderer: &mut SDL_Renderer, edges: &[[f32; 6]], players: &[Player]) {
    let (mut w, mut h) = (0, 0);
    if SDL_GetRenderOutputSize(renderer, &mut w, &mut h) != 0 {
        return;
    }

    // Clear the screen
    renderer.set_draw_color(SDL_Color {
        r: 0,
        g: 0,
        b: 0,
        a: SDL_ALPHA_OPAQUE,
    });
    renderer.clear();

    if !players.is_empty() {
        let wf = w as f32;
        let hf = h as f32;

        // Divide the screen into player-specific viewports
        let part_hor = if players.len() > 2 { 2 } else { 1 };
        let part_ver = if players.len() > 1 { 2 } else { 1 };
        let size_hor = wf / part_hor as f32;
        let size_ver = hf / part_ver as f32;

        for (i, player) in players.iter().enumerate() {
            // Calculate viewport for the current player
            let mod_x = (i % part_hor) as f32;
            let mod_y = (i / part_hor) as f32;
            let hor_origin = (mod_x + 0.5) * size_hor;
            let ver_origin = (mod_y + 0.5) * size_ver;
            let cam_origin = 0.5 * ((size_hor * size_hor + size_ver * size_ver).sqrt());
            let hor_offset = mod_x * size_hor;
            let ver_offset = mod_y * size_ver;

            let rect = SDL_Rect {
                x: hor_offset as i32,
                y: ver_offset as i32,
                w: size_hor as i32,
                h: size_ver as i32,
            };
            renderer.set_clip_rect(Some(&rect));

            // Camera calculations
            let x0 = player.pos[0];
            let y0 = player.pos[1];
            let z0 = player.pos[2];
            let bin_rad = std::f64::consts::PI / 2147483648.0;
            let yaw_rad = bin_rad * player.yaw as f64;
            let pitch_rad = bin_rad * player.pitch as f64;

            let cos_yaw = yaw_rad.cos();
            let sin_yaw = yaw_rad.sin();
            let cos_pitch = pitch_rad.cos();
            let sin_pitch = pitch_rad.sin();

            let mat = [
                cos_yaw,
                0.0,
                -sin_yaw,
                sin_yaw * sin_pitch,
                cos_pitch,
                cos_yaw * sin_pitch,
                sin_yaw * cos_pitch,
                -sin_pitch,
                cos_yaw * cos_pitch,
            ];

            // Draw edges
            renderer.set_draw_color(SDL_Color {
                r: 64,
                g: 64,
                b: 64,
                a: 255,
            });
            for edge in edges {
                let ax = mat[0] * (edge[0] as f64 - x0)
                    + mat[1] * (edge[1] as f64 - y0)
                    + mat[2] * (edge[2] as f64 - z0);
                let ay = mat[3] * (edge[0] as f64 - x0)
                    + mat[4] * (edge[1] as f64 - y0)
                    + mat[5] * (edge[2] as f64 - z0);
                let az = mat[6] * (edge[0] as f64 - x0)
                    + mat[7] * (edge[1] as f64 - y0)
                    + mat[8] * (edge[2] as f64 - z0);
                let bx = mat[0] * (edge[3] as f64 - x0)
                    + mat[1] * (edge[4] as f64 - y0)
                    + mat[2] * (edge[5] as f64 - z0);
                let by = mat[3] * (edge[3] as f64 - x0)
                    + mat[4] * (edge[4] as f64 - y0)
                    + mat[5] * (edge[5] as f64 - z0);
                let bz = mat[6] * (edge[3] as f64 - x0)
                    + mat[7] * (edge[4] as f64 - y0)
                    + mat[8] * (edge[5] as f64 - z0);

                draw_clipped_segment(
                    renderer,
                    ax as f32,
                    ay as f32,
                    az as f32,
                    bx as f32,
                    by as f32,
                    bz as f32,
                    hor_origin,
                    ver_origin,
                    cam_origin as f32,
                    1.0,
                );
            }

            // Draw other players
            for (j, target) in players.iter().enumerate() {
                if i == j {
                    continue;
                }

                renderer.set_draw_color(SDL_Color {
                    r: target.color[0],
                    g: target.color[1],
                    b: target.color[2],
                    a: 255,
                });

                for k in 0..2 {
                    let rx = target.pos[0] - player.pos[0];
                    let ry = target.pos[1] - player.pos[1]
                        + (target.radius - target.height) as f64 * k as f64;
                    let rz = target.pos[2] - player.pos[2];
                    let dx = mat[0] * rx + mat[1] * ry + mat[2] * rz;
                    let dy = mat[3] * rx + mat[4] * ry + mat[5] * rz;
                    let dz = mat[6] * rx + mat[7] * ry + mat[8] * rz;

                    if dz < 0.0 {
                        let r_eff = target.radius as f64 * cam_origin / dz;
                        draw_circle(
                            renderer,
                            r_eff as f32,
                            (hor_origin - cam_origin as f32 * dx as f32 / dz as f32),
                            (ver_origin + cam_origin as f32 * dy as f32 / dz as f32),
                        );
                    }
                }
            }

            // Crosshair
            renderer.set_draw_color(SDL_Color {
                r: 255,
                g: 255,
                b: 255,
                a: 255,
            });
            renderer
                .draw_line(
                    hor_origin as i32,
                    (ver_origin - 10.0) as i32,
                    hor_origin as i32,
                    (ver_origin + 10.0) as i32,
                )
                .unwrap();
            renderer
                .draw_line(
                    (hor_origin - 10.0) as i32,
                    ver_origin as i32,
                    (hor_origin + 10.0) as i32,
                    ver_origin as i32,
                )
                .unwrap();
        }
    }

    // Debug text and present
    renderer.set_clip_rect(None);
    renderer.set_draw_color(SDL_Color {
        r: 255,
        g: 255,
        b: 255,
        a: 255,
    });
    renderer.render_debug_text(0, 0, unsafe { &DEBUG_STRING });
    renderer.present();
}

fn init_players(players: &mut [Player]) {
    for (i, player) in players.iter_mut().enumerate() {
        player.pos[0] = if i & 1 != 0 { -8.0 } else { 8.0 };
        player.pos[1] = 0.0;
        player.pos[2] = if i & 1 != 0 { -8.0 } else { 8.0 } * if i & 2 != 0 { -1.0 } else { 1.0 };

        player.vel[0] = 0.0;
        player.vel[1] = 0.0;
        player.vel[2] = 0.0;

        player.yaw = 0x20000000
            + if i & 1 != 0 { 0x80000000 } else { 0 }
            + if i & 2 != 0 { 0x40000000 } else { 0 };
        player.pitch = -0x08000000;

        player.radius = 0.5;
        player.height = 1.5;
        player.wasd = 0;
        player.mouse = 0;
        player.keyboard = 0;

        player.color[0] = if (1 << (i / 2)) & 2 == 0 { 0xff } else { 0x00 };
        player.color[1] = if (1 << (i / 2)) & 1 == 0 { 0xff } else { 0x00 };
        player.color[2] = if (1 << (i / 2)) & 4 == 0 { 0xff } else { 0x00 };

        player.color[0] = if i & 1 != 0 {
            player.color[0]
        } else {
            !player.color[0]
        };
        player.color[1] = if i & 1 != 0 {
            player.color[1]
        } else {
            !player.color[1]
        };
        player.color[2] = if i & 1 != 0 {
            player.color[2]
        } else {
            !player.color[2]
        };
    }
}

fn init_edges(scale: i32, edges: &mut [[f32; 6]]) {
    const MAP: [i32; 24] = [
        0, 1, 1, 3, 3, 2, 2, 0, 7, 6, 6, 4, 4, 5, 5, 7, 6, 2, 3, 7, 0, 4, 5, 1,
    ];

    let r = scale as f32;

    // Initialize the first 12 edges
    for i in 0..12 {
        for j in 0..3 {
            edges[i][j + 0] = if MAP[i * 2 + 0] & (1 << j) != 0 {
                r
            } else {
                -r
            };
            edges[i][j + 3] = if MAP[i * 2 + 1] & (1 << j) != 0 {
                r
            } else {
                -r
            };
        }
    }

    // Initialize the additional edges for the scale
    for i in 0..scale {
        let d = (i * 2) as f32;
        for j in 0..2 {
            edges[(i + 12) as usize][j * 3 + 0] = if j != 0 { r } else { -r };
            edges[(i + 12) as usize][j * 3 + 1] = -r;
            edges[(i + 12) as usize][j * 3 + 2] = d - r;

            edges[(i + 12 + scale) as usize][j * 3 + 0] = d - r;
            edges[(i + 12 + scale) as usize][j * 3 + 1] = -r;
            edges[(i + 12 + scale) as usize][j * 3 + 2] = if j != 0 { r } else { -r };
        }
    }
}

fn sdl_app_init(appstate: &mut Option<AppState>, args: &[String]) -> Result<(), String> {
    // Set application metadata
    if !sdl2::hint::set_app_metadata(
        "Example splitscreen shooter game",
        "1.0",
        "com.example.woodeneye-008",
    ) {
        return Err("Failed to set app metadata".to_string());
    }

    for meta in EXTENDED_METADATA.iter() {
        if !sdl2::hint::set(meta.key, meta.value) {
            return Err(format!("Failed to set metadata property: {}", meta.key));
        }
    }

    // Allocate and initialize application state
    let mut state = AppState::default();

    // Initialize SDL video subsystem
    let sdl_context = sdl2::init().map_err(|e| format!("SDL initialization failed: {}", e))?;
    let video_subsystem = sdl_context
        .video()
        .map_err(|e| format!("Video subsystem initialization failed: {}", e))?;

    // Create window and renderer
    let window = video_subsystem
        .window("examples/demo/woodeneye-008", 640, 480)
        .position_centered()
        .build()
        .map_err(|e| format!("Window creation failed: {}", e))?;
    let mut canvas = window
        .into_canvas()
        .build()
        .map_err(|e| format!("Renderer creation failed: {}", e))?;

    // Initialize players and edges
    state.player_count = 1;
    init_players(&mut state.players);
    init_edges(MAP_BOX_SCALE, &mut state.edges);

    debug_string.clear();

    // Set renderer vsync
    canvas.set_present_vsync(false);

    // Set relative mouse mode and hints
    video_subsystem.set_relative_mouse_mode(true);
    sdl2::hint::set_with_priority(
        "SDL_HINT_WINDOWS_RAW_KEYBOARD",
        "1",
        sdl2::hint::HintPriority::Override,
    );

    *appstate = Some(state);

    Ok(())
}

fn sdl_app_event(appstate: &mut AppState, event: &sdl2::event::Event) -> Result<(), String> {
    let players = &mut appstate.players;
    let player_count = &mut appstate.player_count;

    match event {
        sdl2::event::Event::Quit { .. } => {
            return Err("Quit requested".to_string());
        }
        sdl2::event::Event::MouseDeviceRemoved { which } => {
            for player in players.iter_mut().take(*player_count) {
                if player.mouse == *which {
                    player.mouse = 0;
                }
            }
        }
        sdl2::event::Event::KeyboardDeviceRemoved { which } => {
            for player in players.iter_mut().take(*player_count) {
                if player.keyboard == *which {
                    player.keyboard = 0;
                }
            }
        }
        sdl2::event::Event::MouseMotion {
            which, xrel, yrel, ..
        } => {
            if let Some(index) = whose_mouse(*which, players, *player_count) {
                players[index].yaw -= (*xrel as i32) * 0x00080000;
                players[index].pitch = players[index]
                    .pitch
                    .saturating_sub((*yrel as i32) * 0x00080000)
                    .clamp(-0x40000000, 0x40000000);
            } else if *which != 0 {
                for (i, player) in players.iter_mut().enumerate().take(MAX_PLAYER_COUNT) {
                    if player.mouse == 0 {
                        player.mouse = *which;
                        *player_count = (*player_count).max(i + 1);
                        break;
                    }
                }
            }
        }
        sdl2::event::Event::MouseButtonDown { which, .. } => {
            if let Some(index) = whose_mouse(*which, players, *player_count) {
                shoot(index, players, *player_count);
            }
        }
        sdl2::event::Event::KeyDown { keycode, which, .. } => {
            if let Some(sym) = keycode {
                if let Some(index) = whose_keyboard(*which, players, *player_count) {
                    match sym {
                        sdl2::keyboard::Keycode::W => players[index].wasd |= 1,
                        sdl2::keyboard::Keycode::A => players[index].wasd |= 2,
                        sdl2::keyboard::Keycode::S => players[index].wasd |= 4,
                        sdl2::keyboard::Keycode::D => players[index].wasd |= 8,
                        sdl2::keyboard::Keycode::Space => players[index].wasd |= 16,
                        _ => {}
                    }
                } else if *which != 0 {
                    for (i, player) in players.iter_mut().enumerate().take(MAX_PLAYER_COUNT) {
                        if player.keyboard == 0 {
                            player.keyboard = *which;
                            *player_count = (*player_count).max(i + 1);
                            break;
                        }
                    }
                }
            }
        }
        sdl2::event::Event::KeyUp { keycode, which, .. } => {
            if let Some(sym) = keycode {
                if sym == sdl2::keyboard::Keycode::Escape {
                    return Err("Quit requested".to_string());
                }
                if let Some(index) = whose_keyboard(*which, players, *player_count) {
                    match sym {
                        sdl2::keyboard::Keycode::W => players[index].wasd &= 30,
                        sdl2::keyboard::Keycode::A => players[index].wasd &= 29,
                        sdl2::keyboard::Keycode::S => players[index].wasd &= 27,
                        sdl2::keyboard::Keycode::D => players[index].wasd &= 23,
                        sdl2::keyboard::Keycode::Space => players[index].wasd &= 15,
                        _ => {}
                    }
                }
            }
        }
        _ => {}
    }
    Ok(())
}

fn sdl_app_iterate(appstate: &mut AppState) -> Result<(), String> {
    use std::time::{Duration, Instant};

    static mut ACCU: u64 = 0;
    static mut LAST: Instant = Instant::now();
    static mut PAST: Instant = Instant::now();

    let now = Instant::now();
    let dt_ns = now.duration_since(unsafe { PAST }).as_nanos() as u64;

    // Update game state
    update(&mut appstate.players, appstate.player_count, dt_ns);

    // Render the scene
    draw(
        &appstate.renderer,
        &appstate.edges,
        &appstate.players,
        appstate.player_count,
    );

    // Handle FPS debug string
    unsafe {
        if now.duration_since(LAST).as_secs() >= 1 {
            LAST = now;
            let debug_string = format!("{} fps", ACCU);
            println!("{}", debug_string); // Replace with actual debug display logic
            ACCU = 0;
        }

        PAST = now;
        ACCU += 1;
    }

    // Sleep to maintain frame rate
    let elapsed_ns = now.elapsed().as_nanos() as u64;
    if elapsed_ns < 999_999 {
        std::thread::sleep(Duration::from_nanos(999_999 - elapsed_ns));
    }

    Ok(())
}

fn sdl_app_quit(appstate: Option<Box<AppState>>) {
    // Free the memory by dropping the `appstate`.
    drop(appstate);
}
use sdl3::event::Event;
use sdl3::init::{InitFlag, Sdl};
use sdl3::keyboard::Keycode;
use sdl3::render::{Renderer, RendererFlag};
use sdl3::timer;
use sdl3::video::{Window, WindowFlag};

fn main() -> Result<(), String> {
    // Initialize SDL3
    let sdl = Sdl::init(InitFlag::Video | InitFlag::Timer)?;

    // Create the window
    let window = Window::new(
        "Example splitscreen shooter game",
        640,
        480,
        WindowFlag::Resizable | WindowFlag::Shown,
    )?;

    // Create the renderer
    let renderer = Renderer::new(
        &window,
        RendererFlag::Accelerated | RendererFlag::PresentVSync,
    )?;

    // Initialize application state
    let mut appstate = AppState {
        window: Some(window),
        renderer,
        players: vec![Player::default(); MAX_PLAYER_COUNT],
        edges: vec![[0.0; 6]; MAP_BOX_EDGES_LEN],
        player_count: 1,
    };

    // Initialize players and edges
    init_players(&mut appstate.players, MAX_PLAYER_COUNT);
    init_edges(MAP_BOX_SCALE, &mut appstate.edges);

    // Game loop
    let mut event_queue = sdl.event_queue()?;
    let mut last_time = timer::performance_counter();
    let mut accumulator = 0_u64;

    'running: loop {
        // Process events
        while let Some(event) = event_queue.poll() {
            match sdl_app_event(&mut appstate, &event) {
                Ok(()) => {}
                Err(_) => break 'running,
            }
        }

        // Calculate delta time
        let current_time = timer::performance_counter();
        let delta_time = (current_time - last_time) * 1_000_000 / timer::performance_frequency();
        last_time = current_time;
        accumulator += delta_time;

        // Perform the game iteration
        while accumulator >= 16_667 {
            match sdl_app_iterate(&mut appstate) {
                Ok(()) => {}
                Err(_) => break 'running,
            }
            accumulator -= 16_667;
        }
    }

    // Clean up and quit
    sdl_app_quit(Some(Box::new(appstate)));
    Ok(())
}
