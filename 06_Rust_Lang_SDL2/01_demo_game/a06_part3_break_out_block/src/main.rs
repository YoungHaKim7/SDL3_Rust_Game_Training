use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

struct Brick {
    x: f64,
    y: f64,
}

impl Brick {
    fn new(x: f64, y: f64) -> Brick {
        Brick { x, y }
    }

    fn draw(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        let brick_width = 75;
        let brick_height = 30;

        let rect = Rect::new(self.x as i32, self.y as i32, brick_width, brick_height);

        canvas.set_draw_color(sdl2::pixels::Color::RGB(255, 0, 0));
        canvas.fill_rect(rect)?;

        Ok(())
    }
}

struct Paddle {
    x: f64,
    y: f64,
    width: f64,
    height: f64,
}

impl Paddle {
    fn new() -> Paddle {
        Paddle {
            x: 300.0, // Center of screen (assuming 600x600 window)
            y: 570.0, // Near the bottom
            width: 80.0,
            height: 10.0,
        }
    }

    fn move_left(&mut self) {
        if self.x > 0.0 + self.width / 2.0 {
            self.x -= 5.0;
        }
    }

    fn move_right(&mut self) {
        if self.x < (600.0 - self.width / 2.0) {
            self.x += 5.0;
        }
    }

    fn draw(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        let rect = Rect::new(
            self.x as i32 - (self.width / 2.0) as i32,
            self.y as i32,
            self.width as u32,
            self.height as u32,
        );

        canvas.set_draw_color(sdl2::pixels::Color::RGB(255, 255, 255));
        canvas.fill_rect(rect)?;

        Ok(())
    }
}

struct Ball {
    x: f64,
    y: f64,
    radius: f64,
}

impl Ball {
    fn new() -> Ball {
        Ball {
            x: 300.0,
            y: 300.0,
            radius: 10.0,
        }
    }

    fn draw(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        let rect = Rect::new(
            (self.x - self.radius) as i32,
            (self.y - self.radius) as i32,
            (self.radius * 2.0) as u32,
            (self.radius * 2.0) as u32,
        );

        canvas.set_draw_color(sdl2::pixels::Color::RGB(255, 255, 255));
        canvas.fill_rect(rect)?;

        Ok(())
    }
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Breakout", 600, 600)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    // let mut canvas = Canvas::from_window(window, RenderDriverType::OpenGL)?;
    let mut canvas: Canvas<Window> = window
        .into_canvas()
        .present_vsync() //< this means the screen cannot
        // render faster than your display rate (usually 60Hz or 144Hz)
        .build()
        .unwrap();

    let mut event_pump = sdl_context.event_pump()?;

    let mut paddle = Paddle::new();
    let mut ball = Ball::new();

    // Initialize bricks
    let num_bricks_per_row = 8;
    let brick_width = 75;
    let brick_height = 30;

    let bricks: Vec<Brick> = (0..num_bricks_per_row)
        .flat_map(|i| {
            (0..4).map(move |j| {
                Brick::new(
                    i as f64 * (brick_width + 2) as f64,
                    j as f64 * (brick_height + 5) as f64 + 30.0, // Add some spacing
                )
            })
        })
        .collect();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'running,
                _ => {}
            }
        }

        let keyboard_state = event_pump.keyboard_state();
        if keyboard_state.is_scancode_pressed(sdl2::keyboard::Scancode::Left) {
            paddle.move_left();
        }
        if keyboard_state.is_scancode_pressed(sdl2::keyboard::Scancode::Right) {
            paddle.move_right();
        }

        // Clear screen
        canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
        canvas.clear();

        // Draw bricks
        for brick in &bricks {
            brick.draw(&mut canvas)?;
        }

        // Draw paddle and ball
        paddle.draw(&mut canvas)?;
        ball.draw(&mut canvas)?;

        canvas.present();
    }

    Ok(())
}
