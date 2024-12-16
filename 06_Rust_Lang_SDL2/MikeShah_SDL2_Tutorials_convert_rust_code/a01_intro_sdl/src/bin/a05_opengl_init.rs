// extern crate sdl2;

// extern crate gl;

// use sdl2::{event::Event, keyboard::Keycode, video::GLProfile};

// use std::time::Duration;

// fn main() -> Result<(), String> {
//     let sdl = sdl2::init().unwrap();
//     let video_subsystem = sdl.video().unwrap();

//     // OpenGL 설정
//     let gl_attr = video_subsystem.gl_attr();
//     gl_attr.set_context_profile(GLProfile::Core);
//     gl_attr.set_context_version(4, 1);
//     gl_attr.set_double_buffer(true);
//     gl_attr.set_depth_size(24);

//     // 윈도우 생성
//     let window = video_subsystem
//         .window("Rust SDL2 Window", 640, 480)
//         .opengl()
//         .position_centered()
//         .build()
//         .unwrap();

//     let window_clone = window.clone();
//     let _gl_context = window.gl_create_context().unwrap();
//     gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const _);

//     // 이벤트 루프
//     let mut event_pump = sdl.event_pump().unwrap();

//     let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

//     'running: loop {
//         for event in event_pump.poll_iter() {
//             match event {
//                 Event::Quit { .. }
//                 | Event::KeyDown {
//                     keycode: Some(Keycode::Escape),
//                     ..
//                 } => break 'running,
//                 // skip mouse motion intentionally because of the verbose it might cause.
//                 Event::MouseMotion { .. } => {}
//                 e => {
//                     println!("{:?}", e);
//                 }
//             }
//         }

//         unsafe {
//             gl::Viewport(0, 0, 640, 480);
//             gl::ClearColor(1.0, 0.0, 0.0, 1.0);
//             gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
//         }

//         canvas.clear();
//         canvas.present();
//         window_clone.gl_swap_window();
//         ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
//         // The rest of the game loop goes here...
//     }

//     Ok(())
// }

extern crate sdl2;

extern crate gl;

use sdl2::{event::Event, keyboard::Keycode, video::GLProfile};

use std::time::Duration;

fn main() -> Result<(), String> {
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

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        unsafe {
            gl::Viewport(0, 0, 640, 480);
            gl::ClearColor(1.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        window.gl_swap_window();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
    }

    Ok(())
}
