use sdl3::event::Event;
use sdl3::keyboard::Keycode;
use sdl3::mouse::MouseButton;
use sdl3::pixels::Color;
use sdl3::render::Canvas;
use sdl3::video::Window;
use sdl3::Sdl;
use std::sync::LazyLock;
use std::time::{Duration, Instant};

mod player;

use player::{draw, init_players, shoot, update, whose_keyboard, whose_mouse, Player};

const MAP_BOX_SCALE: i32 = 16;
const MAP_BOX_EDGES_LEN: usize = 12 + (MAP_BOX_SCALE * 2) as usize;
const MAX_PLAYER_COUNT: usize = 4;

// static mut DEBUG_STRING: [u8; 32] = [0; 32];

struct AppState {
    window: Window,
    renderer: Canvas<Window>,
    player_count: i32,
    players: [Player; MAX_PLAYER_COUNT],
    edges: [[f32; 6]; MAP_BOX_EDGES_LEN],
}

static mut DEBUG_STRING: [u8; 32] = [0; 32];
static LAST: LazyLock<Instant> = LazyLock::new(Instant::now);
static PAST: LazyLock<Instant> = LazyLock::new(Instant::now);

fn update_times() {
    let now = Instant::now();
    *LazyLock::force(&LAST) = now;
    *LazyLock::force(&PAST) = now;
}

fn sdl_app_iterate(app_state: &mut AppState) -> bool {
    static mut ACCU: u64 = 0;
    // static mut LAST: Instant = Instant::now();
    // static mut PAST: Instant = Instant::now();
    // static LAST: LazyLock<Instant> = LazyLock::new(Instant::now);
    // static PAST: LazyLock<Instant> = LazyLock::new(Instant::now);

    let now = Instant::now();
    let dt_ns = now.duration_since(unsafe { *PAST }).as_nanos() as u64;
    update(
        &mut app_state.players,
        app_state.player_count as usize,
        dt_ns,
    );
    draw(
        &mut app_state.renderer,
        &app_state.edges,
        &app_state.players,
        app_state.player_count as usize,
    );

    unsafe {
        if now.duration_since(*LAST).as_secs() >= 1 {
            LAST = now;
            let fps = ACCU;
            ACCU = 0;
            let debug_str = format!("{} fps", fps);
            DEBUG_STRING.copy_from_slice(debug_str.as_bytes());
        }
        PAST = now;
        ACCU += 1;
    }

    let elapsed = now.elapsed().as_nanos() as u64;
    if elapsed < 999_999 {
        std::thread::sleep(Duration::from_nanos(999_999 - elapsed));
    }

    false
}

fn sdl_app_quit(app_state: &mut AppState) {
    // SDL will clean up the window/renderer for us.
    // Just free the memory.
    drop(app_state);
}

fn init_edges(scale: i32, edges: &mut [[f32; 6]]) {
    let r = scale as f32;
    let map = [
        0, 1, 1, 3, 3, 2, 2, 0, 7, 6, 6, 4, 4, 5, 5, 7, 6, 2, 3, 7, 0, 4, 5, 1,
    ];
    for i in 0..12 {
        for j in 0..3 {
            edges[i][j + 0] = if map[i * 2 + 0] & (1 << j) != 0 {
                r
            } else {
                -r
            };
            edges[i][j + 3] = if map[i * 2 + 1] & (1 << j) != 0 {
                r
            } else {
                -r
            };
        }
    }
    for i in 0..scale {
        let d = (i * 2) as f32;
        for j in 0..2 {
            edges[i as usize + 12][3 * j + 0] = if j != 0 { r } else { -r };
            edges[i as usize + 12][3 * j + 1] = -r;
            edges[i as usize + 12][3 * j + 2] = d - r;
            edges[i as usize + scale as usize][3 * j + 0] = d - r;
            edges[i as usize + scale as usize][3 * j + 1] = -r;
            edges[i as usize + scale as usize][3 * j + 2] = if j != 0 { r } else { -r };
        }
    }
}

