// // use sdl2::event::Event;
// // use sdl2::keyboard::Keycode;
// // use sdl2::pixels::Color;
// // use sdl2::rect::Rect;
// // use sdl2::render::Canvas;
// // use sdl2::video::Window;
// // use sdl2::Sdl;
// // use std::collections::HashSet;
// // use std::f64::consts::PI;
// // use std::time::{Duration, Instant};

// // const SCREEN_WIDTH: u32 = 640;
// // const SCREEN_HEIGHT: u32 = 480;
// // const FOV: f64 = PI / 3.0; // 60 degrees field of view
// // const PLAYER_SPEED: f64 = 100.0; // Units per second
// // const ROTATION_SPEED: f64 = 3.0; // Radians per second

// // struct Player {
// //     x: f64,
// //     y: f64,
// //     angle: f64,
// // }

// // struct Wall {
// //     x1: f64,
// //     y1: f64,
// //     x2: f64,
// //     y2: f64,
// // }

// // fn main() -> Result<(), String> {
// //     let sdl_context: Sdl = sdl2::init()?;
// //     let video_subsystem = sdl_context.video()?;
// //     let window = video_subsystem
// //         .window("First Person View", SCREEN_WIDTH, SCREEN_HEIGHT)
// //         .position_centered()
// //         .build()
// //         .map_err(|e| e.to_string())?;
// //     let mut canvas: Canvas<Window> = window
// //         .into_canvas()
// //         .present_vsync()
// //         .build()
// //         .map_err(|e| e.to_string())?;
// //     let mut event_pump = sdl_context.event_pump()?;
// //     let mut keys_pressed = HashSet::new();

// //     let mut player = Player {
// //         x: 100.0,
// //         y: 100.0,
// //         angle: 0.0,
// //     };

// //     let wall = Wall {
// //         x1: 300.0,
// //         y1: 100.0,
// //         x2: 300.0,
// //         y2: 300.0,
// //     };

// //     let mut last_frame_time = Instant::now();

// //     'running: loop {
// //         for event in event_pump.poll_iter() {
// //             match event {
// //                 Event::Quit { .. }
// //                 | Event::KeyDown {
// //                     keycode: Some(Keycode::Escape),
// //                     ..
// //                 } => break 'running,
// //                 Event::KeyDown {
// //                     keycode: Some(key), ..
// //                 } => {
// //                     keys_pressed.insert(key);
// //                 }
// //                 Event::KeyUp {
// //                     keycode: Some(key), ..
// //                 } => {
// //                     keys_pressed.remove(&key);
// //                 }
// //                 _ => {}
// //             }
// //         }

// //         let now = Instant::now();
// //         let delta_time = now.duration_since(last_frame_time).as_secs_f64();
// //         last_frame_time = now;

// //         update_player(&mut player, &keys_pressed, delta_time);

// //         canvas.set_draw_color(Color::RGB(0, 0, 0));
// //         canvas.clear();
// //         render_view(&mut canvas, &player, &wall)?;
// //         canvas.present();

// //         ::std::thread::sleep(Duration::from_millis(16));
// //     }

// //     Ok(())
// // }

// // fn update_player(player: &mut Player, keys_pressed: &HashSet<Keycode>, dt: f64) {
// //     if keys_pressed.contains(&Keycode::W) {
// //         player.x += player.angle.cos() * PLAYER_SPEED * dt;
// //         player.y += player.angle.sin() * PLAYER_SPEED * dt;
// //     }
// //     if keys_pressed.contains(&Keycode::S) {
// //         player.x -= player.angle.cos() * PLAYER_SPEED * dt;
// //         player.y -= player.angle.sin() * PLAYER_SPEED * dt;
// //     }
// //     if keys_pressed.contains(&Keycode::A) {
// //         player.angle -= ROTATION_SPEED * dt;
// //     }
// //     if keys_pressed.contains(&Keycode::D) {
// //         player.angle += ROTATION_SPEED * dt;
// //     }
// // }

// // fn render_view(canvas: &mut Canvas<Window>, player: &Player, wall: &Wall) -> Result<(), String> {
// //     let ray_count = SCREEN_WIDTH as usize;
// //     let ray_step = FOV / ray_count as f64;

