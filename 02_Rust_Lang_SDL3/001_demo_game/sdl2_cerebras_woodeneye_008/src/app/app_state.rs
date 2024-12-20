use sdl2::event::Event;

use super::sdl_app::handle_event;
use super::{player::Player, renderer::Renderer};

pub struct AppState {
    pub players: Vec<Player>,
    pub player_count: usize,
    pub renderer: Renderer,
    pub edges: Vec<[f32; 6]>,
    pub past: u64,
    pub last: u64,
    pub now: u64,
    pub accu: u64,
    pub debug_string: String,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            players: Vec::new(),
            player_count: 0,
            renderer: Renderer {},
            edges: Vec::new(),
            past: 0,
            last: 0,
            now: 0,
            accu: 0,
            debug_string: String::new(),
        }
    }
    pub fn handle_event(&mut self, event: Event) {
        handle_event(event, self);
    }
    pub fn app_iterate(&mut self) -> i32 {
        self.app_iterate()
    }
}
