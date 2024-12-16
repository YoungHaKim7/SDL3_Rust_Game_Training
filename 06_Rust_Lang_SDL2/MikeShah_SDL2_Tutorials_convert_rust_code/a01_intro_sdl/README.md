# Result

```bash
$ MIRIFLAGS=-Zmiri-backtrace=full cargo miri run --bin a05_opengl_init


error: unsupported operation: can't call foreign function `SDL_Init` on OS `linux`
  --> /home/gygy/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl2-0.37.0/src/sdl2/sdl.rs:90:26
   |
90 |                 result = sys::SDL_Init(0);
   |                          ^^^^^^^^^^^^^^^^ can't call foreign function `SDL_Init` on OS `linux`
   |
   = help: if this is a basic API commonly used on this target, please report an issue with Miri
   = help: however, note that Miri does not aim to support every FFI function out there; for instance, we will not support APIs for things such as GUIs, scripting languages, or databases
   = note: BACKTRACE:
   = note: inside `sdl2::Sdl::new` at /home/gygy/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl2-0.37.0/src/sdl2/sdl.rs:90:26: 90:42
   = note: inside `sdl2::init` at /home/gygy/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl2-0.37.0/src/sdl2/sdl.rs:375:5: 375:15
note: inside `main`
  --> src/bin/a05_opengl_init.rs:10:15
   |
10 |     let sdl = sdl2::init().unwrap();
   |               ^^^^^^^^^^^^
   = note: inside `<fn() -> std::result::Result<(), std::string::String> as std::ops::FnOnce<()>>::call_once - shim(fn() -> std::result::Result<(), std::string::String>)` at /home/gygy/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5: 250:71
   = note: inside `std::sys::backtrace::__rust_begin_short_backtrace::<fn() -> std::result::Result<(), std::string::String>, std::result::Result<(), std::string::String>>` at /home/gygy/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/sys/backtrace.rs:152:18: 152:21
   = note: inside closure at /home/gygy/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/rt.rs:195:18: 195:75
   = note: inside `std::ops::function::impls::<impl std::ops::FnOnce<()> for &dyn std::ops::Fn() -> i32 + std::marker::Sync + std::panic::RefUnwindSafe>::call_once` at /home/gygy/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:284:13: 284:31
   = note: inside `std::panicking::r#try::do_call::<&dyn std::ops::Fn() -> i32 + std::marker::Sync + std::panic::RefUnwindSafe, i32>` at /home/gygy/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/panicking.rs:573:40: 573:43
   = note: inside `std::panicking::r#try::<i32, &dyn std::ops::Fn() -> i32 + std::marker::Sync + std::panic::RefUnwindSafe>` at /home/gygy/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/panicking.rs:536:19: 536:88
   = note: inside `std::panic::catch_unwind::<&dyn std::ops::Fn() -> i32 + std::marker::Sync + std::panic::RefUnwindSafe, i32>` at /home/gygy/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/panic.rs:358:14: 358:33
   = note: inside closure at /home/gygy/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/rt.rs:174:48: 174:73
   = note: inside `std::panicking::r#try::do_call::<{closure@std::rt::lang_start_internal::{closure#1}}, isize>` at /home/gygy/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/panicking.rs:573:40: 573:43
   = note: inside `std::panicking::r#try::<isize, {closure@std::rt::lang_start_internal::{closure#1}}>` at /home/gygy/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/panicking.rs:536:19: 536:88
   = note: inside `std::panic::catch_unwind::<{closure@std::rt::lang_start_internal::{closure#1}}, isize>` at /home/gygy/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/panic.rs:358:14: 358:33
   = note: inside `std::rt::lang_start_internal` at /home/gygy/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/rt.rs:174:20: 174:98
   = note: inside `std::rt::lang_start::<std::result::Result<(), std::string::String>>` at /home/gygy/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/rt.rs:194:17: 199:6

error: aborting due to 1 previous error

```

