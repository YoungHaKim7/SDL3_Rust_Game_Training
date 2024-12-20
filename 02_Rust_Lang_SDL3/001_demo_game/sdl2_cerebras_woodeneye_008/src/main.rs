extern crate sdl2;

use std::thread;
use std::time::Duration;

use crate::app::app_iterate::app_iterate;
use crate::app::app_quit::app_quit;
use app::app_state::AppState;

mod app;

fn main() {
    let mut app_state = AppState::new();
    loop {
        let result = app_iterate(&mut app_state);
        if result != SDL_APP_CONTINUE {
            break;
        }
    }
    app_quit(&mut app_state, result);
}
