# Result

- 여기만 해결하면됨.

```bash
   |
160  |     canvas.draw_lines(unsafe { points.as_slice() });
     |            ---------- ^^^^^^^^^-----------------^^
     |            |          |        |
     |            |          |        this tail expression is of type `&[Point]`
     |            |          the trait `From<&[Point]>` is not implemented for `&[FPoint]`
     |            required by a bound introduced by this call
     |
     = help: the following other types implement trait `From<T>`:
               `[SDL_MessageBoxColor; 5]` implements `From<MessageBoxColorScheme>`
               `[T; 10]` implements `From<(T, T, T, T, T, T, T, T, T, T)>`
               `[T; 11]` implements `From<(T, T, T, T, T, T, T, T, T, T, T)>`
               `[T; 12]` implements `From<(T, T, T, T, T, T, T, T, T, T, T, T)>`
               `[T; 1]` implements `From<(T,)>`
               `[T; 2]` implements `From<(T, T)>`
               `[T; 3]` implements `From<(T, T, T)>`
               `[T; 4]` implements `From<(T, T, T, T)>`
             and 20 others
     = note: required for `&[Point]` to implement `Into<&[FPoint]>`
note: required by a bound in `Canvas::<T>::draw_lines`
    --> /Users/gy-gyoung/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl3-0.14.2/src/sdl3/render.rs:1236:30
     |
1236 |     pub fn draw_lines<'a, P: Into<&'a [FPoint]>>(&mut self, points: P) -> Result<(), Error> {
     |                              ^^^^^^^^^^^^^^^^^^ required by this bound in `Canvas::<T>::draw_lines`

```

