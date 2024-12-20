pub fn app_iterate(app_state: &mut AppState) -> i32 {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos() as u64;
    let dt_ns = now - app_state.past;
    app_state.past = now;
    let accu = app_state.accu + 1;
    if now - app_state.last > 999999999 {
        app_state.last = now;
        app_state.debug_string = format!("{} fps", accu);
        app_state.accu = 0;
    }
    if now - app_state.now > 999999 {
        app_state.now = now;
        std::thread::sleep(std::time::Duration::from_nanos(
            999999 - (now - app_state.now),
        ));
    }
    let result = update(&mut app_state.players, app_state.player_count, dt_ns);
    if result != 0 {
        return result;
    }
    let result = draw(
        &app_state.renderer,
        &app_state.edges,
        &app_state.players,
        app_state.player_count,
    );
    if result != 0 {
        return result;
    }
    app_state.accu = accu;
    SDL_APP_CONTINUE
}
