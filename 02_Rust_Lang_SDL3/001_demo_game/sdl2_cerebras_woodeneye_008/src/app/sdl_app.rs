use sdl2::event::Event;
use sdl2::keyboard::Keycode;

pub fn handle_event(event: Event, app_state: &mut AppState) {
    match event {
        Event::Quit { .. } => {
            app_state.debug_string = "Quit".to_string();
        }
        Event::KeyDown {
            keycode: Some(Keycode::W),
            ..
        } => {
            app_state.players[0].wasd |= 1;
        }
        Event::KeyDown {
            keycode: Some(Keycode::A),
            ..
        } => {
            app_state.players[0].wasd |= 2;
        }
        Event::KeyDown {
            keycode: Some(Keycode::S),
            ..
        } => {
            app_state.players[0].wasd |= 4;
        }
        Event::KeyDown {
            keycode: Some(Keycode::D),
            ..
        } => {
            app_state.players[0].wasd |= 8;
        }
        Event::KeyDown {
            keycode: Some(Keycode::Space),
            ..
        } => {
            app_state.players[0].wasd |= 16;
        }
        _ => {}
    }
}
