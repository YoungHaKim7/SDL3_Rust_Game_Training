# Result

```bash

$ cargo r

error[E0432]: unresolved import `get_error`
 --> /Users/gy-gyoung/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl3-0.11.10/src/sdl3/gfx/framerate.rs:3:5
  |
3 | use get_error;
  |     ^^^^^^^^^ no external crate `get_error`

error[E0432]: unresolved import `sys::gfx`
 --> /Users/gy-gyoung/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl3-0.11.10/src/sdl3/gfx/framerate.rs:7:5
  |
7 | use sys::gfx;
  |     ^^^^^^^^ no `gfx` in the root
  |
help: consider importing this module instead
  |
7 | use crate::gfx;
  |     ~~~~~~~~~~

error[E0432]: unresolved import `get_error`
 --> /Users/gy-gyoung/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl3-0.11.10/src/sdl3/gfx/imagefilter.rs:4:5
  |
4 | use get_error;
  |     ^^^^^^^^^ no external crate `get_error`

error[E0432]: unresolved import `sys::gfx`
 --> /Users/gy-gyoung/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl3-0.11.10/src/sdl3/gfx/imagefilter.rs:7:10
  |
7 | use sys::gfx::imagefilter;
  |          ^^^ could not find `gfx` in `sys`

error[E0432]: unresolved import `get_error`
 --> /Users/gy-gyoung/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl3-0.11.10/src/sdl3/gfx/rotozoom.rs:3:5
  |
3 | use get_error;
  |     ^^^^^^^^^ no external crate `get_error`

error[E0432]: unresolved import `surface`
 --> /Users/gy-gyoung/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl3-0.11.10/src/sdl3/gfx/rotozoom.rs:6:5
  |
6 | use surface::Surface;
  |     ^^^^^^^ help: a similar path exists: `crate::surface`
  |
  = note: `use` statements changed in Rust 2018; read more at <https://doc.rust-lang.org/edition-guide/rust-2018/module-system/path-clarity.html>

error[E0432]: unresolved import `sys::gfx`
 --> /Users/gy-gyoung/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl3-0.11.10/src/sdl3/gfx/rotozoom.rs:7:10
  |
7 | use sys::gfx::rotozoom;
  |          ^^^ could not find `gfx` in `sys`

error[E0433]: failed to resolve: could not find `render` in the list of imported crates
   --> /Users/gy-gyoung/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl3-0.11.10/src/sdl3/gfx/primitives.rs:214:10
    |
214 |     T: ::render::RenderTarget,
    |          ^^^^^^ could not find `render` in the list of imported crates
    |
help: consider importing this module
    |
3   + use crate::render;
    |
help: if you import `render`, refer to it directly
    |
214 -     T: ::render::RenderTarget,
214 +     T: render::RenderTarget,
    |

error[E0425]: cannot find function `get_error` in this scope
   --> /Users/gy-gyoung/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl3-0.11.10/src/sdl3/gfx/primitives.rs:221:17
    |
221 |             Err(get_error())
    |                 ^^^^^^^^^ not found in this scope
    |
help: consider importing this function through its public re-export
    |
3   + use crate::get_error;
    |

error[E0425]: cannot find function `get_error` in this scope
   --> /Users/gy-gyoung/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl3-0.11.10/src/sdl3/gfx/primitives.rs:229:17
    |
229 |             Err(get_error())
    |                 ^^^^^^^^^ not found in this scope
    |
help: consider importing this function through its public re-export
    |
3   + use crate::get_error;
    |

error[E0425]: cannot find function `get_error` in this scope
   --> /Users/gy-gyoung/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl3-0.11.10/src/sdl3/gfx/primitives.rs:237:17
    |
237 |             Err(get_error())
    |                 ^^^^^^^^^ not found in this scope
    |
help: consider importing this function through its public re-export
    |
3   + use crate::get_error;
    |

error[E0425]: cannot find function `get_error` in this scope
   --> /Users/gy-gyoung/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl3-0.11.10/src/sdl3/gfx/primitives.rs:252:17
    |
252 |             Err(get_error())
    |                 ^^^^^^^^^ not found in this scope
    |
help: consider importing this function through its public re-export
    |
3   + use crate::get_error;
    |

error[E0425]: cannot find function `get_error` in this scope
   --> /Users/gy-gyoung/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl3-0.11.10/src/sdl3/gfx/primitives.rs:270:17
    |
270 |             Err(get_error())
    |                 ^^^^^^^^^ not found in this scope
    |
help: consider importing this function through its public re-export
    |
3   + use crate::get_error;
    |

error[E0425]: cannot find function `get_error` in this scope
   --> /Users/gy-gyoung/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl3-0.11.10/src/sdl3/gfx/primitives.rs:278:17
    |
278 |             Err(get_error())
    |                 ^^^^^^^^^ not found in this scope
    |
help: consider importing this function through its public re-export
    |
3   + use crate::get_error;
    |

error[E0425]: cannot find function `get_error` in this scope
   --> /Users/gy-gyoung/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl3-0.11.10/src/sdl3/gfx/primitives.rs:295:17
    |
295 |             Err(get_error())
    |                 ^^^^^^^^^ not found in this scope
    |
help: consider importing this function through its public re-export
    |
3   + use crate::get_error;
    |

error[E0425]: cannot find function `get_error` in this scope
   --> /Users/gy-gyoung/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl3-0.11.10/src/sdl3/gfx/primitives.rs:303:17
    |
303 |             Err(get_error())
    |                 ^^^^^^^^^ not found in this scope
    |
help: consider importing this function through its public re-export
    |
3   + use crate::get_error;
    |

error[E0425]: cannot find function `get_error` in this scope
   --> /Users/gy-gyoung/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl3-0.11.10/src/sdl3/gfx/primitives.rs:318:17
    |
318 |             Err(get_error())
    |                 ^^^^^^^^^ not found in this scope
    |
help: consider importing this function through its public re-export
    |
3   + use crate::get_error;
    |

error[E0425]: cannot find function `get_error` in this scope
   --> /Users/gy-gyoung/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl3-0.11.10/src/sdl3/gfx/primitives.rs:336:17
    |
336 |             Err(get_error())
    |                 ^^^^^^^^^ not found in this scope
    |
help: consider importing this function through its public re-export
    |
3   + use crate::get_error;
    |

error[E0425]: cannot find function `get_error` in this scope
   --> /Users/gy-gyoung/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl3-0.11.10/src/sdl3/gfx/primitives.rs:344:17
    |
344 |             Err(get_error())
    |                 ^^^^^^^^^ not found in this scope
    |
help: consider importing this function through its public re-export
    |
3   + use crate::get_error;
    |

error[E0425]: cannot find function `get_error` in this scope
   --> /Users/gy-gyoung/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl3-0.11.10/src/sdl3/gfx/primitives.rs:352:17
    |
352 |             Err(get_error())
    |                 ^^^^^^^^^ not found in this scope
    |
help: consider importing this function through its public re-export
    |
3   + use crate::get_error;
    |

error[E0425]: cannot find function `get_error` in this scope
   --> /Users/gy-gyoung/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl3-0.11.10/src/sdl3/gfx/primitives.rs:360:17
    |
360 |             Err(get_error())
    |                 ^^^^^^^^^ not found in this scope
    |
help: consider importing this function through its public re-export
    |
3   + use crate::get_error;
    |

error[E0425]: cannot find function `get_error` in this scope
   --> /Users/gy-gyoung/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl3-0.11.10/src/sdl3/gfx/primitives.rs:377:17
    |
377 |             Err(get_error())
    |                 ^^^^^^^^^ not found in this scope
    |
help: consider importing this function through its public re-export
    |
3   + use crate::get_error;
    |

error[E0425]: cannot find function `get_error` in this scope
   --> /Users/gy-gyoung/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl3-0.11.10/src/sdl3/gfx/primitives.rs:392:17
    |
392 |             Err(get_error())
    |                 ^^^^^^^^^ not found in this scope
    |
help: consider importing this function through its public re-export
    |
3   + use crate::get_error;
    |

error[E0425]: cannot find function `get_error` in this scope
   --> /Users/gy-gyoung/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl3-0.11.10/src/sdl3/gfx/primitives.rs:407:17
    |
407 |             Err(get_error())
    |                 ^^^^^^^^^ not found in this scope
    |
help: consider importing this function through its public re-export
    |
3   + use crate::get_error;
    |

error[E0425]: cannot find function `get_error` in this scope
   --> /Users/gy-gyoung/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl3-0.11.10/src/sdl3/gfx/primitives.rs:423:17
    |
423 |             Err(get_error())
    |                 ^^^^^^^^^ not found in this scope
    |
help: consider importing this function through its public re-export
    |
3   + use crate::get_error;
    |

error[E0425]: cannot find function `get_error` in this scope
   --> /Users/gy-gyoung/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl3-0.11.10/src/sdl3/gfx/primitives.rs:440:17
    |
440 |             Err(get_error())
    |                 ^^^^^^^^^ not found in this scope
    |
help: consider importing this function through its public re-export
    |
3   + use crate::get_error;
    |

error[E0425]: cannot find function `get_error` in this scope
   --> /Users/gy-gyoung/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl3-0.11.10/src/sdl3/gfx/primitives.rs:458:17
    |
458 |             Err(get_error())
    |                 ^^^^^^^^^ not found in this scope
    |
help: consider importing this function through its public re-export
    |
3   + use crate::get_error;
    |

error[E0425]: cannot find function `get_error` in this scope
   --> /Users/gy-gyoung/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl3-0.11.10/src/sdl3/gfx/primitives.rs:476:17
    |
476 |             Err(get_error())
    |                 ^^^^^^^^^ not found in this scope
    |
help: consider importing this function through its public re-export
    |
3   + use crate::get_error;
    |

error[E0425]: cannot find function `get_error` in this scope
   --> /Users/gy-gyoung/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl3-0.11.10/src/sdl3/gfx/primitives.rs:495:17
    |
495 |             Err(get_error())
    |                 ^^^^^^^^^ not found in this scope
    |
help: consider importing this function through its public re-export
    |
3   + use crate::get_error;
    |

error[E0425]: cannot find function `get_error` in this scope
   --> /Users/gy-gyoung/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl3-0.11.10/src/sdl3/gfx/primitives.rs:514:17
    |
514 |             Err(get_error())
    |                 ^^^^^^^^^ not found in this scope
    |
help: consider importing this function through its public re-export
    |
3   + use crate::get_error;
    |

error[E0425]: cannot find function `get_error` in this scope
   --> /Users/gy-gyoung/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl3-0.11.10/src/sdl3/gfx/primitives.rs:527:17
    |
527 |             Err(get_error())
    |                 ^^^^^^^^^ not found in this scope
    |
help: consider importing this function through its public re-export
    |
3   + use crate::get_error;
    |

error[E0425]: cannot find function `get_error` in this scope
   --> /Users/gy-gyoung/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl3-0.11.10/src/sdl3/gfx/primitives.rs:540:17
    |
540 |             Err(get_error())
    |                 ^^^^^^^^^ not found in this scope
    |
help: consider importing this function through its public re-export
    |
3   + use crate::get_error;
    |

error[E0425]: cannot find function `get_error` in this scope
   --> /Users/gy-gyoung/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl3-0.11.10/src/sdl3/gfx/primitives.rs:553:17
    |
553 |             Err(get_error())
    |                 ^^^^^^^^^ not found in this scope
    |
help: consider importing this function through its public re-export
    |
3   + use crate::get_error;
    |

error[E0425]: cannot find function `get_error` in this scope
   --> /Users/gy-gyoung/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl3-0.11.10/src/sdl3/gfx/primitives.rs:585:17
    |
585 |             Err(get_error())
    |                 ^^^^^^^^^ not found in this scope
    |
help: consider importing this function through its public re-export
    |
3   + use crate::get_error;
    |

error[E0425]: cannot find function `get_error` in this scope
   --> /Users/gy-gyoung/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl3-0.11.10/src/sdl3/gfx/primitives.rs:595:17
    |
595 |             Err(get_error())
    |                 ^^^^^^^^^ not found in this scope
    |
help: consider importing this function through its public re-export
    |
3   + use crate::get_error;
    |

error[E0425]: cannot find function `get_error` in this scope
   --> /Users/gy-gyoung/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl3-0.11.10/src/sdl3/gfx/primitives.rs:608:17
    |
608 |             Err(get_error())
    |                 ^^^^^^^^^ not found in this scope
    |
help: consider importing this function through its public re-export
    |
3   + use crate::get_error;
    |

error[E0433]: failed to resolve: use of undeclared crate or module `primitives`
   --> /Users/gy-gyoung/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl3-0.11.10/src/sdl3/gfx/primitives.rs:217:28
    |
217 |         let ret = unsafe { primitives::pixelColor(self.raw(), x, y, color.as_u32()) };
    |                            ^^^^^^^^^^ use of undeclared crate or module `primitives`

error[E0433]: failed to resolve: use of undeclared crate or module `primitives`
   --> /Users/gy-gyoung/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl3-0.11.10/src/sdl3/gfx/primitives.rs:225:28
    |
225 |         let ret = unsafe { primitives::hlineColor(self.raw(), x1, x2, y, color.as_u32()) };
    |                            ^^^^^^^^^^ use of undeclared crate or module `primitives`

error[E0433]: failed to resolve: use of undeclared crate or module `primitives`
   --> /Users/gy-gyoung/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl3-0.11.10/src/sdl3/gfx/primitives.rs:233:28
    |
233 |         let ret = unsafe { primitives::vlineColor(self.raw(), x, y1, y2, color.as_u32()) };
    |                            ^^^^^^^^^^ use of undeclared crate or module `primitives`

error[E0433]: failed to resolve: use of undeclared crate or module `primitives`
   --> /Users/gy-gyoung/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl3-0.11.10/src/sdl3/gfx/primitives.rs:248:28
    |
248 |         let ret = unsafe { primitives::rectangleColor(self.raw(), x1, y1, x2, y2, color.as_u32()) };
    |                            ^^^^^^^^^^ use of undeclared crate or module `primitives`

error[E0433]: failed to resolve: use of undeclared crate or module `primitives`
   --> /Users/gy-gyoung/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl3-0.11.10/src/sdl3/gfx/primitives.rs:265:13
    |
265 |             primitives::roundedRectangleColor(self.raw(), x1, y1, x2, y2, rad, color.as_u32())
    |             ^^^^^^^^^^ use of undeclared crate or module `primitives`

error[E0433]: failed to resolve: use of undeclared crate or module `primitives`
   --> /Users/gy-gyoung/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl3-0.11.10/src/sdl3/gfx/primitives.rs:274:28
    |
274 |         let ret = unsafe { primitives::boxColor(self.raw(), x1, y1, x2, y2, color.as_u32()) };
    |                            ^^^^^^^^^^ use of undeclared crate or module `primitives`

error[E0433]: failed to resolve: use of undeclared crate or module `primitives`
   --> /Users/gy-gyoung/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl3-0.11.10/src/sdl3/gfx/primitives.rs:291:22
    |
291 |             unsafe { primitives::roundedBoxColor(self.raw(), x1, y1, x2, y2, rad, color.as_u32()) };
    |                      ^^^^^^^^^^ use of undeclared crate or module `primitives`

error[E0433]: failed to resolve: use of undeclared crate or module `primitives`
   --> /Users/gy-gyoung/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl3-0.11.10/src/sdl3/gfx/primitives.rs:299:28
    |
299 |         let ret = unsafe { primitives::lineColor(self.raw(), x1, y1, x2, y2, color.as_u32()) };
    |                            ^^^^^^^^^^ use of undeclared crate or module `primitives`

error[E0433]: failed to resolve: use of undeclared crate or module `primitives`
   --> /Users/gy-gyoung/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl3-0.11.10/src/sdl3/gfx/primitives.rs:314:28
    |
314 |         let ret = unsafe { primitives::aalineColor(self.raw(), x1, y1, x2, y2, color.as_u32()) };
    |                            ^^^^^^^^^^ use of undeclared crate or module `primitives`

error[E0433]: failed to resolve: use of undeclared crate or module `primitives`
   --> /Users/gy-gyoung/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl3-0.11.10/src/sdl3/gfx/primitives.rs:331:13
    |
331 |             primitives::thickLineColor(self.raw(), x1, y1, x2, y2, width, color.as_u32())
    |             ^^^^^^^^^^ use of undeclared crate or module `primitives`

error[E0433]: failed to resolve: use of undeclared crate or module `primitives`
   --> /Users/gy-gyoung/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl3-0.11.10/src/sdl3/gfx/primitives.rs:340:28
    |
340 |         let ret = unsafe { primitives::circleColor(self.raw(), x, y, rad, color.as_u32()) };
    |                            ^^^^^^^^^^ use of undeclared crate or module `primitives`

error[E0433]: failed to resolve: use of undeclared crate or module `primitives`
   --> /Users/gy-gyoung/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl3-0.11.10/src/sdl3/gfx/primitives.rs:348:28
    |
348 |         let ret = unsafe { primitives::aacircleColor(self.raw(), x, y, rad, color.as_u32()) };
    |                            ^^^^^^^^^^ use of undeclared crate or module `primitives`

error[E0433]: failed to resolve: use of undeclared crate or module `primitives`
   --> /Users/gy-gyoung/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl3-0.11.10/src/sdl3/gfx/primitives.rs:356:28
    |
356 |         let ret = unsafe { primitives::filledCircleColor(self.raw(), x, y, rad, color.as_u32()) };
    |                            ^^^^^^^^^^ use of undeclared crate or module `primitives`

error[E0433]: failed to resolve: use of undeclared crate or module `primitives`
   --> /Users/gy-gyoung/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl3-0.11.10/src/sdl3/gfx/primitives.rs:373:22
    |
373 |             unsafe { primitives::arcColor(self.raw(), x, y, rad, start, end, color.as_u32()) };
    |                      ^^^^^^^^^^ use of undeclared crate or module `primitives`

error[E0433]: failed to resolve: use of undeclared crate or module `primitives`
   --> /Users/gy-gyoung/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl3-0.11.10/src/sdl3/gfx/primitives.rs:388:28
    |
388 |         let ret = unsafe { primitives::ellipseColor(self.raw(), x, y, rx, ry, color.as_u32()) };
    |                            ^^^^^^^^^^ use of undeclared crate or module `primitives`

error[E0433]: failed to resolve: use of undeclared crate or module `primitives`
   --> /Users/gy-gyoung/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl3-0.11.10/src/sdl3/gfx/primitives.rs:403:28
    |
403 |         let ret = unsafe { primitives::aaellipseColor(self.raw(), x, y, rx, ry, color.as_u32()) };
    |                            ^^^^^^^^^^ use of undeclared crate or module `primitives`

error[E0433]: failed to resolve: use of undeclared crate or module `primitives`
   --> /Users/gy-gyoung/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl3-0.11.10/src/sdl3/gfx/primitives.rs:419:22
    |
419 |             unsafe { primitives::filledEllipseColor(self.raw(), x, y, rx, ry, color.as_u32()) };
    |                      ^^^^^^^^^^ use of undeclared crate or module `primitives`

error[E0433]: failed to resolve: use of undeclared crate or module `primitives`
   --> /Users/gy-gyoung/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl3-0.11.10/src/sdl3/gfx/primitives.rs:436:22
    |
436 |             unsafe { primitives::pieColor(self.raw(), x, y, rad, start, end, color.as_u32()) };
    |                      ^^^^^^^^^^ use of undeclared crate or module `primitives`

error[E0433]: failed to resolve: use of undeclared crate or module `primitives`
   --> /Users/gy-gyoung/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl3-0.11.10/src/sdl3/gfx/primitives.rs:453:13
    |
453 |             primitives::filledPieColor(self.raw(), x, y, rad, start, end, color.as_u32())
    |             ^^^^^^^^^^ use of undeclared crate or module `primitives`

error[E0433]: failed to resolve: use of undeclared crate or module `primitives`
   --> /Users/gy-gyoung/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl3-0.11.10/src/sdl3/gfx/primitives.rs:472:22
    |
472 |             unsafe { primitives::trigonColor(self.raw(), x1, y1, x2, y2, x3, y3, color.as_u32()) };
    |                      ^^^^^^^^^^ use of undeclared crate or module `primitives`

error[E0433]: failed to resolve: use of undeclared crate or module `primitives`
   --> /Users/gy-gyoung/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl3-0.11.10/src/sdl3/gfx/primitives.rs:490:13
    |
490 |             primitives::aatrigonColor(self.raw(), x1, y1, x2, y2, x3, y3, color.as_u32())
    |             ^^^^^^^^^^ use of undeclared crate or module `primitives`

error[E0433]: failed to resolve: use of undeclared crate or module `primitives`
   --> /Users/gy-gyoung/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl3-0.11.10/src/sdl3/gfx/primitives.rs:509:13
    |
509 |             primitives::filledTrigonColor(self.raw(), x1, y1, x2, y2, x3, y3, color.as_u32())
    |             ^^^^^^^^^^ use of undeclared crate or module `primitives`

error[E0433]: failed to resolve: use of undeclared crate or module `primitives`
   --> /Users/gy-gyoung/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl3-0.11.10/src/sdl3/gfx/primitives.rs:522:13
    |
522 |             primitives::polygonColor(self.raw(), vx.as_ptr(), vy.as_ptr(), n, color.as_u32())
    |             ^^^^^^^^^^ use of undeclared crate or module `primitives`

error[E0433]: failed to resolve: use of undeclared crate or module `primitives`
   --> /Users/gy-gyoung/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl3-0.11.10/src/sdl3/gfx/primitives.rs:535:13
    |
535 |             primitives::aapolygonColor(self.raw(), vx.as_ptr(), vy.as_ptr(), n, color.as_u32())
    |             ^^^^^^^^^^ use of undeclared crate or module `primitives`

error[E0433]: failed to resolve: use of undeclared crate or module `primitives`
   --> /Users/gy-gyoung/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl3-0.11.10/src/sdl3/gfx/primitives.rs:548:13
    |
548 |             primitives::filledPolygonColor(self.raw(), vx.as_ptr(), vy.as_ptr(), n, color.as_u32())
    |             ^^^^^^^^^^ use of undeclared crate or module `primitives`

error[E0433]: failed to resolve: use of undeclared crate or module `primitives`
   --> /Users/gy-gyoung/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl3-0.11.10/src/sdl3/gfx/primitives.rs:573:13
    |
573 |             primitives::bezierColor(
    |             ^^^^^^^^^^ use of undeclared crate or module `primitives`

error[E0433]: failed to resolve: use of undeclared crate or module `primitives`
   --> /Users/gy-gyoung/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl3-0.11.10/src/sdl3/gfx/primitives.rs:591:22
    |
591 |             unsafe { primitives::characterColor(self.raw(), x, y, c as c_char, color.as_u32()) };
    |                      ^^^^^^^^^^ use of undeclared crate or module `primitives`

error[E0433]: failed to resolve: use of undeclared crate or module `primitives`
   --> /Users/gy-gyoung/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl3-0.11.10/src/sdl3/gfx/primitives.rs:603:13
    |
603 |             primitives::stringColor(self.raw(), x, y, buf as *mut c_char, color.as_u32())
    |             ^^^^^^^^^^ use of undeclared crate or module `primitives`

error[E0433]: failed to resolve: use of undeclared crate or module `primitives`
   --> /Users/gy-gyoung/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl3-0.11.10/src/sdl3/gfx/primitives.rs:622:14
    |
622 |     unsafe { primitives::gfxPrimitivesSetFont(actual_fontdata as *const c_void, cw, ch) }
    |              ^^^^^^^^^^ use of undeclared crate or module `primitives`

error[E0433]: failed to resolve: use of undeclared crate or module `primitives`
   --> /Users/gy-gyoung/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl3-0.11.10/src/sdl3/gfx/primitives.rs:627:14
    |
627 |     unsafe { primitives::gfxPrimitivesSetFontRotation(rotation as u32) }
    |              ^^^^^^^^^^ use of undeclared crate or module `primitives`

Some errors have detailed explanations: E0425, E0432, E0433.
For more information about an error, try `rustc --explain E0425`.
error: could not compile `sdl3` (lib) due to 66 previous errors

```

