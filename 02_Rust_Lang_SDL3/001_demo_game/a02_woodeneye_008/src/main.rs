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

fn main() -> Result<(), String> {
    Ok(())
}
