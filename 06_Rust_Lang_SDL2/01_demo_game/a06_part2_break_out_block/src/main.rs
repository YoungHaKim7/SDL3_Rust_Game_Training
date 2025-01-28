use sdl2::event::Event;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::keyboard::Scancode;
use sdl2::pixels::Color;

struct Brick {
    x: f64,
    y: f64,
    width: f64,
    height: f64,
}

impl Brick {
    fn new(x: f64, y: f64) -> Brick {
        Brick {
            x,
            y,
            width: 70.0,
            height: 25.0,
        }
    }

    fn draw(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) -> Result<(), String> {
        canvas.set_draw_color(sdl2::pixels::Color::RGB(255, 0, 0));
        canvas.fill_rect(sdl2::rect::Rect::new(50, 50, 100, 100));
        Ok(())
    }

    fn is_colliding(&self, ball_x: f64, ball_y: f64, ball_radius: f64) -> bool {
        let brick_left = self.x;
        let brick_right = self.x + self.width;
        let brick_top = self.y;
        let brick_bottom = self.y + self.height;

        // Check collision with any side of the brick
        let right_of_brick = ball_x - ball_radius > brick_right;
        let left_of_brick = ball_x + ball_radius < brick_left;
        let above_brick = ball_y - ball_radius > brick_top;
        let below_brick = ball_y + ball_radius < brick_bottom;

        !(right_of_brick || left_of_brick || above_brick || below_brick)
    }
}

struct Paddle {
    x: f64,
    y: f64,
    width: f64,
    height: f64,
}

impl Paddle {
    fn new(y_pos: f64) -> Paddle {
        Paddle {
            x: 0.0,
            y: y_pos,
            width: 120.0,
            height: 10.0,
        }
    }

    fn move_left(&mut self, ball_x: &mut f64, paddle_width: f64) {
        if self.x > 0.0 && (*ball_x >= self.x - 5.0 && *ball_x <= self.x + paddle_width) {
            *ball_x -= 5.0;
        }
        self.x = self.x.min(800.0 - self.width);
    }

    fn move_right(&mut self, ball_x: &mut f64, paddle_width: f64) {
        if (self.x + self.width) < 800.0
            && (*ball_x >= self.x && *ball_x <= self.x + paddle_width + 5.0)
        {
            *ball_x += 5.0;
        }
        self.x = self.x.max(0.0);
    }

    fn draw(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) -> Result<(), String> {
        canvas.set_draw_color(sdl2::pixels::Color::RGB(255, 255, 255));
        canvas.fill_rect(sdl2::rect::Rect::new(50, 50, 100, 100));
        Ok(())
    }
}

struct Ball {
    x: f64,
    y: f64,
    dx: f64,
    dy: f64,
    radius: f64,
}

impl Ball {
    fn new(x_pos: f64, y_pos: f64) -> Ball {
        Ball {
            x: x_pos,
            y: y_pos,
            dx: 5.0,
            dy: -7.0,
            radius: 10.0,
        }
    }

    fn update(&mut self, paddle_y: f64, bricks: &mut [Option<Brick>]) {
        // Ball physics
        self.x += self.dx;
        self.y += self.dy;

        // Collision with walls
        if self.x <= self.radius || self.x >= 800.0 - self.radius {
            self.dx *= -1.0;
        }
        if self.y <= self.radius {
            self.dy *= -1.0;
        }

        // Collision with paddle
        let mut paddle_collision = false;
        let paddle_top = paddle_y;
        let paddle_bottom = paddle_y + 10.0;
        if self.y >= paddle_top && self.y <= paddle_bottom {
            if self.x >= 0.0 && self.x <= 800.0 {
                self.dy *= -1.0;
                paddle_collision = true;
            }
        }

        // Collision with bricks
        for brick in bricks.iter_mut() {
            if let Some(ref mut b) = brick {
                if b.is_colliding(self.x, self.y, self.radius) {
                    // brick.set(None);
                    let mut pinned = std::pin::pin!(brick);
                    pinned.as_mut().set(&mut None);
                    self.dy *= -1.0;
                    break;
                }
            }
        }

        // Reset ball if it goes below paddle
        if self.y >= 600.0 + self.radius {
            self.reset();
        }
    }

    fn reset(&mut self) {
        self.x = 400.0;
        self.y = 300.0;
        self.dx = 5.0;
        self.dy = -7.0;
    }

    fn draw(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) -> Result<(), String> {
        canvas.set_draw_color(sdl2::pixels::Color::RGB(255, 255, 255));
        // canvas.fill_circle(self.x as i32, self.y as i32, self.radius as u16)?;
        canvas.circle(16, 16, 16, Color::RGBA(0, 0, 0, 255));
        Ok(())
    }
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Breakout", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas();

    // Initialize bricks
    const BRICKS_PER_ROW: usize = 12;
    const ROWS_OF_BRICKS: usize = 5;
    let mut bricks: Vec<Option<Brick>> = Vec::with_capacity(BRICKS_PER_ROW * ROWS_OF_BRICKS);
    for i in 0..ROWS_OF_BRICKS {
        for j in 0..BRICKS_PER_ROW {
            bricks.push(Some(Brick::new(
                (j as f64) * 75.0 + 20.0,
                (i as f64) * 35.0 + 20.0,
            )));
        }
    }

    let mut paddle = Paddle::new(590.0);
    let mut ball = Ball::new(400.0, 300.0);

    let mut event_pump = sdl_context.event_pump().unwrap();

    'mainloop: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'mainloop,
                _ => {}
            }
        }

        // Paddle movement
        match !event_pump.key_pressed(Scancode::Left) && !event_pump.key_pressed(Scancode::Right) {
            true => {
                paddle.x = (ball.x - 60.0).max(0.0).min(800.0 - paddle.width);
            }
            false => {
                if event_pump.key_pressed(Scancode::Left) {
                    paddle.move_left(&mut ball.x, paddle.width);
                }
                if event_pump.key_pressed(Scancode::Right) {
                    paddle.move_right(&mut ball.x, paddle.width);
                }
            }
        }

        // Update game state
        ball.update(paddle.y, &mut bricks);

        // Draw background
        canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
        canvas.clear();

        // Draw objects
        paddle.draw(&mut canvas)?;
        ball.draw(&mut canvas)?;

        for brick in bricks.iter_mut() {
            if let Some(ref b) = brick {
                b.draw(&mut canvas)?;
            }
        }

        // Update screen
        canvas.present();
    }

    Ok(())
}
