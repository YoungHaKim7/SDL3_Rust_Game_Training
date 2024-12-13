# SDL3_Rust_Game_Training

- SDL3-rs[![crates.io](https://img.shields.io/crates/v/sdl3.svg)](https://crates.io/crates/sdl3)![Crates.io](https://img.shields.io/crates/l/sdl3)![Downloads](https://img.shields.io/crates/d/sdl3.svg)<a href="https://github.com/libsdl-org/SDL"><img alt="githubicon" width="20px" src="https://user-images.githubusercontent.com/67513038/218287708-001511d7-1cce-42d3-92d2-4a61193b38f0.png" /></a>
![star](https://img.shields.io/github/stars/revmischa/sdl3-rs.svg)

- Bindings for SDL3 in Rust.

SDL is the Simple Directmedia Library, a cross-platform library to abstract the platform-specific details for building applications. It takes care of everything from handling events, creating windows, playing audio, accessing device cameras and sensors, locking, GPU access, and much more. See more here: https://wiki.libsdl.org/SDL3/APIByCategory.

SDL officially supports Windows, macOS, Linux, iOS, and Android, and several other platforms.    

# C언어로 만듬Simple Directmedia Layer  

- https://github.com/libsdl-org/SDL
  - https://libsdl.org/

<hr / >

- https://crates.io/crates/sdl3
  - https://github.com/revmischa/sdl3-rs



```
cargo add sdl3  
```

- toml
```bash
sdl3 = "0.11.8"
```

# Documentation

- [SDL3 higher-level documentation.](https://docs.rs/sdl3/latest/sdl3/)
- [SDL3-sys lower-level bindings documentation](https://docs.rs/sdl3-sys/latest/sdl3_sys/)


# History

- This project was forked from [Rust-sdl2](https://github.com/Rust-sdl2/rust-sdl2) and the SDL2 code migrated to SDL3 according to the [SDL2->SDL3 migration guide.](https://github.com/libsdl-org/SDL/blob/main/docs/README-migration.md)
  - If you want a library compatible with earlier versions of SDL, please see [Rust-sdl2.](https://github.com/Rust-sdl2/rust-sdl2)

# Install

```bash
--   SDL_XINPUT                  (Wanted: OFF): OFF
-- 
--  CFLAGS:         -idirafter "/home/g/utilities/SDL2-2.30.10/src/video/khronos" -DHAVE_LINUX_VERSION_H
--  EXTRA_CFLAGS:   -Wall -Wundef -fno-strict-aliasing -Wdeclaration-after-statement -fvisibility=hidden -Wshadow -Wno-unused-local-typedefs -mmmx -msse -msse2 -msse3  -isystem/usr/include -D_REENTRANT
--  EXTRA_LDFLAGS: -pthread -Wl,--no-undefined
--  EXTRA_LIBS:    m
-- 
--  Build Shared Library: ON
--  Build Static Library: ON
--  Build Static Library with Position Independent Code: OFF
-- 
-- If something was not detected, although the libraries
-- were installed, then make sure you have set the
-- CFLAGS and LDFLAGS environment variables correctly.
-- 
-- Configuring done (0.3s)
-- Generating done (0.0s)
-- Build files have been written to: /home/g/utilities/SDL2-2.30.10/build
[ 15%] Built target sdl_headers_copy
[ 16%] Built target SDL2main
[ 18%] Built target SDL2_test
[100%] Built target SDL2-static
[100%] Built target SDL2
-- Install configuration: ""
-- Installing: /usr/local/lib/libSDL2-2.0.so.0.3000.10

CMake Error at build/cmake_install.cmake:46 (file):
  file INSTALL cannot copy file
  "/home/gygy/utilities/SDL2-2.30.10/build/libSDL2-2.0.so.0.3000.10" to
  "/usr/local/lib/libSDL2-2.0.so.0.3000.10": Permission denied.

```