fn sdl_app_init(sdl_context: &Sdl) -> Result<AppState, String> {
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem
        .window("examples/demo/woodeneye-008", 640, 480)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;
    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    let mut app_state = AppState {
        window: canvas.window().clone(),
        renderer: canvas,
        player_count: 1,
        players: [Player::default(); MAX_PLAYER_COUNT],
        edges: [[0.0; 6]; MAP_BOX_EDGES_LEN],
    };

    init_players(&mut app_state.players);
    init_edges(MAP_BOX_SCALE, &mut app_state.edges);
    unsafe {
        DEBUG_STRING[0] = 0;
    }

    app_state.renderer.set_draw_color(Color::RGB(0, 0, 0));
    app_state.renderer.clear();
    app_state.renderer.present();

    Ok(app_state)
}

fn sdl_app_event(app_state: &mut AppState, event: Event) -> bool {
    let players = &mut app_state.players;
    let player_count = app_state.player_count;
    match event {
        Event::Quit { .. } => return true,
        Event::MouseMotion {
            which, xrel, yrel, ..
        } => {
            let index = whose_mouse(which, players);
            if let Some(index) = index {
                players[index].yaw = players[index]
                    .yaw
                    .wrapping_sub((xrel as i32 * 0x00080000) as u32);
                players[index].pitch = (players[index].pitch - (yrel as i32 * 0x00080000))
                    .clamp(-0x40000000, 0x40000000);
            } else if which != 0 {
                for player in players.iter_mut() {
                    if player.mouse == 0 {
                        player.mouse = which;
                        app_state.player_count = app_state.player_count.max(players.len() as i32);
                        break;
                    }
                }
            }
        }
        Event::MouseButtonDown {
            which, mouse_btn, ..
        } => {
            if mouse_btn == MouseButton::Left {
                let index = whose_mouse(which, players);
                if let Some(index) = index {
                    shoot(index, players);
                }
            }
        }
        Event::KeyDown { keycode, which, .. } => {
            if let Some(keycode) = keycode {
                let index = whose_keyboard(which, players);
                if let Some(index) = index {
                    match keycode {
                        Keycode::W => players[index].wasd |= 1,
                        Keycode::A => players[index].wasd |= 2,
                        Keycode::S => players[index].wasd |= 4,
                        Keycode::D => players[index].wasd |= 8,
                        Keycode::Space => players[index].wasd |= 16,
                        _ => {}
                    }
                } else if which != 0 {
                    for player in players.iter_mut() {
                        if player.keyboard == 0 {
                            player.keyboard = which;
                            app_state.player_count =
                                app_state.player_count.max(players.len() as i32);
                            break;
                        }
                    }
                }
            }
        }
        Event::KeyUp { keycode, which, .. } => {
            if let Some(keycode) = keycode {
                if keycode == Keycode::Escape {
                    return true;
                }
                let index = whose_keyboard(which, players);
                if let Some(index) = index {
                    match keycode {
                        Keycode::W => players[index].wasd &= 30,
                        Keycode::A => players[index].wasd &= 29,
                        Keycode::S => players[index].wasd &= 27,
                        Keycode::D => players[index].wasd &= 23,
                        Keycode::Space => players[index].wasd &= 15,
                        _ => {}
                    }
                }
            }
        }
        _ => {}
    }
    false
}

fn main() -> Result<(), String> {
    let sdl_context = sdl3::init()?;
    let mut app_state = sdl_app_init(&sdl_context)?;

    let mut event_pump = sdl_context.event_pump()?;
    'running: loop {
        for event in event_pump.poll_iter() {
            if sdl_app_event(&mut app_state, event) {
                break 'running;
            }
        }
        if sdl_app_iterate(&mut app_state) {
            break 'running;
        }
    }

    sdl_app_quit(&mut app_state);
    Ok(())
}
