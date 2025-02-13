# link 

- [Shaderc(glslc) & 컴파일 예시](#shadercglslc)
- [GLSL기능들](https://github.com/KhronosGroup/GLSL/tree/main/extensions/ext)
  - [GLSL기능들(Extension Specifications)](https://github.com/KhronosGroup/GLSL#extension-specifications-in-this-repository)
- [zig언어로 만든 GLSL_analyzer](https://github.com/nolanderc/glsl_analyzer)
- [Rust언어로 만든 GLSL_lsp](https://github.com/KubaP/glsl-lsp)

<hr />

# Shaderc(glslc)
- A collection of tools, libraries and tests for shader compilation. At the moment it includes:
  - https://github.com/google/shaderc

# 컴파일 예시

```
glslc -O triangle.vert -o triangle.vert.spv
glslc -O cube.frag -o cube.frag.spv
glslc -O cube.vert -o cube.vert.spv
glslc -O cube-texture.frag -o cube-texture.frag.spv
glslc -O cube-texture.vert -o cube-texture.vert.spv
```

- https://github.com/vhspace/sdl3-rs/tree/master/examples/shaders

<hr />

# HLSL Tutorials | Ben Cloward
- https://youtube.com/playlist?list=PL78XDi0TS4lEMvytsE_MoWEpzBcukXv9b&si=zXtEjjrJyFRVaa4F

# Shader Tutorial Series | Lewis Lepton
- https://youtube.com/playlist?list=PL4neAtv21WOmIrTrkNO3xCyrxg4LKkrF7&si=gLxRV8ViChwKcrEL


<hr />

# 쉐이더 기술 이해_Shader compiler and tools for SDLSL (Simple Directmedia Layer Shader Language)[|🔝|](#link)

- [Shader compiler and tools for SDLSL (Simple Directmedia Layer Shader Language](https://github.com/libsdl-org/SDL_shader_tools)
  - [README-shader-language-quickstart.md](https://github.com/libsdl-org/SDL_shader_tools/blob/main/docs/README-shader-language-quickstart.md)
    - [쉐이더 문법을 코드랑 잘 설명한 UE3에서 일부 가져왔다가고 함.Vertex Shader Syntax Megathread](https://github.com/libsdl-org/SDL_shader_tools/issues/3)
  - [SDL GPU SUPPORT: THE BASIC IDEA](https://github.com/libsdl-org/SDL_shader_tools/blob/main/docs/README-SDL_gpu.md)

- [Shader translation library for SDL's GPU API._https://github.com/libsdl-org/SDL_shadercross](https://github.com/libsdl-org/SDL_shadercross)

<hr />

# GLSL snippets

- https://gist.github.com/lewislepton/8b17f56baa7f1790a70284e7520f9623
