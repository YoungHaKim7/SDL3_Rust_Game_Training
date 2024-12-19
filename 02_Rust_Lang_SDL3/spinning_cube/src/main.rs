extern crate sdl2;

extern crate gl;

use sdl2::{
    event::Event,
    keyboard::Keycode,
    video::{GLProfile, Window},
};

use std::{
    ffi::CString,
    ptr,
};

fn main() {
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();

    // OpenGL 설정
    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(GLProfile::Core);
    gl_attr.set_context_version(4, 1);
    gl_attr.set_double_buffer(true);
    gl_attr.set_depth_size(24);

    // 윈도우 생성
    let window = video_subsystem
        .window("Rust SDL2 Window", 640, 480)
        .opengl()
        .position_centered()
        .build()
        .unwrap();

    let _gl_context = window.gl_create_context().unwrap();
    gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const _);

    // 이벤트 루프
    let mut event_pump = sdl.event_pump().unwrap();
    let mut running = true;

    while running {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => running = false,
                Event::MouseMotion { .. } => println!("mouse has been moved"),
                Event::KeyDown {
                    keycode: Some(Keycode::Num0),
                    ..
                } => println!("0 was pressed"),
                Event::KeyDown { .. } => println!("a key has been pressed"),
                _ => {}
            }
        }

        let keyboard_state = event_pump.keyboard_state();
        if keyboard_state.is_scancode_pressed(sdl2::keyboard::Scancode::Right) {
            println!("right arrow key is pressed");
        }

        unsafe {
            gl::Viewport(0, 0, 640, 480);
            gl::ClearColor(1.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        window.gl_swap_window();
    }

    // SDL 종료
    sdl.delay(3000);
}
