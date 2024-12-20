use super::app_state::AppState;

pub fn app_quit(app_state: &mut AppState, result: i32) {
    if app_state != std::ptr::null_mut() {
        std::mem::drop(app_state);
    }
}
