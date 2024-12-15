# Result

```bash
$ cargo r --release

error[E0277]: the trait bound `&sdl3::video::Window: Into<SurfaceTarget<'_>>` is not satisfied
   --> src/main.rs:29:52
    |
29  |     let surface = unsafe { instance.create_surface(&window) };
    |                                     -------------- ^^^^^^^ the trait `HasWindowHandle` is not implemented for `sdl3::video::Window`, which is required by `&sdl3::video::Window: Into<SurfaceTarget<'_>>`
    |                                     |
    |                                     required by a bound introduced by this call
    |
    = help: the following other types implement trait `HasWindowHandle`:
              &H
              &mut H
              Arc<H>
              Box<H>
              Rc<H>
              wgpu::raw_window_handle::WindowHandle<'_>
    = note: required for `&sdl3::video::Window` to implement `HasWindowHandle`
    = note: required for `&sdl3::video::Window` to implement `wgpu::WindowHandle`
    = note: required for `SurfaceTarget<'_>` to implement `From<&sdl3::video::Window>`
    = note: required for `&sdl3::video::Window` to implement `Into<SurfaceTarget<'_>>`
note: required by a bound in `wgpu::Instance::create_surface`
   --> /Users/gy-gyoung/.cargo/registry/src/index.crates.io-6f17d22bba15001f/wgpu-23.0.1/src/api/instance.rs:278:22
    |
276 |     pub fn create_surface<'window>(
    |            -------------- required by a bound in this associated function
277 |         &self,
278 |         target: impl Into<SurfaceTarget<'window>>,
    |                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `Instance::create_surface`

error[E0277]: the trait bound `&sdl3::video::Window: Into<SurfaceTarget<'_>>` is not satisfied
   --> src/main.rs:29:52
    |
29  |     let surface = unsafe { instance.create_surface(&window) };
    |                                     -------------- ^^^^^^^ the trait `HasDisplayHandle` is not implemented for `sdl3::video::Window`, which is required by `&sdl3::video::Window: Into<SurfaceTarget<'_>>`
    |                                     |
    |                                     required by a bound introduced by this call
    |
    = help: the following other types implement trait `HasDisplayHandle`:
              &H
              &mut H
              Arc<H>
              Box<H>
              DisplayHandle<'a>
              Rc<H>
    = note: required for `&sdl3::video::Window` to implement `HasDisplayHandle`
    = note: required for `&sdl3::video::Window` to implement `wgpu::WindowHandle`
    = note: required for `SurfaceTarget<'_>` to implement `From<&sdl3::video::Window>`
    = note: required for `&sdl3::video::Window` to implement `Into<SurfaceTarget<'_>>`
note: required by a bound in `wgpu::Instance::create_surface`
   --> /Users/gy-gyoung/.cargo/registry/src/index.crates.io-6f17d22bba15001f/wgpu-23.0.1/src/api/instance.rs:278:22
    |
276 |     pub fn create_surface<'window>(
    |            -------------- required by a bound in this associated function
277 |         &self,
278 |         target: impl Into<SurfaceTarget<'window>>,
    |                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `Instance::create_surface`

error[E0277]: `Rc<WindowContext>` cannot be shared between threads safely
   --> src/main.rs:29:52
    |
29  |     let surface = unsafe { instance.create_surface(&window) };
    |                                     -------------- ^^^^^^^ `Rc<WindowContext>` cannot be shared between threads safely
    |                                     |
    |                                     required by a bound introduced by this call
    |
    = help: within `sdl3::video::Window`, the trait `Sync` is not implemented for `Rc<WindowContext>`, which is required by `&sdl3::video::Window: Into<SurfaceTarget<'_>>`
note: required because it appears within the type `sdl3::video::Window`
   --> /Users/gy-gyoung/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl3-0.11.10/src/sdl3/video.rs:673:12
    |
