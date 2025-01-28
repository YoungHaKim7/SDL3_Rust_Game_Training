use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use std::collections::VecDeque;

const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 600;

struct Ball {
    x: i32,
    y: i32,
    dx: i32,
    dy: i32,
    radius: u32,
}

impl Ball {
    fn new(x: i32, y: i32) -> Ball {
        Ball {
            x,
            y,
            dx: 5,
            dy: -5,
            radius: 10,
        }
    }

    fn update(&mut self) {
        self.x += self.dx;
        self.y += self.dy;

        // Check wall collisions
        if self.x < self.radius as i32 || self.x > (SCREEN_WIDTH as i32 - self.radius as i32) {
            self.dx *= -1;
        }
        if self.y < self.radius as i32 {
            self.dy *= -1;
        }

        // Check paddle collision
        let paddle = Rect::new(350, 550, 100, 20);
        if self.get_rect().intersection(paddle).is_some() {
            self.dy *= -1;
        }

        // Check brick collisions (example with one brick)
        let bricks = VecDeque::from(vec![
            Rect::new(370, 200, 60, 40),
            Rect::new(370, 250, 60, 40),
        ]);
        for brick in &bricks {
            if self.get_rect().intersection(*brick).is_some() {
                self.dx *= -1;
                self.dy *= -1;
            }
        }
    }

    fn get_rect(&self) -> Rect {
        let radius = self.radius as i32;
        Rect::new(
            self.x - radius,
            self.y - radius,
            (radius * 2) as u32,
            (radius * 2) as u32,
        )
    }
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("Ball Physics", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().unwrap();
    // let _renderer = canvas.renderer();

    let mut ball = Ball::new(400, 300);

    let mut event_pump = sdl_context.event_pump()?;

    'mainloop: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'mainloop,
                _ => {}
            }
        }

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        // Draw ball
        let ball_rect = ball.get_rect();
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.fill_rect(ball_rect)?;

        // Draw paddle
        let paddle = Rect::new(350, 550, 100, 20);
        canvas.fill_rect(paddle)?;

        // Draw bricks (example with two bricks)
        let mut bricks = VecDeque::from(vec![
            Rect::new(370, 200, 60, 40),
            Rect::new(370, 250, 60, 40),
        ]);
        for brick in &mut bricks {
            canvas.fill_rect(*brick)?;
        }

        ball.update();

        canvas.present();
    }

    Ok(())
}
