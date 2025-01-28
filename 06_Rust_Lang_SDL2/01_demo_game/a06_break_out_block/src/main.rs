use sdl2::pixels::Color;
use sdl2::rect::Rect;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    const WINDOW_WIDTH: u32 = 800;
    const WINDOW_HEIGHT: u32 = 600;

    let window = video_subsystem
        .window("Breakout", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    #[derive(Debug)]
    struct Ball {
        x: f64,
        y: f64,
        radius: f64,
        dx: f64,
        dy: f64,
    }

    impl Ball {
        fn new(x: f64, y: f64) -> Ball {
            Ball {
                x,
                y,
                radius: 20.0,
                dx: 5.0,
                dy: 5.0,
            }
        }

        fn update(&mut self, width: u32, height: u32) {
            self.x += self.dx;
            self.y += self.dy;

            if self.x + self.radius > width as f64 || self.x - self.radius < 0.0 {
                self.dx *= -1.0;
            }
            if self.y + self.radius > height as f64 || self.y - self.radius < 0.0 {
                self.dy *= -1.0;
            }
        }

        fn draw(
            &self,
            canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
        ) -> Result<(), String> {
            let ball_rect = Rect::new(
                (self.x - self.radius) as i32,
                (self.y - self.radius) as i32,
                (self.radius * 2.0) as u32,
                (self.radius * 2.0) as u32,
            );

            canvas.set_draw_color(Color::RGB(0, 0, 255));
            canvas.fill_rect(ball_rect)?;
            Ok(())
        }
    }

    #[derive(Debug)]
    struct Paddle {
        x: f64,
        y: f64,
        width: f64,
        height: f64,
    }

    impl Paddle {
        fn new(y: f64) -> Paddle {
            Paddle {
                x: (WINDOW_WIDTH as f64 - 100.0) / 2.0,
                y,
                width: 100.0,
                height: 10.0,
            }
        }

        fn move_left(&mut self) {
            if self.x > 0.0 {
                self.x -= 7.0;
            }
        }

        fn move_right(&mut self) {
            if self.x + self.width < WINDOW_WIDTH as f64 {
                self.x += 7.0;
            }
        }

        fn draw(
            &self,
            canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
        ) -> Result<(), String> {
            let paddle_rect = Rect::new(
                self.x as i32,
                self.y as i32,
                self.width as u32,
                self.height as u32,
            );

            canvas.set_draw_color(Color::RGB(0, 255, 0));
            canvas.fill_rect(paddle_rect)?;
            Ok(())
        }
    }

    let mut ball = Ball::new(WINDOW_WIDTH as f64 / 2.0, WINDOW_HEIGHT as f64 / 2.0);
    let mut paddle = Paddle::new(WINDOW_HEIGHT as f64 - 50.0);

    let mut event_pump = sdl_context.event_pump()?;
    let mut game_loop = true;

    while game_loop {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        ball.update(WINDOW_WIDTH, WINDOW_HEIGHT);
        paddle.draw(&mut canvas)?;
        ball.draw(&mut canvas)?;

        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => {
                    game_loop = false;
                }
                _ => {}
            }
        }

        let keyboard_state = event_pump.keyboard_state();
        if keyboard_state.is_scancode_pressed(sdl2::keyboard::Scancode::Escape) {
            game_loop = false;
        }

        if keyboard_state.is_scancode_pressed(sdl2::keyboard::Scancode::Left) {
            paddle.move_left();
        }
        if keyboard_state.is_scancode_pressed(sdl2::keyboard::Scancode::Right) {
            paddle.move_right();
        }

        canvas.present();

        ::std::thread::sleep(::std::time::Duration::new(0, 1_000_000_000 / 60));
    }

    Ok(())
}
