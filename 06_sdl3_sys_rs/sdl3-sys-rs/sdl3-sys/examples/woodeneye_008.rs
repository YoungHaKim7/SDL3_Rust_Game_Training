use error::SDL_GetError;
use events::SDL_PollEvent;
use init::{SDL_Init, SDL_Quit, SDL_INIT_VIDEO};
use rect::SDL_Rect;
use sdl3_sys::{keyboard::SDL_KeyboardID, mouse::SDL_MouseID, render::*, video::*, *};
use std::ffi::{c_char, CStr, CString};

const MAP_BOX_SCALE: usize = 16;
const MAP_BOX_EDGES_LEN: usize = 12 + MAP_BOX_SCALE * 2;
const MAX_PLAYER_COUNT: usize = 4;

#[repr(C)]
struct Player {
    mouse: SDL_MouseID,
    keyboard: SDL_KeyboardID,
    pos: [f64; 3],
    vel: [f64; 3],
    yaw: u32,
    pitch: i32,
    radius: f32,
    height: f32,
    color: [u8; 3], // Fixed array size for color
    wasd: u8,
}

#[repr(C)]
struct AppState {
    window: *mut SDL_Window,
    renderer: *mut SDL_Renderer,
    player_count: i32,
    players: [Player; MAX_PLAYER_COUNT],
    edges: [[f32; 6]; MAP_BOX_EDGES_LEN],
}

#[repr(C)]
struct Metadata {
    key: *const c_char,
    value: *const c_char,
}

static EXTENDED_METADATA: &[Metadata] = &[
    Metadata {
        key: sdl3_sys::init::SDL_PROP_APP_METADATA_URL_STRING,
        value: c"https://examples.libsdl.org/SDL3/game/02-woodeneye-008/".as_ptr(),
    },
    Metadata {
        key: sdl3_sys::init::SDL_PROP_APP_METADATA_CREATOR_STRING,
        value: c"SDL team".as_ptr(),
    },
    Metadata {
        key: sdl3_sys::init::SDL_PROP_APP_METADATA_COPYRIGHT_STRING,
        value: c"Placed in the public domain".as_ptr(),
    },
    Metadata {
        key: sdl3_sys::init::SDL_PROP_APP_METADATA_TYPE_STRING,
        value: c"game".as_ptr(),
    },
];

fn whose_mouse(mouse: SDL_MouseID, players: &[Player]) -> i32 {
    players
        .iter()
        .position(|p| p.mouse == mouse)
        .map_or(-1, |i| i as i32)
}

fn whose_keyboard(keyboard: SDL_KeyboardID, players: &[Player]) -> i32 {
    players
        .iter()
        .position(|p| p.keyboard == keyboard)
        .map_or(-1, |i| i as i32)
}

impl Player {
    fn update_position(&mut self) {
        self.pos[0] += self.vel[0];
        self.pos[1] += self.vel[1];
        self.pos[2] += self.vel[2];
    }

    fn render(&self, renderer: *mut SDL_Renderer) {
        unsafe {
            let rect = SDL_Rect {
                x: self.pos[0] as i32,
                y: self.pos[1] as i32,
                w: 50,
                h: 50,
            };
            SDL_SetRenderDrawColor(renderer, self.color[0], self.color[1], self.color[2], 255);
            SDL_RenderFillRect(renderer, &rect as *const _);
        }
    }
}

fn handle_events(event: &sdl3_sys::SDL_Event, player: &mut Player, running: &mut bool) {
    unsafe {
        if event.type_ == sdl3_sys::SDL_EventType::SDL_QUIT as u32 {
            *running = false;
        } else if event.type_ == sdl3_sys::SDL_EventType::SDL_KEYDOWN as u32 {
            let key = event.key.keysym.sym;
            match key {
                sdl3_sys::SDL_Scancode::SDL_SCANCODE_LEFT => player.vel[0] = -5.0,
                sdl3_sys::SDL_Scancode::SDL_SCANCODE_RIGHT => player.vel[0] = 5.0,
                sdl3_sys::SDL_Scancode::SDL_SCANCODE_UP => player.vel[1] = -5.0,
                sdl3_sys::SDL_Scancode::SDL_SCANCODE_DOWN => player.vel[1] = 5.0,
                _ => (),
            }
        } else if event.type_ == sdl3_sys::SDL_EventType::SDL_KEYUP as u32 {
            let key = event.key.keysym.sym;
            match key {
                sdl3_sys::SDL_Scancode::SDL_SCANCODE_LEFT | sdl3_sys::SDL_SCANCODE_RIGHT => {
                    player.vel[0] = 0.0
                }
                sdl3_sys::SDL_Scancode::SDL_SCANCODE_UP
                | sdl3_sys::SDL_Scancode::SDL_SCANCODE_DOWN => player.vel[1] = 0.0,
                _ => (),
            }
        }
    }
}

fn main() {
    unsafe {
        if SDL_Init(SDL_INIT_VIDEO) != 0 {
            eprintln!(
                "Failed to initialize SDL: {}",
                CStr::from_ptr(SDL_GetError())
                    .to_str()
                    .unwrap_or("Unknown error")
            );
            return;
        }

        let window_name = CString::new("SDL3-sys Example").unwrap();
        let window = SDL_CreateWindow(
            window_name.as_ptr(),
            800,
            600,
            SDL_WindowFlags::SDL_WINDOW_SHOWN as u32,
        );

        if window.is_null() {
            eprintln!(
                "Failed to create window: {}",
                CStr::from_ptr(SDL_GetError())
                    .to_str()
                    .unwrap_or("Unknown error")
            );
            SDL_Quit();
            return;
        }

        let renderer = SDL_CreateRenderer(
            window,
            -1,
            SDL_RendererFlags::SDL_RENDERER_ACCELERATED as u32,
        );

        if renderer.is_null() {
            eprintln!(
                "Failed to create renderer: {}",
                CStr::from_ptr(SDL_GetError())
                    .to_str()
                    .unwrap_or("Unknown error")
            );
            SDL_DestroyWindow(window);
            SDL_Quit();
            return;
        }

        let mut player = Player {
            mouse: 0,
            keyboard: 0,
            pos: [375.0, 275.0, 0.0],
            vel: [0.0, 0.0, 0.0],
            yaw: 0,
            pitch: 0,
            radius: 1.0,
            height: 1.0,
            color: [0, 255, 0],
            wasd: 0,
        };

        let mut event = sdl3_sys::SDL_Event { type_: 0 };
        let mut running = true;

        while running {
            while SDL_PollEvent(&mut event as *mut _) != 0 {
                handle_events(&event, &mut player, &mut running);
            }

            player.update_position();

            SDL_SetRenderDrawColor(renderer, 0, 0, 0, 255);
            SDL_RenderClear(renderer);

            player.render(renderer);

            SDL_RenderPresent(renderer);
        }

        SDL_DestroyRenderer(renderer);
        SDL_DestroyWindow(window);
        SDL_Quit();
    }
}
