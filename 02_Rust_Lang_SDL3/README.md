# SDL 공식 Github
- https://github.com/libsdl-org/SDL

<hr />

# SDL3(sdl3-rs)

- https://crates.io/crates/sdl3
  - https://github.com/revmischa/sdl3-rs

- SDL3 new GPU API merged (github.com/libsdl-org)
  - https://news.ycombinator.com/item?id=41396260
  - Cross-platform, graphics API agnostic, "Bring Your Own Engine/Framework" style rendering library.
    - https://github.com/bkaradzic/bgfx

- SDL3 wiki
  - https://github.com/libsdl-org/sdlwiki

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


# sdl3-sys: Low level Rust bindings for SDL 3
- https://github.com/maia-s/sdl3-sys-rs

- This version of `sdl3-sys` has bindings for SDL version `3.1.6-preview` and earlier.

- SDL 3 is ABI stable as of the 3.1.3 preview release, but `sdl3-sys` is new and may have bugs. Please submit an issue at github if you have any issues or comments!

- https://docs.rs/sdl3-sys/latest/sdl3_sys/

- Known issues(241223):

> [!CAUTION]
> - Satellite libraries (`mixer`, `image`, `ttf`) aren’t available yet
> - There are no tests yet, except for static asserts translated from the original headers
> - Some less common targets are missing detection or features to enable corresponding SDL features


# SDL3 new GPU API merged (Hacker News)
- [SDL3 is still in preview, but the new GPU API is now merged into the main branch while SDL3 maintainers apply some final tweaks(240930)](https://news.ycombinator.com/item?id=41396260)
  - Unreal/Unity are not the only solutions. There is also bgfx (https://github.com/bkaradzic/bgfx), which is quite popular and sokol gfx (https://github.com/floooh/sokol) which I know of. Of course there are many more lesser known ones.

# libsdl-org news(the latest news)
- https://news.ycombinator.com/from?site=github.com/libsdl-org

# Simple Directmedia Layer
- Simple Directmedia Layer (SDL) is a framework for creating cross-platform games and applications.
  - https://github.com/libsdl-org
