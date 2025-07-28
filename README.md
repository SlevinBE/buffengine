The goal of this project is for me to learn Rust and writing a game engine, two things that interest me.
Uses [wgpu](https://wgpu.rs) as its low-level graphics library.

While a quite simple 2D game engine, it has the following features:
- wgpu rendering pipeline with sprite shader
- 2D texture support
- coordinate systems:
  - local space (meshes, -0.5 to 0.5)
  - world space (game objects, any units or range)
  - view space (camera, same units as world space)
  - clip space (screen, -1 to 1)
- event system
- layer system
- scene and renderables