673 | pub struct Window {
    |            ^^^^^^
    = note: required for `&sdl3::video::Window` to implement `Send`
    = note: required for `&sdl3::video::Window` to implement `WasmNotSend`
    = note: required for `&sdl3::video::Window` to implement `WasmNotSendSync`
    = note: required for `&sdl3::video::Window` to implement `wgpu::WindowHandle`
    = note: required for `SurfaceTarget<'_>` to implement `From<&sdl3::video::Window>`
    = note: required for `&sdl3::video::Window` to implement `Into<SurfaceTarget<'_>>`
note: required by a bound in `wgpu::Instance::create_surface`
   --> /Users/gy-gyoung/.cargo/registry/src/index.crates.io-6f17d22bba15001f/wgpu-23.0.1/src/api/instance.rs:278:22
    |
276 |     pub fn create_surface<'window>(
    |            -------------- required by a bound in this associated function
277 |         &self,
278 |         target: impl Into<SurfaceTarget<'window>>,
    |                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `Instance::create_surface`

error[E0308]: mismatched types
   --> src/main.rs:33:34
    |
33  |         compatible_surface: Some(&surface),
    |                             ---- ^^^^^^^^ expected `&Surface<'_>`, found `&Result<..., ...>`
    |                             |
    |                             arguments to this enum variant are incorrect
    |
    = note: expected reference `&wgpu::Surface<'_>`
               found reference `&std::result::Result<wgpu::Surface<'_>, wgpu::CreateSurfaceError>`
help: the type constructed contains `&std::result::Result<wgpu::Surface<'_>, wgpu::CreateSurfaceError>` due to the type of the argument passed
   --> src/main.rs:33:29
    |
33  |         compatible_surface: Some(&surface),
    |                             ^^^^^--------^
    |                                  |
    |                                  this argument influences the type of `Some`
note: tuple variant defined here
   --> /Users/gy-gyoung/.rustup/toolchains/stable-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/option.rs:579:5
    |
579 |     Some(#[stable(feature = "rust1", since = "1.0.0")] T),
    |     ^^^^

error[E0560]: struct `wgpu_types::DeviceDescriptor<Option<&str>>` has no field named `limits`
  --> src/main.rs:42:13
   |
42 |             limits: wgpu::Limits::default(),
   |             ^^^^^^ `wgpu_types::DeviceDescriptor<Option<&str>>` does not have this field
   |
   = note: available fields are: `required_features`, `required_limits`, `memory_hints`

error[E0560]: struct `wgpu_types::DeviceDescriptor<Option<&str>>` has no field named `features`
  --> src/main.rs:44:13
   |
44 |             features: wgpu::Features::empty(),
   |             ^^^^^^^^ `wgpu_types::DeviceDescriptor<Option<&str>>` does not have this field
   |
   = note: available fields are: `required_features`, `required_limits`, `memory_hints`

error[E0063]: missing field `compilation_options` in initializer of `wgpu::VertexState<'_>`
  --> src/main.rs:74:17
   |
74 |         vertex: wgpu::VertexState {
   |                 ^^^^^^^^^^^^^^^^^ missing `compilation_options`

error[E0063]: missing field `compilation_options` in initializer of `wgpu::FragmentState<'_>`
  --> src/main.rs:79:24
   |
79 |         fragment: Some(wgpu::FragmentState {
   |                        ^^^^^^^^^^^^^^^^^^^ missing `compilation_options`

error[E0063]: missing field `cache` in initializer of `wgpu::RenderPipelineDescriptor<'_>`
  --> src/main.rs:72:58
   |
72 | ..._pipeline(&wgpu::RenderPipelineDescriptor {
   |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ missing `cache`

error[E0599]: no method named `get_supported_formats` found for enum `std::result::Result` in the current scope
   --> src/main.rs:109:25
    |
109 |         format: surface.get_supported_formats(&adapter)[0],
    |                         ^^^^^^^^^^^^^^^^^^^^^ method not found in `Result<Surface<'_>, CreateSurfaceError>`

error[E0063]: missing fields `desired_maximum_frame_latency` and `view_formats` in initializer of `wgpu_types::SurfaceConfiguration<Vec<TextureFormat>>`
   --> src/main.rs:107:22
    |
107 |     let mut config = wgpu::SurfaceConfiguration {
    |                      ^^^^^^^^^^^^^^^^^^^^^^^^^^ missing `desired_maximum_frame_latency` and `view_formats`

error[E0599]: no method named `configure` found for enum `std::result::Result` in the current scope
   --> src/main.rs:115:13
    |
115 |     surface.configure(&device, &config);
    |             ^^^^^^^^^ method not found in `Result<Surface<'_>, CreateSurfaceError>`
    |
note: the method `configure` exists on the type `wgpu::Surface<'_>`
   --> /Users/gy-gyoung/.cargo/registry/src/index.crates.io-6f17d22bba15001f/wgpu-23.0.1/src/api/surface.rs:88:5
    |
88  |     pub fn configure(&self, device: &Device, config: &SurfaceConfiguration...
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
help: use the `?` operator to extract the `wgpu::Surface<'_>` value, propagating a `Result::Err` value to the caller
    |
115 |     surface?.configure(&device, &config);
    |            +

error[E0599]: no method named `configure` found for enum `std::result::Result` in the current scope
   --> src/main.rs:128:29
    |
128 |                     surface.configure(&device, &config);
    |                             ^^^^^^^^^ method not found in `Result<Surface<'_>, CreateSurfaceError>`
    |
note: the method `configure` exists on the type `wgpu::Surface<'_>`
   --> /Users/gy-gyoung/.cargo/registry/src/index.crates.io-6f17d22bba15001f/wgpu-23.0.1/src/api/surface.rs:88:5
    |
88  |     pub fn configure(&self, device: &Device, config: &SurfaceConfiguration...
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
help: use the `?` operator to extract the `wgpu::Surface<'_>` value, propagating a `Result::Err` value to the caller
    |
128 |                     surface?.configure(&device, &config);
    |                            +

error[E0599]: no method named `get_current_texture` found for enum `std::result::Result` in the current scope
   --> src/main.rs:143:35
    |
143 |         let frame = match surface.get_current_texture() {
    |                                   ^^^^^^^^^^^^^^^^^^^ method not found in `Result<Surface<'_>, CreateSurfaceError>`
    |
note: the method `get_current_texture` exists on the type `wgpu::Surface<'_>`
   --> /Users/gy-gyoung/.cargo/registry/src/index.crates.io-6f17d22bba15001f/wgpu-23.0.1/src/api/surface.rs:108:5
    |
108 |     pub fn get_current_texture(&self) -> Result<SurfaceTexture, SurfaceError...
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
help: use the `?` operator to extract the `wgpu::Surface<'_>` value, propagating a `Result::Err` value to the caller
    |
143 |         let frame = match surface?.get_current_texture() {
    |                                  +

error[E0308]: mismatched types
   --> src/main.rs:170:32
    |
170 |                         store: true,
    |                                ^^^^ expected `StoreOp`, found `bool`

error[E0063]: missing fields `occlusion_query_set` and `timestamp_writes` in initializer of `wgpu::RenderPassDescriptor<'_>`
   --> src/main.rs:164:56
    |
164 | ...gin_render_pass(&wgpu::RenderPassDescriptor {
    |                     ^^^^^^^^^^^^^^^^^^^^^^^^^^ missing `occlusion_query_set` and `timestamp_writes`

Some errors have detailed explanations: E0063, E0277, E0308, E0560, E0599.
For more information about an error, try `rustc --explain E0063`.
error: could not compile `raw-windows-handle-with-wgpu` (bin "raw-windows-handle-with-wgpu") due to 16 previous errors

…ng_SDL3/raw-windows-handle-with-wgpu on  main [?] is 📦 v0.1.0 via 🦀 v1.83.0
❯ 


```