// //     // for (i, ray_angle) in (0..ray_count).map(|i| player.angle - FOV / 2.0 + i as f64 * ray_step) {
// //     for (i, ray_angle) in (0..ray_count)
// //         .enumerate()
// //         .map(|(i, _)| (i, player.angle - FOV / 2.0 + i as f64 * ray_step))
// //     {
// //         let (hit, distance) = cast_ray(player, ray_angle, wall);
// //         if hit {
// //             let wall_height = (SCREEN_HEIGHT as f64 / distance).min(SCREEN_HEIGHT as f64);
// //             let wall_top = (SCREEN_HEIGHT as f64 / 2.0 - wall_height / 2.0) as i32;
// //             canvas.set_draw_color(Color::RGB(255, 255, 255));
// //             canvas.fill_rect(Rect::new(i as i32, wall_top, 1, wall_height as u32))?;
// //         }
// //     }
// //     Ok(())
// // }

// // fn cast_ray(player: &Player, angle: f64, wall: &Wall) -> (bool, f64) {
// //     let dx = angle.cos();
// //     let dy = angle.sin();

// //     let t1 = (wall.x1 - player.x) * dy - (wall.y1 - player.y) * dx;
// //     let t2 = (wall.x2 - player.x) * dy - (wall.y2 - player.y) * dx;
// //     let u1 =
// //         (wall.y1 - player.y) * (wall.x2 - wall.x1) - (wall.x1 - player.x) * (wall.y2 - wall.y1);
// //     let u2 = dx * (wall.y2 - wall.y1) - dy * (wall.x2 - wall.x1);

// //     if u2 != 0.0 && (t1 < 0.0) != (t2 < 0.0) {
// //         let u = u1 / u2;
// //         if u > 0.0 {
// //             return (true, u);
// //         }
// //     }
// //     (false, f64::MAX)
// // }

// use sdl2::event::Event;
// use sdl2::keyboard::Keycode;
// use sdl2::pixels::Color;
// use sdl2::rect::{Point, Rect};
// use sdl2::render::Canvas;
// use sdl2::video::Window;
// use sdl2::Sdl;
// use std::collections::HashSet;
// use std::f64::consts::PI;
// use std::time::{Duration, Instant};

// const SCREEN_WIDTH: u32 = 640;
// const SCREEN_HEIGHT: u32 = 480;
// const FOV: f64 = PI / 3.0; // 60 degrees field of view
// const PLAYER_SPEED: f64 = 100.0; // Units per second
// const ROTATION_SPEED: f64 = 3.0; // Radians per second

// struct Player {
//     x: f64,
//     y: f64,
//     angle: f64,
// }

// struct Wall {
//     x: f64,
//     y: f64,
//     width: f64,
//     height: f64,
// }

// fn main() -> Result<(), String> {
//     let sdl_context: Sdl = sdl2::init()?;
//     let video_subsystem = sdl_context.video()?;
//     let window = video_subsystem
//         .window("First Person View", SCREEN_WIDTH, SCREEN_HEIGHT)
//         .position_centered()
//         .build()
//         .map_err(|e| e.to_string())?;
//     let mut canvas: Canvas<Window> = window
//         .into_canvas()
//         .present_vsync()
//         .build()
//         .map_err(|e| e.to_string())?;
//     let mut event_pump = sdl_context.event_pump()?;
//     let mut keys_pressed = HashSet::new();

//     let mut player = Player {
//         x: 100.0,
//         y: 100.0,
//         angle: 0.0,
//     };

//     // Define a wall that blocks the view
//     let wall = Wall {
//         x: 320.0, // Centered on the screen
//         y: 200.0, // Halfway down the screen
//         width: 400.0,
//         height: 200.0,
//     };

//     let mut last_frame_time = Instant::now();

//     'running: loop {
//         for event in event_pump.poll_iter() {
//             match event {
//                 Event::Quit { .. }
//                 | Event::KeyDown {
//                     keycode: Some(Keycode::Escape),
//                     ..
//                 } => break 'running,
//                 Event::KeyDown {
//                     keycode: Some(key), ..
//                 } => {
//                     keys_pressed.insert(key);
//                 }
//                 Event::KeyUp {
//                     keycode: Some(key), ..
//                 } => {
//                     keys_pressed.remove(&key);
//                 }
//                 _ => {}
//             }
//         }

//         let now = Instant::now();
//         let delta_time = now.duration_since(last_frame_time).as_secs_f64();
//         last_frame_time = now;

//         update_player(&mut player, &keys_pressed, delta_time);

//         canvas.set_draw_color(Color::RGB(0, 0, 0));
//         canvas.clear();

//         // Render the wall in front of the player
//         render_wall(&mut canvas, &wall)?;

//         canvas.present();

//         ::std::thread::sleep(Duration::from_millis(16));
//     }

//     Ok(())
// }

