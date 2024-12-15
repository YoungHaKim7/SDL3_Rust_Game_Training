extern crate sdl2;

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video();
    match video_subsystem {
        Ok(video_subsystem) => {
            println!("SDL video subsystem initialized successfully: ");
            video_subsystem
                .window("rust-sdl2 demo: Video", 800, 600)
                .position_centered()
                .opengl()
                .build()
                .map_err(|e| e.to_string())?;
        }
        Err(e) => {
            eprintln!("Failed to initialize SDL video subsystem: {}", e);
        }
    }

    Ok(())
}
