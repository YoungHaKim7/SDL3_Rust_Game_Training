# SDL3(sdl3-rs)

- https://crates.io/crates/sdl3
  - https://github.com/revmischa/sdl3-rs

- SDL3 new GPU API merged (github.com/libsdl-org)
  - https://news.ycombinator.com/item?id=41396260
  - Cross-platform, graphics API agnostic, "Bring Your Own Engine/Framework" style rendering library.
    - https://github.com/bkaradzic/bgfx

<hr />

# `Cargo.toml`

```toml

[dependencies]
sdl3 = { version = "0", features = [] }

[dependencies.sdl3-sys]
version = "0.1.3+SDL3-preview-3.1.6"
```

# `create_texture_streaming( unsafe { PixelFormat::from_ll(SDL_PixelFormat::RGB24) }` 패턴

```rs
let mut texture = texture_creator
    .create_texture_streaming(
        unsafe { PixelFormat::from_ll(SDL_PixelFormat::RGB24) },
        256,
        256,
    )
    .map_err(|e| e.to_string())?;
// Create a red-green gradient
texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {
    for y in 0..256 {
        for x in 0..256 {
            let offset = y * pitch + x * 3;
            buffer[offset] = x as u8;
            buffer[offset + 1] = y as u8;
            buffer[offset + 2] = 0;
        }
    }
})?;


```