// fn update_player(player: &mut Player, keys_pressed: &HashSet<Keycode>, dt: f64) {
//     if keys_pressed.contains(&Keycode::W) {
//         player.x += player.angle.cos() * PLAYER_SPEED * dt;
//         player.y += player.angle.sin() * PLAYER_SPEED * dt;
//     }
//     if keys_pressed.contains(&Keycode::S) {
//         player.x -= player.angle.cos() * PLAYER_SPEED * dt;
//         player.y -= player.angle.sin() * PLAYER_SPEED * dt;
//     }
//     if keys_pressed.contains(&Keycode::A) {
//         player.angle -= ROTATION_SPEED * dt;
//     }
//     if keys_pressed.contains(&Keycode::D) {
//         player.angle += ROTATION_SPEED * dt;
//     }
// }

// fn render_wall(canvas: &mut Canvas<Window>, wall: &Wall) -> Result<(), String> {
//     canvas.set_draw_color(Color::RGB(255, 255, 255)); // White color for the wall
//     let rect = Rect::new(
//         (wall.x - wall.width / 2.0) as i32,
//         (wall.y - wall.height / 2.0) as i32,
//         wall.width as u32,
//         wall.height as u32,
//     );

//     canvas.fill_rect(rect)?;

//     Ok(())
// }

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::Sdl;
use std::collections::HashSet;
use std::f64::consts::PI;
use std::time::{Duration, Instant};

const SCREEN_WIDTH: u32 = 640;
const SCREEN_HEIGHT: u32 = 480;
const PLAYER_SPEED: f64 = 100.0; // Units per second
const ROTATION_SPEED: f64 = 2.0; // Radians per second
const WALL_WIDTH: f64 = 800.0; // Double the horizontal length
const WALL_HEIGHT: f64 = 200.0;
const WALL_DISTANCE: f64 = 400.0;

struct Player {
    x: f64,
    y: f64,
    angle: f64,
    vertical_angle: f64,
}

fn main() -> Result<(), String> {
    let sdl_context: Sdl = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem
        .window("First Person Wall View", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;
    let mut canvas: Canvas<Window> = window
        .into_canvas()
        .present_vsync()
        .build()
        .map_err(|e| e.to_string())?;
    let mut event_pump = sdl_context.event_pump()?;
    let mut keys_pressed = HashSet::new();

    let mut player = Player {
        x: 0.0,
        y: 0.0,
        angle: 0.0,
        vertical_angle: 0.0,
    };

    let mut last_frame_time = Instant::now();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(key), ..
                } => {
                    keys_pressed.insert(key);
                }
                Event::KeyUp {
                    keycode: Some(key), ..
                } => {
                    keys_pressed.remove(&key);
                }
                _ => {}
            }
        }

        let now = Instant::now();
        let delta_time = now.duration_since(last_frame_time).as_secs_f64();
        last_frame_time = now;

        update_player(&mut player, &keys_pressed, delta_time);

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        render_wall(&mut canvas, &player)?;

        canvas.present();

        ::std::thread::sleep(Duration::from_millis(16));
    }

    Ok(())
}

fn update_player(player: &mut Player, keys_pressed: &HashSet<Keycode>, dt: f64) {
    if keys_pressed.contains(&Keycode::A) {
        player.x -= PLAYER_SPEED * dt;
    }
    if keys_pressed.contains(&Keycode::D) {
        player.x += PLAYER_SPEED * dt;
    }
    if keys_pressed.contains(&Keycode::W) {
        player.vertical_angle = (player.vertical_angle - ROTATION_SPEED * dt).max(-PI / 3.0);
    }
    if keys_pressed.contains(&Keycode::S) {
        player.vertical_angle = (player.vertical_angle + ROTATION_SPEED * dt).min(PI / 3.0);
    }
}

fn render_wall(canvas: &mut Canvas<Window>, player: &Player) -> Result<(), String> {
    let perspective_factor = 1.0 + player.x / WALL_DISTANCE;
    let wall_width = WALL_WIDTH * perspective_factor;
    let wall_height = WALL_HEIGHT * perspective_factor;

    let x_offset = -player.x * perspective_factor;
    let y_offset = player.vertical_angle * SCREEN_HEIGHT as f64 / 2.0;

    let rect = Rect::new(
        ((SCREEN_WIDTH as f64 - wall_width) / 2.0 + x_offset) as i32,
        ((SCREEN_HEIGHT as f64 - wall_height) / 2.0 + y_offset) as i32,
        wall_width as u32,
        wall_height as u32,
    );

    // Calculate color based on perspective
    let brightness = (255.0 * (1.0 - perspective_factor.abs() / 2.0)) as u8;
    canvas.set_draw_color(Color::RGB(brightness, brightness, brightness));
    canvas.fill_rect(rect)?;

    Ok(())
}
