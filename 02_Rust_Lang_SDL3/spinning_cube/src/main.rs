// // https://en.wikipedia.org/wiki/Rotation_matrix
extern crate sdl3;

use sdl3::{
    event::Event, keyboard::Keycode, pixels::Color, rect::Point, render::Canvas, video::Window,
};
use std::time::Duration;

const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;
const CUBE_SIZE: f32 = 100.0;

#[derive(Clone, Copy)]
struct Vertex {
    x: f32,
    y: f32,
    z: f32,
}

impl Vertex {
    fn rotate_x(&self, angle: f32) -> Vertex {
        Vertex {
            x: self.x,
            y: self.y * angle.cos() - self.z * angle.sin(),
            z: self.y * angle.sin() + self.z * angle.cos(),
        }
    }

    fn rotate_y(&self, angle: f32) -> Vertex {
        Vertex {
            x: self.x * angle.cos() + self.z * angle.sin(),
            y: self.y,
            z: -self.x * angle.sin() + self.z * angle.cos(),
        }
    }

    fn rotate_z(&self, angle: f32) -> Vertex {
        Vertex {
            x: self.x * angle.cos() - self.y * angle.sin(),
            y: self.x * angle.sin() + self.y * angle.cos(),
            z: self.z,
        }
    }

    fn project(&self, width: u32, height: u32, fov: f32, viewer_distance: f32) -> (i32, i32) {
        let factor = fov / (viewer_distance + self.z);
        let x = self.x * factor + width as f32 / 2.0;
        let y = -self.y * factor + height as f32 / 2.0;
        (x as i32, y as i32)
    }
}

fn draw_line(canvas: &mut Canvas<Window>, p1: (i32, i32), p2: (i32, i32), color: Color) {
    canvas.set_draw_color(color);
    canvas
        .draw_line(Point::new(p1.0, p1.1), Point::new(p2.0, p2.1))
        .unwrap();
}

fn main() -> Result<(), String> {
    let sdl_context = sdl3::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("SDL3 Rotating Cube", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas();
    let mut event_pump = sdl_context.event_pump()?;

    let vertices = [
        Vertex {
            x: -CUBE_SIZE,
            y: -CUBE_SIZE,
            z: -CUBE_SIZE,
        },
        Vertex {
            x: CUBE_SIZE,
            y: -CUBE_SIZE,
            z: -CUBE_SIZE,
        },
        Vertex {
            x: CUBE_SIZE,
            y: CUBE_SIZE,
            z: -CUBE_SIZE,
        },
        Vertex {
            x: -CUBE_SIZE,
            y: CUBE_SIZE,
            z: -CUBE_SIZE,
        },
        Vertex {
            x: -CUBE_SIZE,
            y: -CUBE_SIZE,
            z: CUBE_SIZE,
        },
        Vertex {
            x: CUBE_SIZE,
            y: -CUBE_SIZE,
            z: CUBE_SIZE,
        },
        Vertex {
            x: CUBE_SIZE,
            y: CUBE_SIZE,
            z: CUBE_SIZE,
        },
        Vertex {
            x: -CUBE_SIZE,
            y: CUBE_SIZE,
            z: CUBE_SIZE,
        },
    ];

    let edges = [
        (0, 1),
        (1, 2),
        (2, 3),
        (3, 0),
        (4, 5),
        (5, 6),
        (6, 7),
        (7, 4),
        (0, 4),
        (1, 5),
        (2, 6),
        (3, 7),
    ];

    let mut angle_x = 0.0;
    let mut angle_y = 0.0;
    let mut angle_z = 0.0;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'running;
                }
                _ => {}
            }
        }

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        let transformed_vertices: Vec<Vertex> = vertices
            .iter()
            .map(|&v| v.rotate_x(angle_x).rotate_y(angle_y).rotate_z(angle_z))
            .collect();

        for &(start, end) in &edges {
            let p1 = transformed_vertices[start].project(WINDOW_WIDTH, WINDOW_HEIGHT, 256.0, 4.0);
            let p2 = transformed_vertices[end].project(WINDOW_WIDTH, WINDOW_HEIGHT, 256.0, 4.0);
            draw_line(&mut canvas, p1, p2, Color::RGB(255, 255, 255));
        }

        canvas.present();

        angle_x += 0.01;
        angle_y += 0.01;
        angle_z += 0.01;

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
