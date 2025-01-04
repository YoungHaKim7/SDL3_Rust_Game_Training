# raylib
- https://www.raylib.com/
  - API https://www.raylib.com/cheatsheet/cheatsheet.html
  - raylib https://github.com/raysan5/raylib

- https://github.com/raysan5/raylib/wiki/Working-on-GNU-Linux#build-raylib-using-cmake

```
git clone https://github.com/raysan5/raylib.git raylib
cd raylib
mkdir build && cd build
cmake -DBUILD_SHARED_LIBS=ON ..
make
sudo make install
sudo ldconfig
```

```
Install the project...
-- Install configuration: "Debug"
-- Installing: /usr/local/lib/libraylib.so.5.5.0
-- Installing: /usr/local/lib/libraylib.so.550
-- Installing: /usr/local/lib/libraylib.so
-- Installing: /usr/local/include/raylib.h
-- Installing: /usr/local/include/rcamera.h
-- Installing: /usr/local/include/rlgl.h
-- Installing: /usr/local/include/raymath.h
-- Installing: /usr/local/lib/pkgconfig/raylib.pc
-- Installing: /usr/local/lib/cmake/raylib/raylib-config-version.cmake
-- Installing: /usr/local/lib/cmake/raylib/raylib-config.cmake

```

- https://github.com/raysan5/raylib-intro-course
- 파이썬 코드 맘에 안들지만 아쉬운데로
  - https://github.com/clear-code-projects/raylib_intro


<hr />

# DOOM Open Source Release 
- https://github.com/id-Software/DOOM
- https://github.com/id-Software/DOOM-3
  - doom3 pk4파일 https://www.reddit.com/r/Roms/comments/mrsb9j/doom_3_pk4_files/

<hr />

# 러스트로 만들고 있음 미친..벌써rust edition 2024적용함(rust 1.85)
- https://gitlab.com/flukejones/room4doom


- Doom Install필수
  - https://gitlab.com/flukejones/room4doom 

```
sudo apt install libsdl2-mixer-dev libsdl2-dev
```

<hr />

# C++로 Doom만들기
- https://github.com/amroibrahim/DIYDoom

<hr />

# Rust로 테트리스 만들기
- https://github.com/bigOconstant/RustTetris
- https://github.com/flippingbitss/rust-tetris
  - https://wikidocs.net/194500

- sdl2는 아니지만
  - https://github.com/AndrewJakubowicz/ggezFlappyCrabby.git

<hr />

# etc그외 프로젝트들

- https://www.lohninger.com/examples_part1.html


# C언어
- https://blog.conan.io/2023/07/20/introduction-to-game-dev-with-sdl2.html

- https://github.com/conan-io/examples2

# C++
- https://thenumb.at/cpp-course/

- snake게임 만들기
  - https://codereview.stackexchange.com/questions/212296/snake-game-in-c-with-sdl
