extern crate sdl2;

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    match sdl_context.video() {
        Ok(_video_subsystem) => {
            println!("SDL video subsystem initialized successfully: ");
        }
        Err(e) => {
            eprintln!("Failed to initialize SDL video subsystem: {}", e);
        }
    }
    Ok(())
}
