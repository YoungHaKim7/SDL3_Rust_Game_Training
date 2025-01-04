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
  - SDL2로 만듬 둠3이걸로 성공함. https://github.com/dhewm/dhewm3
  - 포크로 정리함 https://github.com/YoungHaKim7/dhewm3

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

# SDL2 성공한거 분석하기(https://github.com/dhewm/dhewm3)
- doom3 pk4파일 https://www.reddit.com/r/Roms/comments/mrsb9j/doom_3_pk4_files/
  - SDL2로 만듬 둠3이걸로 성공함. https://github.com/dhewm/dhewm3
  - 포크로 정리함 https://github.com/YoungHaKim7/dhewm3

```
cmake ../neo/
-- The C compiler identification is GNU 13.3.0
-- The CXX compiler identification is GNU 13.3.0
-- Detecting C compiler ABI info
-- Detecting C compiler ABI info - done
-- Check for working C compiler: /usr/bin/cc - skipped
-- Detecting C compile features
-- Detecting C compile features - done
-- Detecting CXX compiler ABI info
-- Detecting CXX compiler ABI info - done
-- Check for working CXX compiler: /usr/bin/c++ - skipped
-- Detecting CXX compile features
-- Detecting CXX compile features - done
-- `/usr/bin/cc -dumpmachine` says: "x86_64-linux-gnu"
--   => CPU architecture extracted from that: "x86_64"
-- Setting -DD3_ARCH="x86_64" -DD3_SIZEOFPTR=8 -DD3_OSTYPE="linux"
-- Found OpenAL: /usr/lib/x86_64-linux-gnu/libopenal.so
-- Performing Test CMAKE_HAVE_LIBC_PTHREAD
-- Performing Test CMAKE_HAVE_LIBC_PTHREAD - Success
-- Found Threads: TRUE
-- Found SDL2: /usr/lib/x86_64-linux-gnu/libSDL2main.a;/usr/lib/x86_64-linux-gnu/libSDL2.so
-- Dear ImGui integration enabled
CMake Warning at CMakeLists.txt:254 (message):
  libcurl not found, server downloads won't be available (apart from that
  dhewm3 will work)


-- Performing Test HAVE_LIBBACKTRACE
-- Performing Test HAVE_LIBBACKTRACE - Success
-- Using libbacktrace
-- Detected Little Endian architecture, setting -DD3_IS_BIG_ENDIAN=0
-- Performing Test cxx_has_fp-contract
-- Performing Test cxx_has_fp-contract - Success
-- Performing Test cxx_has_fvisibility
-- Performing Test cxx_has_fvisibility - Success
-- Performing Test cxx_has_Woverload_virtual
-- Performing Test cxx_has_Woverload_virtual - Success
-- Performing Test cxx_has_Wno-class-memaccess
-- Performing Test cxx_has_Wno-class-memaccess - Success
-- Performing Test cxx_has_Wno-cpp20-compat
-- Performing Test cxx_has_Wno-cpp20-compat - Success
-- Building RelWithDebInfo for linux-x86_64
-- The install target will use the following directories:
--   Binary directory:  /usr/local/bin
--   Library directory: /usr/local/lib/dhewm3
--   Data directory:    /usr/local/share/dhewm3
-- Configuring done (1.1s)
-- Generating done (0.0s)
-- Build files have been written to: /home/gygy/my_project/Rust_Lang/99999999/1111/2222/dhewm3/build

dhewm3/build on  master via C v13.3.0-gcc via △ v3.28.3
❯ make -j8
[  1%] Building CXX object CMakeFiles/idlib.dir/idlib/bv/Box.cpp.o
[  1%] Building CXX object CMakeFiles/idlib.dir/idlib/bv/Sphere.cpp.o
[  1%] Building CXX object CMakeFiles/idlib.dir/idlib/bv/Frustum.cpp.o
[  2%] Building CXX object CMakeFiles/idlib.dir/idlib/geometry/Winding2D.cpp.o
[  2%] Building CXX object CMakeFiles/idlib.dir/idlib/geometry/DrawVert.cpp.o
[  2%] Building CXX object CMakeFiles/idlib.dir/idlib/bv/Bounds.cpp.o
[  2%] Building CXX object CMakeFiles/idlib.dir/idlib/geometry/Surface_SweptSpline.cpp.o
[  2%] Building CXX object CMakeFiles/idlib.dir/idlib/geometry/Winding.cpp.o
[  3%] Building CXX object CMakeFiles/idlib.dir/idlib/geometry/Surface.cpp.o
[  3%] Building CXX object CMakeFiles/idlib.dir/idlib/geometry/Surface_Patch.cpp.o
[  3%] Building CXX object CMakeFiles/idlib.dir/idlib/geometry/TraceModel.cpp.o
[  3%] Building CXX object CMakeFiles/idlib.dir/idlib/geometry/JointTransform.cpp.o
[  4%] Building CXX object CMakeFiles/idlib.dir/idlib/hashing/CRC32.cpp.o
[  4%] Building CXX object CMakeFiles/idlib.dir/idlib/hashing/MD4.cpp.o
[  4%] Building CXX object CMakeFiles/idlib.dir/idlib/hashing/MD5.cpp.o
[  4%] Building CXX object CMakeFiles/idlib.dir/idlib/math/Angles.cpp.o
[  5%] Building CXX object CMakeFiles/idlib.dir/idlib/math/Lcp.cpp.o
[  5%] Building CXX object CMakeFiles/idlib.dir/idlib/math/Math.cpp.o
[  5%] Building CXX object CMakeFiles/idlib.dir/idlib/math/Matrix.cpp.o
[  6%] Building CXX object CMakeFiles/idlib.dir/idlib/math/Ode.cpp.o
[  6%] Building CXX object CMakeFiles/idlib.dir/idlib/math/Plane.cpp.o
[  6%] Building CXX object CMakeFiles/idlib.dir/idlib/math/Pluecker.cpp.o
[  6%] Building CXX object CMakeFiles/idlib.dir/idlib/math/Polynomial.cpp.o
[  7%] Building CXX object CMakeFiles/idlib.dir/idlib/math/Rotation.cpp.o
[  7%] Building CXX object CMakeFiles/idlib.dir/idlib/math/Quat.cpp.o
[  7%] Building CXX object CMakeFiles/idlib.dir/idlib/math/Simd.cpp.o
[  7%] Building CXX object CMakeFiles/idlib.dir/idlib/math/Simd_Generic.cpp.o
[  8%] Building CXX object CMakeFiles/idlib.dir/idlib/math/Simd_AltiVec.cpp.o
[  8%] Building CXX object CMakeFiles/idlib.dir/idlib/math/Simd_MMX.cpp.o
[  8%] Building CXX object CMakeFiles/idlib.dir/idlib/math/Simd_3DNow.cpp.o
[  9%] Building CXX object CMakeFiles/idlib.dir/idlib/math/Simd_SSE.cpp.o
[  9%] Building CXX object CMakeFiles/idlib.dir/idlib/math/Simd_SSE2.cpp.o
[  9%] Building CXX object CMakeFiles/idlib.dir/idlib/math/Simd_SSE3.cpp.o
[  9%] Building CXX object CMakeFiles/idlib.dir/idlib/math/Vector.cpp.o
[ 10%] Building CXX object CMakeFiles/idlib.dir/idlib/BitMsg.cpp.o
[ 10%] Building CXX object CMakeFiles/idlib.dir/idlib/LangDict.cpp.o
[ 10%] Building CXX object CMakeFiles/idlib.dir/idlib/Lexer.cpp.o
[ 10%] Building CXX object CMakeFiles/idlib.dir/idlib/Lib.cpp.o
[ 11%] Building CXX object CMakeFiles/idlib.dir/idlib/containers/HashIndex.cpp.o
[ 11%] Building CXX object CMakeFiles/idlib.dir/idlib/Dict.cpp.o
[ 11%] Building CXX object CMakeFiles/idlib.dir/idlib/Str.cpp.o
[ 12%] Building CXX object CMakeFiles/idlib.dir/idlib/Parser.cpp.o
[ 12%] Building CXX object CMakeFiles/idlib.dir/idlib/MapFile.cpp.o
[ 12%] Building CXX object CMakeFiles/idlib.dir/idlib/CmdArgs.cpp.o
[ 12%] Building CXX object CMakeFiles/idlib.dir/idlib/Token.cpp.o
[ 13%] Building CXX object CMakeFiles/idlib.dir/idlib/Base64.cpp.o
[ 13%] Building CXX object CMakeFiles/idlib.dir/idlib/Timer.cpp.o
[ 13%] Building CXX object CMakeFiles/idlib.dir/idlib/Heap.cpp.o
[ 14%] Linking CXX static library libidlib.a
[ 14%] Built target idlib
[ 14%] Building CXX object CMakeFiles/d3xp.dir/d3xp/AF.cpp.o
[ 14%] Building CXX object CMakeFiles/d3xp.dir/d3xp/AFEntity.cpp.o
[ 15%] Building CXX object CMakeFiles/d3xp.dir/d3xp/Actor.cpp.o
[ 15%] Building CXX object CMakeFiles/base.dir/game/AFEntity.cpp.o
[ 15%] Building CXX object CMakeFiles/d3xp.dir/d3xp/Entity.cpp.o
[ 15%] Building CXX object CMakeFiles/base.dir/game/AF.cpp.o
[ 15%] Building CXX object CMakeFiles/d3xp.dir/d3xp/Camera.cpp.o
[ 16%] Building CXX object CMakeFiles/dhewm3.dir/renderer/Cinematic.cpp.o
/home/gygy/my_project/Rust_Lang/99999999/1111/2222/dhewm3/neo/game/AF.cpp: In member function ‘bool idAF::Load(idEntity*, const char*)’:
/home/gygy/my_project/Rust_Lang/99999999/1111/2222/dhewm3/neo/game/AF.cpp:901:68: warning: comparison between ‘enum declAFConstraintType_t’ and ‘enum constraintType_t’ [-Wenum-compare]
  901 |                                         file->constraints[j]->type == constraint->GetType() ) {
/home/gygy/my_project/Rust_Lang/99999999/1111/2222/dhewm3/neo/d3xp/AF.cpp: In member function ‘bool idAF::Load(idEntity*, const char*)’:
/home/gygy/my_project/Rust_Lang/99999999/1111/2222/dhewm3/neo/d3xp/AF.cpp:901:68: warning: comparison between ‘enum declAFConstraintType_t’ and ‘enum constraintType_t’ [-Wenum-compare]
  901 |                                         file->constraints[j]->type == constraint->GetType() ) {
[ 16%] Building CXX object CMakeFiles/d3xp.dir/d3xp/BrittleFracture.cpp.o
[ 16%] Building CXX object CMakeFiles/dhewm3.dir/renderer/GuiModel.cpp.o
[ 16%] Building CXX object CMakeFiles/base.dir/game/Actor.cpp.o
[ 17%] Building CXX object CMakeFiles/d3xp.dir/d3xp/Fx.cpp.o
[ 17%] Building CXX object CMakeFiles/dhewm3.dir/renderer/Image_files.cpp.o
[ 18%] Building CXX object CMakeFiles/base.dir/game/Camera.cpp.o
[ 18%] Building CXX object CMakeFiles/d3xp.dir/d3xp/GameEdit.cpp.o
[ 18%] Building CXX object CMakeFiles/d3xp.dir/d3xp/Game_local.cpp.o
[ 18%] Building CXX object CMakeFiles/d3xp.dir/d3xp/Game_network.cpp.o
[ 19%] Building CXX object CMakeFiles/dhewm3.dir/renderer/Image_init.cpp.o
[ 19%] Building CXX object CMakeFiles/base.dir/game/Entity.cpp.o
[ 20%] Building CXX object CMakeFiles/d3xp.dir/d3xp/Item.cpp.o
[ 20%] Building CXX object CMakeFiles/base.dir/game/BrittleFracture.cpp.o
[ 20%] Building CXX object CMakeFiles/d3xp.dir/d3xp/IK.cpp.o
[ 20%] Building CXX object CMakeFiles/d3xp.dir/d3xp/Light.cpp.o
[ 20%] Building CXX object CMakeFiles/dhewm3.dir/renderer/Image_load.cpp.o
[ 21%] Building CXX object CMakeFiles/d3xp.dir/d3xp/Misc.cpp.o
[ 21%] Building CXX object CMakeFiles/base.dir/game/Fx.cpp.o
[ 21%] Building CXX object CMakeFiles/d3xp.dir/d3xp/Mover.cpp.o
[ 21%] Building CXX object CMakeFiles/dhewm3.dir/renderer/Image_process.cpp.o
[ 21%] Building CXX object CMakeFiles/d3xp.dir/d3xp/Moveable.cpp.o
[ 21%] Building CXX object CMakeFiles/d3xp.dir/d3xp/MultiplayerGame.cpp.o
[ 21%] Building CXX object CMakeFiles/dhewm3.dir/renderer/Image_program.cpp.o
[ 22%] Building CXX object CMakeFiles/dhewm3.dir/renderer/Interaction.cpp.o
[ 23%] Building CXX object CMakeFiles/base.dir/game/GameEdit.cpp.o
[ 23%] Building CXX object CMakeFiles/dhewm3.dir/renderer/Material.cpp.o
/home/gygy/my_project/Rust_Lang/99999999/1111/2222/dhewm3/neo/d3xp/MultiplayerGame.cpp: In member function ‘const char* idMultiplayerGame::GameTime()’:
/home/gygy/my_project/Rust_Lang/99999999/1111/2222/dhewm3/neo/d3xp/MultiplayerGame.cpp:864:38: warning: ‘%i’ directive writing between 1 and 8 bytes into a region of size between 3 and 13 [-Wformat-overflow=]
  864 |                 sprintf( buff, "%i:%i%i", m, t, s );
      |                                      ^~
In member function ‘const char* idMultiplayerGame::GameTime()’,
    inlined from ‘const char* idMultiplayerGame::GameTime()’ at /home/gygy/my_project/Rust_Lang/99999999/1111/2222/dhewm3/neo/d3xp/MultiplayerGame.cpp:835:13:
/home/gygy/my_project/Rust_Lang/99999999/1111/2222/dhewm3/neo/d3xp/MultiplayerGame.cpp:864:32: note: directive argument in the range [-4294940, 4294943]
  864 |                 sprintf( buff, "%i:%i%i", m, t, s );
      |                                ^~~~~~~~~
In file included from /usr/include/stdio.h:980,
                 from /home/gygy/my_project/Rust_Lang/99999999/1111/2222/dhewm3/neo/sys/platform.h:237,
                 from /home/gygy/my_project/Rust_Lang/99999999/1111/2222/dhewm3/neo/d3xp/MultiplayerGame.cpp:29:
In function ‘int sprintf(char*, const char*, ...)’,
    inlined from ‘const char* idMultiplayerGame::GameTime()’ at /home/gygy/my_project/Rust_Lang/99999999/1111/2222/dhewm3/neo/d3xp/MultiplayerGame.cpp:864:10,
    inlined from ‘const char* idMultiplayerGame::GameTime()’ at /home/gygy/my_project/Rust_Lang/99999999/1111/2222/dhewm3/neo/d3xp/MultiplayerGame.cpp:835:13:
/usr/include/x86_64-linux-gnu/bits/stdio2.h:30:34: note: ‘__builtin___sprintf_chk’ output between 5 and 22 bytes into a destination of size 16
   30 |   return __builtin___sprintf_chk (__s, __USE_FORTIFY_LEVEL - 1,
      |          ~~~~~~~~~~~~~~~~~~~~~~~~^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
   31 |                                   __glibc_objsize (__s), __fmt,
      |                                   ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
   32 |                                   __va_arg_pack ());
      |                                   ~~~~~~~~~~~~~~~~~
[ 23%] Building CXX object CMakeFiles/dhewm3.dir/renderer/MegaTexture.cpp.o
[ 24%] Building CXX object CMakeFiles/d3xp.dir/d3xp/Player.cpp.o
[ 24%] Building CXX object CMakeFiles/base.dir/game/Game_local.cpp.o
[ 24%] Building CXX object CMakeFiles/d3xp.dir/d3xp/PlayerIcon.cpp.o
[ 24%] Building CXX object CMakeFiles/dhewm3.dir/renderer/Model.cpp.o
[ 25%] Building CXX object CMakeFiles/dhewm3.dir/renderer/ModelDecal.cpp.o
[ 25%] Building CXX object CMakeFiles/base.dir/game/Game_network.cpp.o
[ 25%] Building CXX object CMakeFiles/d3xp.dir/d3xp/PlayerView.cpp.o
[ 25%] Building CXX object CMakeFiles/dhewm3.dir/renderer/ModelManager.cpp.o
[ 25%] Building CXX object CMakeFiles/d3xp.dir/d3xp/Projectile.cpp.o
[ 25%] Building CXX object CMakeFiles/dhewm3.dir/renderer/ModelOverlay.cpp.o
[ 26%] Building CXX object CMakeFiles/dhewm3.dir/renderer/Model_beam.cpp.o
[ 27%] Building CXX object CMakeFiles/d3xp.dir/d3xp/Pvs.cpp.o
[ 28%] Building CXX object CMakeFiles/base.dir/game/Item.cpp.o
[ 28%] Building CXX object CMakeFiles/dhewm3.dir/renderer/Model_ase.cpp.o
[ 28%] Building CXX object CMakeFiles/dhewm3.dir/renderer/Model_liquid.cpp.o
[ 28%] Building CXX object CMakeFiles/dhewm3.dir/renderer/Model_lwo.cpp.o
[ 29%] Building CXX object CMakeFiles/dhewm3.dir/renderer/Model_ma.cpp.o
[ 29%] Building CXX object CMakeFiles/base.dir/game/IK.cpp.o
[ 29%] Building CXX object CMakeFiles/d3xp.dir/d3xp/SecurityCamera.cpp.o
[ 29%] Building CXX object CMakeFiles/dhewm3.dir/renderer/Model_md3.cpp.o
[ 29%] Building CXX object CMakeFiles/base.dir/game/Light.cpp.o
[ 29%] Building CXX object CMakeFiles/d3xp.dir/d3xp/SmokeParticles.cpp.o
[ 29%] Building CXX object CMakeFiles/dhewm3.dir/renderer/Model_md5.cpp.o
[ 29%] Building CXX object CMakeFiles/dhewm3.dir/renderer/Model_prt.cpp.o
[ 30%] Building CXX object CMakeFiles/d3xp.dir/d3xp/Sound.cpp.o
[ 31%] Building CXX object CMakeFiles/dhewm3.dir/renderer/Model_sprite.cpp.o
[ 31%] Building CXX object CMakeFiles/base.dir/game/Misc.cpp.o
[ 32%] Building CXX object CMakeFiles/base.dir/game/Mover.cpp.o
[ 32%] Building CXX object CMakeFiles/base.dir/game/Moveable.cpp.o
[ 32%] Building CXX object CMakeFiles/d3xp.dir/d3xp/Target.cpp.o
[ 32%] Building CXX object CMakeFiles/d3xp.dir/d3xp/Trigger.cpp.o
[ 32%] Building CXX object CMakeFiles/dhewm3.dir/renderer/RenderEntity.cpp.o
[ 32%] Building CXX object CMakeFiles/dhewm3.dir/renderer/RenderSystem.cpp.o
[ 33%] Building CXX object CMakeFiles/dhewm3.dir/renderer/RenderSystem_init.cpp.o
[ 33%] Building CXX object CMakeFiles/base.dir/game/MultiplayerGame.cpp.o
[ 33%] Building CXX object CMakeFiles/d3xp.dir/d3xp/Weapon.cpp.o
[ 33%] Building CXX object CMakeFiles/dhewm3.dir/renderer/RenderWorld.cpp.o
[ 34%] Building CXX object CMakeFiles/d3xp.dir/d3xp/WorldSpawn.cpp.o
[ 34%] Building CXX object CMakeFiles/base.dir/game/Player.cpp.o
/home/gygy/my_project/Rust_Lang/99999999/1111/2222/dhewm3/neo/game/MultiplayerGame.cpp: In member function ‘const char* idMultiplayerGame::GameTime()’:
/home/gygy/my_project/Rust_Lang/99999999/1111/2222/dhewm3/neo/game/MultiplayerGame.cpp:564:38: warning: ‘%i’ directive writing between 1 and 8 bytes into a region of size between 3 and 13 [-Wformat-overflow=]
  564 |                 sprintf( buff, "%i:%i%i", m, t, s );
      |                                      ^~
In member function ‘const char* idMultiplayerGame::GameTime()’,
    inlined from ‘const char* idMultiplayerGame::GameTime()’ at /home/gygy/my_project/Rust_Lang/99999999/1111/2222/dhewm3/neo/game/MultiplayerGame.cpp:534:13:
/home/gygy/my_project/Rust_Lang/99999999/1111/2222/dhewm3/neo/game/MultiplayerGame.cpp:564:32: note: directive argument in the range [-4294940, 4294943]
  564 |                 sprintf( buff, "%i:%i%i", m, t, s );
      |                                ^~~~~~~~~
In file included from /usr/include/stdio.h:980,
                 from /home/gygy/my_project/Rust_Lang/99999999/1111/2222/dhewm3/neo/sys/platform.h:237,
                 from /home/gygy/my_project/Rust_Lang/99999999/1111/2222/dhewm3/neo/game/MultiplayerGame.cpp:29:
In function ‘int sprintf(char*, const char*, ...)’,
    inlined from ‘const char* idMultiplayerGame::GameTime()’ at /home/gygy/my_project/Rust_Lang/99999999/1111/2222/dhewm3/neo/game/MultiplayerGame.cpp:564:10,
    inlined from ‘const char* idMultiplayerGame::GameTime()’ at /home/gygy/my_project/Rust_Lang/99999999/1111/2222/dhewm3/neo/game/MultiplayerGame.cpp:534:13:
/usr/include/x86_64-linux-gnu/bits/stdio2.h:30:34: note: ‘__builtin___sprintf_chk’ output between 5 and 22 bytes into a destination of size 16
   30 |   return __builtin___sprintf_chk (__s, __USE_FORTIFY_LEVEL - 1,
      |          ~~~~~~~~~~~~~~~~~~~~~~~~^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
   31 |                                   __glibc_objsize (__s), __fmt,
      |                                   ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
   32 |                                   __va_arg_pack ());
      |                                   ~~~~~~~~~~~~~~~~~
[ 34%] Building CXX object CMakeFiles/d3xp.dir/d3xp/ai/AAS.cpp.o
[ 35%] Building CXX object CMakeFiles/base.dir/game/PlayerIcon.cpp.o
[ 35%] Building CXX object CMakeFiles/dhewm3.dir/renderer/RenderWorld_demo.cpp.o
[ 35%] Building CXX object CMakeFiles/d3xp.dir/d3xp/ai/AAS_debug.cpp.o
[ 35%] Building CXX object CMakeFiles/base.dir/game/PlayerView.cpp.o
[ 35%] Building CXX object CMakeFiles/dhewm3.dir/renderer/RenderWorld_load.cpp.o
[ 35%] Building CXX object CMakeFiles/base.dir/game/Projectile.cpp.o
[ 36%] Building CXX object CMakeFiles/dhewm3.dir/renderer/RenderWorld_portals.cpp.o
[ 36%] Building CXX object CMakeFiles/dhewm3.dir/renderer/VertexCache.cpp.o
[ 36%] Building CXX object CMakeFiles/d3xp.dir/d3xp/ai/AAS_pathing.cpp.o
[ 36%] Building CXX object CMakeFiles/dhewm3.dir/renderer/draw_arb2.cpp.o
[ 37%] Building CXX object CMakeFiles/base.dir/game/Pvs.cpp.o
[ 37%] Building CXX object CMakeFiles/dhewm3.dir/renderer/draw_common.cpp.o
[ 38%] Building CXX object CMakeFiles/dhewm3.dir/renderer/tr_backend.cpp.o
[ 38%] Building CXX object CMakeFiles/dhewm3.dir/renderer/tr_deform.cpp.o
[ 39%] Building CXX object CMakeFiles/d3xp.dir/d3xp/ai/AAS_routing.cpp.o
[ 39%] Building CXX object CMakeFiles/base.dir/game/SecurityCamera.cpp.o
[ 39%] Building CXX object CMakeFiles/dhewm3.dir/renderer/tr_font.cpp.o
[ 40%] Building CXX object CMakeFiles/dhewm3.dir/renderer/tr_guisurf.cpp.o
[ 40%] Building CXX object CMakeFiles/d3xp.dir/d3xp/ai/AI.cpp.o
[ 40%] Building CXX object CMakeFiles/base.dir/game/SmokeParticles.cpp.o
[ 40%] Building CXX object CMakeFiles/dhewm3.dir/renderer/tr_light.cpp.o
[ 40%] Building CXX object CMakeFiles/dhewm3.dir/renderer/tr_lightrun.cpp.o
[ 40%] Building CXX object CMakeFiles/dhewm3.dir/renderer/tr_main.cpp.o
[ 40%] Building CXX object CMakeFiles/d3xp.dir/d3xp/ai/AI_events.cpp.o
[ 40%] Building CXX object CMakeFiles/base.dir/game/Sound.cpp.o
[ 41%] Building CXX object CMakeFiles/base.dir/game/Target.cpp.o
[ 42%] Building CXX object CMakeFiles/dhewm3.dir/renderer/tr_orderIndexes.cpp.o
[ 42%] Building CXX object CMakeFiles/base.dir/game/Trigger.cpp.o
[ 42%] Building CXX object CMakeFiles/dhewm3.dir/renderer/tr_polytope.cpp.o
[ 42%] Building CXX object CMakeFiles/dhewm3.dir/renderer/tr_render.cpp.o
[ 42%] Building CXX object CMakeFiles/dhewm3.dir/renderer/tr_rendertools.cpp.o
[ 43%] Building CXX object CMakeFiles/dhewm3.dir/renderer/tr_shadowbounds.cpp.o
[ 43%] Building CXX object CMakeFiles/dhewm3.dir/renderer/tr_stencilshadow.cpp.o
[ 43%] Building CXX object CMakeFiles/base.dir/game/Weapon.cpp.o
[ 44%] Building CXX object CMakeFiles/d3xp.dir/d3xp/ai/AI_pathing.cpp.o
[ 44%] Building CXX object CMakeFiles/d3xp.dir/d3xp/ai/AI_Vagary.cpp.o
[ 44%] Building CXX object CMakeFiles/base.dir/game/WorldSpawn.cpp.o
[ 45%] Building CXX object CMakeFiles/base.dir/game/ai/AAS.cpp.o
[ 45%] Building CXX object CMakeFiles/dhewm3.dir/renderer/tr_subview.cpp.o
[ 45%] Building CXX object CMakeFiles/d3xp.dir/d3xp/gamesys/DebugGraph.cpp.o
[ 45%] Building CXX object CMakeFiles/d3xp.dir/d3xp/gamesys/Class.cpp.o
[ 45%] Building CXX object CMakeFiles/base.dir/game/ai/AAS_debug.cpp.o
[ 45%] Building CXX object CMakeFiles/base.dir/game/ai/AAS_pathing.cpp.o
[ 46%] Building CXX object CMakeFiles/d3xp.dir/d3xp/gamesys/Event.cpp.o
[ 47%] Building CXX object CMakeFiles/dhewm3.dir/renderer/tr_trace.cpp.o
[ 47%] Building CXX object CMakeFiles/d3xp.dir/d3xp/gamesys/SaveGame.cpp.o
[ 47%] Building CXX object CMakeFiles/d3xp.dir/d3xp/gamesys/SysCmds.cpp.o
[ 48%] Building CXX object CMakeFiles/base.dir/game/ai/AAS_routing.cpp.o
[ 48%] Building CXX object CMakeFiles/base.dir/game/ai/AI.cpp.o
[ 48%] Building CXX object CMakeFiles/dhewm3.dir/renderer/tr_trisurf.cpp.o
[ 48%] Building CXX object CMakeFiles/dhewm3.dir/renderer/tr_turboshadow.cpp.o
[ 48%] Building CXX object CMakeFiles/d3xp.dir/d3xp/gamesys/SysCvar.cpp.o
[ 48%] Building C object CMakeFiles/dhewm3.dir/renderer/stblib_impls.c.o
[ 49%] Building CXX object CMakeFiles/d3xp.dir/d3xp/gamesys/TypeInfo.cpp.o
[ 49%] Building CXX object CMakeFiles/base.dir/game/ai/AI_events.cpp.o
[ 49%] Building CXX object CMakeFiles/d3xp.dir/d3xp/anim/Anim.cpp.o
[ 49%] Building CXX object CMakeFiles/d3xp.dir/d3xp/anim/Anim_Blend.cpp.o
[ 50%] Building CXX object CMakeFiles/d3xp.dir/d3xp/anim/Anim_Import.cpp.o
[ 50%] Building CXX object CMakeFiles/d3xp.dir/d3xp/anim/Anim_Testmodel.cpp.o
[ 51%] Building CXX object CMakeFiles/dhewm3.dir/framework/CVarSystem.cpp.o
[ 51%] Building CXX object CMakeFiles/d3xp.dir/d3xp/script/Script_Compiler.cpp.o
[ 51%] Building CXX object CMakeFiles/d3xp.dir/d3xp/script/Script_Interpreter.cpp.o
[ 51%] Building CXX object CMakeFiles/base.dir/game/ai/AI_pathing.cpp.o
[ 52%] Building CXX object CMakeFiles/base.dir/game/ai/AI_Vagary.cpp.o
[ 52%] Building CXX object CMakeFiles/dhewm3.dir/framework/CmdSystem.cpp.o
[ 52%] Building CXX object CMakeFiles/dhewm3.dir/framework/Common.cpp.o
[ 52%] Building CXX object CMakeFiles/base.dir/game/gamesys/DebugGraph.cpp.o
[ 53%] Building CXX object CMakeFiles/d3xp.dir/d3xp/script/Script_Program.cpp.o
[ 53%] Building CXX object CMakeFiles/base.dir/game/gamesys/Class.cpp.o
[ 53%] Building CXX object CMakeFiles/dhewm3.dir/framework/Compressor.cpp.o
[ 53%] Building CXX object CMakeFiles/d3xp.dir/d3xp/script/Script_Thread.cpp.o
[ 53%] Building CXX object CMakeFiles/base.dir/game/gamesys/Event.cpp.o
[ 54%] Building CXX object CMakeFiles/base.dir/game/gamesys/SaveGame.cpp.o
[ 54%] Building CXX object CMakeFiles/d3xp.dir/d3xp/physics/Clip.cpp.o
[ 55%] Building CXX object CMakeFiles/dhewm3.dir/framework/Console.cpp.o
[ 55%] Building CXX object CMakeFiles/base.dir/game/gamesys/SysCmds.cpp.o
[ 55%] Building CXX object CMakeFiles/d3xp.dir/d3xp/physics/Force.cpp.o
[ 56%] Building CXX object CMakeFiles/d3xp.dir/d3xp/physics/Force_Constant.cpp.o
[ 56%] Building CXX object CMakeFiles/base.dir/game/gamesys/SysCvar.cpp.o
[ 56%] Building CXX object CMakeFiles/d3xp.dir/d3xp/physics/Force_Drag.cpp.o
[ 56%] Building CXX object CMakeFiles/dhewm3.dir/framework/DemoFile.cpp.o
[ 56%] Building CXX object CMakeFiles/d3xp.dir/d3xp/physics/Force_Field.cpp.o
[ 57%] Building CXX object CMakeFiles/base.dir/game/gamesys/TypeInfo.cpp.o
[ 57%] Building CXX object CMakeFiles/base.dir/game/anim/Anim.cpp.o
[ 57%] Building CXX object CMakeFiles/dhewm3.dir/framework/DeclAF.cpp.o
[ 58%] Building CXX object CMakeFiles/d3xp.dir/d3xp/physics/Force_Spring.cpp.o
[ 58%] Building CXX object CMakeFiles/d3xp.dir/d3xp/physics/Physics.cpp.o
[ 58%] Building CXX object CMakeFiles/d3xp.dir/d3xp/physics/Physics_AF.cpp.o
[ 58%] Building CXX object CMakeFiles/d3xp.dir/d3xp/physics/Physics_Actor.cpp.o
[ 59%] Building CXX object CMakeFiles/d3xp.dir/d3xp/physics/Physics_Base.cpp.o
[ 59%] Building CXX object CMakeFiles/base.dir/game/anim/Anim_Blend.cpp.o
[ 59%] Building CXX object CMakeFiles/base.dir/game/anim/Anim_Import.cpp.o
[ 60%] Building CXX object CMakeFiles/base.dir/game/anim/Anim_Testmodel.cpp.o
[ 60%] Building CXX object CMakeFiles/base.dir/game/script/Script_Compiler.cpp.o
[ 60%] Building CXX object CMakeFiles/d3xp.dir/d3xp/physics/Physics_Monster.cpp.o
[ 60%] Building CXX object CMakeFiles/base.dir/game/script/Script_Interpreter.cpp.o
[ 61%] Building CXX object CMakeFiles/dhewm3.dir/framework/DeclEntityDef.cpp.o
[ 61%] Building CXX object CMakeFiles/dhewm3.dir/framework/DeclFX.cpp.o
[ 61%] Building CXX object CMakeFiles/dhewm3.dir/framework/DeclManager.cpp.o
[ 61%] Building CXX object CMakeFiles/base.dir/game/script/Script_Program.cpp.o
[ 62%] Building CXX object CMakeFiles/base.dir/game/script/Script_Thread.cpp.o
[ 62%] Building CXX object CMakeFiles/base.dir/game/physics/Clip.cpp.o
[ 62%] Building CXX object CMakeFiles/dhewm3.dir/framework/DeclParticle.cpp.o
[ 62%] Building CXX object CMakeFiles/base.dir/game/physics/Force.cpp.o
[ 63%] Building CXX object CMakeFiles/base.dir/game/physics/Force_Constant.cpp.o
[ 63%] Building CXX object CMakeFiles/base.dir/game/physics/Force_Drag.cpp.o
[ 64%] Building CXX object CMakeFiles/dhewm3.dir/framework/DeclPDA.cpp.o
[ 64%] Building CXX object CMakeFiles/base.dir/game/physics/Force_Field.cpp.o
[ 64%] Building CXX object CMakeFiles/base.dir/game/physics/Force_Spring.cpp.o
[ 65%] Building CXX object CMakeFiles/base.dir/game/physics/Physics.cpp.o
[ 65%] Building CXX object CMakeFiles/base.dir/game/physics/Physics_AF.cpp.o
[ 65%] Building CXX object CMakeFiles/base.dir/game/physics/Physics_Actor.cpp.o
[ 65%] Building CXX object CMakeFiles/base.dir/game/physics/Physics_Base.cpp.o
[ 66%] Building CXX object CMakeFiles/base.dir/game/physics/Physics_Monster.cpp.o
[ 66%] Building CXX object CMakeFiles/dhewm3.dir/framework/DeclSkin.cpp.o
[ 66%] Building CXX object CMakeFiles/dhewm3.dir/framework/DeclTable.cpp.o
[ 66%] Building CXX object CMakeFiles/dhewm3.dir/framework/Dhewm3SettingsMenu.cpp.o
[ 67%] Building CXX object CMakeFiles/dhewm3.dir/framework/EditField.cpp.o
[ 67%] Building CXX object CMakeFiles/dhewm3.dir/framework/EventLoop.cpp.o
[ 67%] Building CXX object CMakeFiles/base.dir/game/physics/Physics_Parametric.cpp.o
[ 67%] Building CXX object CMakeFiles/base.dir/game/physics/Physics_Player.cpp.o
[ 68%] Building CXX object CMakeFiles/base.dir/game/physics/Physics_RigidBody.cpp.o
[ 68%] Building CXX object CMakeFiles/dhewm3.dir/framework/File.cpp.o
[ 69%] Building CXX object CMakeFiles/dhewm3.dir/framework/FileSystem.cpp.o
/home/gygy/my_project/Rust_Lang/99999999/1111/2222/dhewm3/neo/framework/FileSystem.cpp: In function ‘int BackgroundDownloadThread(void*)’:
/home/gygy/my_project/Rust_Lang/99999999/1111/2222/dhewm3/neo/framework/FileSystem.cpp:3452:38: warning: ignoring return value of ‘size_t fread(void*, size_t, size_t, FILE*)’ declared with attribute ‘warn_unused_result’ [-Wunused-result]
 3452 |                                 fread(  bgl->file.buffer, bgl->file.length, 1, static_cast<idFile_Permanent*>(bgl->f)->GetFilePtr() );
      |                                 ~~~~~^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
[ 69%] Building CXX object CMakeFiles/dhewm3.dir/framework/KeyInput.cpp.o
[ 69%] Building CXX object CMakeFiles/dhewm3.dir/framework/UsercmdGen.cpp.o
[ 69%] Building CXX object CMakeFiles/base.dir/game/physics/Physics_Static.cpp.o
[ 69%] Building CXX object CMakeFiles/base.dir/game/physics/Physics_StaticMulti.cpp.o
[ 69%] Building CXX object CMakeFiles/d3xp.dir/d3xp/physics/Physics_Parametric.cpp.o
[ 69%] Building CXX object CMakeFiles/dhewm3.dir/framework/Session_menu.cpp.o
[ 69%] Building CXX object CMakeFiles/base.dir/game/physics/Push.cpp.o
[ 69%] Building CXX object CMakeFiles/d3xp.dir/d3xp/physics/Physics_Player.cpp.o
[ 70%] Building CXX object CMakeFiles/dhewm3.dir/framework/Session.cpp.o
[ 70%] Building CXX object CMakeFiles/dhewm3.dir/framework/async/AsyncClient.cpp.o
[ 71%] Building CXX object CMakeFiles/d3xp.dir/d3xp/physics/Physics_RigidBody.cpp.o
[ 71%] Building CXX object CMakeFiles/d3xp.dir/d3xp/physics/Physics_Static.cpp.o
[ 71%] Building CXX object CMakeFiles/dhewm3.dir/framework/async/AsyncNetwork.cpp.o
[ 71%] Building CXX object CMakeFiles/d3xp.dir/d3xp/physics/Physics_StaticMulti.cpp.o
[ 72%] Building CXX object CMakeFiles/d3xp.dir/d3xp/physics/Push.cpp.o
[ 73%] Building CXX object CMakeFiles/dhewm3.dir/framework/async/AsyncServer.cpp.o
[ 73%] Building CXX object CMakeFiles/d3xp.dir/d3xp/Grabber.cpp.o
[ 73%] Building CXX object CMakeFiles/dhewm3.dir/framework/async/MsgChannel.cpp.o
[ 73%] Building CXX object CMakeFiles/d3xp.dir/d3xp/physics/Force_Grab.cpp.o
[ 73%] Building CXX object CMakeFiles/dhewm3.dir/framework/async/NetworkSystem.cpp.o
[ 73%] Building CXX object CMakeFiles/dhewm3.dir/framework/async/ServerScan.cpp.o
[ 74%] Building C object CMakeFiles/dhewm3.dir/framework/miniz/miniz.c.o
[ 74%] Building C object CMakeFiles/dhewm3.dir/framework/minizip/ioapi.c.o
[ 74%] Building CXX object CMakeFiles/dhewm3.dir/framework/minizip/unzip.cpp.o
[ 74%] Building CXX object CMakeFiles/dhewm3.dir/cm/CollisionModel_contacts.cpp.o
[ 74%] Linking CXX shared library d3xp.so
[ 75%] Building CXX object CMakeFiles/dhewm3.dir/cm/CollisionModel_contents.cpp.o
[ 75%] Building CXX object CMakeFiles/dhewm3.dir/cm/CollisionModel_debug.cpp.o
[ 75%] Built target d3xp
[ 75%] Building CXX object CMakeFiles/dhewm3.dir/cm/CollisionModel_files.cpp.o
[ 76%] Building CXX object CMakeFiles/dhewm3.dir/cm/CollisionModel_load.cpp.o
[ 76%] Building CXX object CMakeFiles/dhewm3.dir/cm/CollisionModel_rotate.cpp.o
[ 76%] Building CXX object CMakeFiles/dhewm3.dir/cm/CollisionModel_trace.cpp.o
[ 76%] Building CXX object CMakeFiles/dhewm3.dir/cm/CollisionModel_translate.cpp.o
[ 77%] Building CXX object CMakeFiles/dhewm3.dir/tools/compilers/dmap/dmap.cpp.o
[ 77%] Building CXX object CMakeFiles/dhewm3.dir/tools/compilers/dmap/facebsp.cpp.o
[ 77%] Building CXX object CMakeFiles/dhewm3.dir/tools/compilers/dmap/gldraw.cpp.o
[ 77%] Building CXX object CMakeFiles/dhewm3.dir/tools/compilers/dmap/glfile.cpp.o
/home/gygy/my_project/Rust_Lang/99999999/1111/2222/dhewm3/neo/tools/compilers/dmap/dmap.cpp: In function ‘void Dmap(const idCmdArgs&)’:
/home/gygy/my_project/Rust_Lang/99999999/1111/2222/dhewm3/neo/tools/compilers/dmap/dmap.cpp:312:35: warning: ‘.reg’ directive writing 4 bytes into a region of size between 1 and 1024 [-Wformat-overflow=]
  312 |                 sprintf( path, "%s.reg", dmapGlobals.mapFileBase );
      |                                   ^~~~
In file included from /usr/include/stdio.h:980,
                 from /home/gygy/my_project/Rust_Lang/99999999/1111/2222/dhewm3/neo/sys/platform.h:237,
                 from /home/gygy/my_project/Rust_Lang/99999999/1111/2222/dhewm3/neo/tools/compilers/dmap/dmap.cpp:29:
In function ‘int sprintf(char*, const char*, ...)’,
    inlined from ‘void Dmap(const idCmdArgs&)’ at /home/gygy/my_project/Rust_Lang/99999999/1111/2222/dhewm3/neo/tools/compilers/dmap/dmap.cpp:312:10:
/usr/include/x86_64-linux-gnu/bits/stdio2.h:30:34: note: ‘__builtin___sprintf_chk’ output between 5 and 1028 bytes into a destination of size 1024
   30 |   return __builtin___sprintf_chk (__s, __USE_FORTIFY_LEVEL - 1,
      |          ~~~~~~~~~~~~~~~~~~~~~~~~^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
   31 |                                   __glibc_objsize (__s), __fmt,
      |                                   ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
   32 |                                   __va_arg_pack ());
      |                                   ~~~~~~~~~~~~~~~~~
/home/gygy/my_project/Rust_Lang/99999999/1111/2222/dhewm3/neo/tools/compilers/dmap/dmap.cpp: In function ‘void Dmap(const idCmdArgs&)’:
/home/gygy/my_project/Rust_Lang/99999999/1111/2222/dhewm3/neo/tools/compilers/dmap/dmap.cpp:322:27: warning: ‘.lin’ directive writing 4 bytes into a region of size between 1 and 1024 [-Wformat-overflow=]
  322 |         sprintf( path, "%s.lin", dmapGlobals.mapFileBase );
      |                           ^~~~
In function ‘int sprintf(char*, const char*, ...)’,
    inlined from ‘void Dmap(const idCmdArgs&)’ at /home/gygy/my_project/Rust_Lang/99999999/1111/2222/dhewm3/neo/tools/compilers/dmap/dmap.cpp:322:9:
/usr/include/x86_64-linux-gnu/bits/stdio2.h:30:34: note: ‘__builtin___sprintf_chk’ output between 5 and 1028 bytes into a destination of size 1024
   30 |   return __builtin___sprintf_chk (__s, __USE_FORTIFY_LEVEL - 1,
      |          ~~~~~~~~~~~~~~~~~~~~~~~~^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
   31 |                                   __glibc_objsize (__s), __fmt,
      |                                   ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
   32 |                                   __va_arg_pack ());
      |                                   ~~~~~~~~~~~~~~~~~
[ 78%] Building CXX object CMakeFiles/dhewm3.dir/tools/compilers/dmap/leakfile.cpp.o
[ 79%] Linking CXX shared library base.so
[ 79%] Building CXX object CMakeFiles/dhewm3.dir/tools/compilers/dmap/map.cpp.o
[ 79%] Building CXX object CMakeFiles/dhewm3.dir/tools/compilers/dmap/optimize.cpp.o
[ 80%] Building CXX object CMakeFiles/dhewm3.dir/tools/compilers/dmap/portals.cpp.o
[ 80%] Building CXX object CMakeFiles/dhewm3.dir/tools/compilers/dmap/output.cpp.o
[ 80%] Building CXX object CMakeFiles/dhewm3.dir/tools/compilers/dmap/shadowopt3.cpp.o
[ 80%] Building CXX object CMakeFiles/dhewm3.dir/tools/compilers/dmap/tritjunction.cpp.o
[ 80%] Built target base
[ 81%] Building CXX object CMakeFiles/dhewm3.dir/tools/compilers/dmap/tritools.cpp.o
[ 81%] Building CXX object CMakeFiles/dhewm3.dir/tools/compilers/dmap/ubrush.cpp.o
[ 81%] Building CXX object CMakeFiles/dhewm3.dir/tools/compilers/dmap/usurface.cpp.o
[ 81%] Building CXX object CMakeFiles/dhewm3.dir/tools/compilers/aas/AASBuild.cpp.o
[ 82%] Building CXX object CMakeFiles/dhewm3.dir/tools/compilers/aas/AASBuild_file.cpp.o
[ 82%] Building CXX object CMakeFiles/dhewm3.dir/tools/compilers/aas/AASBuild_gravity.cpp.o
[ 82%] Building CXX object CMakeFiles/dhewm3.dir/tools/compilers/aas/AASBuild_ledge.cpp.o
[ 83%] Building CXX object CMakeFiles/dhewm3.dir/tools/compilers/aas/AASBuild_merge.cpp.o
[ 83%] Building CXX object CMakeFiles/dhewm3.dir/tools/compilers/aas/AASCluster.cpp.o
[ 83%] Building CXX object CMakeFiles/dhewm3.dir/tools/compilers/aas/AASFile.cpp.o
[ 84%] Building CXX object CMakeFiles/dhewm3.dir/tools/compilers/aas/AASFile_optimize.cpp.o
[ 84%] Building CXX object CMakeFiles/dhewm3.dir/tools/compilers/aas/AASFile_sample.cpp.o
[ 84%] Building CXX object CMakeFiles/dhewm3.dir/tools/compilers/aas/AASReach.cpp.o
[ 84%] Building CXX object CMakeFiles/dhewm3.dir/tools/compilers/aas/AASFileManager.cpp.o
[ 85%] Building CXX object CMakeFiles/dhewm3.dir/tools/compilers/aas/Brush.cpp.o
[ 85%] Building CXX object CMakeFiles/dhewm3.dir/tools/compilers/aas/BrushBSP.cpp.o
[ 85%] Building CXX object CMakeFiles/dhewm3.dir/tools/compilers/roqvq/NSBitmapImageRep.cpp.o
[ 85%] Building CXX object CMakeFiles/dhewm3.dir/tools/compilers/roqvq/codec.cpp.o
[ 86%] Building CXX object CMakeFiles/dhewm3.dir/tools/compilers/roqvq/roq.cpp.o
[ 86%] Building CXX object CMakeFiles/dhewm3.dir/tools/compilers/roqvq/roqParam.cpp.o
[ 86%] Building CXX object CMakeFiles/dhewm3.dir/tools/compilers/renderbump/renderbump.cpp.o
[ 86%] Building CXX object CMakeFiles/dhewm3.dir/sound/snd_cache.cpp.o
[ 87%] Building CXX object CMakeFiles/dhewm3.dir/sound/snd_decoder.cpp.o
[ 87%] Building CXX object CMakeFiles/dhewm3.dir/sound/snd_efxfile.cpp.o
[ 87%] Building CXX object CMakeFiles/dhewm3.dir/sound/snd_emitter.cpp.o
[ 87%] Building CXX object CMakeFiles/dhewm3.dir/sound/snd_shader.cpp.o
[ 88%] Building CXX object CMakeFiles/dhewm3.dir/sound/snd_system.cpp.o
[ 88%] Building CXX object CMakeFiles/dhewm3.dir/sound/snd_wavefile.cpp.o
[ 88%] Building CXX object CMakeFiles/dhewm3.dir/sound/snd_world.cpp.o
[ 89%] Building C object CMakeFiles/dhewm3.dir/sound/stbvorbis_impl.c.o
[ 89%] Building CXX object CMakeFiles/dhewm3.dir/ui/BindWindow.cpp.o
[ 89%] Building CXX object CMakeFiles/dhewm3.dir/ui/ChoiceWindow.cpp.o
[ 89%] Building CXX object CMakeFiles/dhewm3.dir/ui/DeviceContext.cpp.o
[ 90%] Building CXX object CMakeFiles/dhewm3.dir/ui/EditWindow.cpp.o
[ 90%] Building CXX object CMakeFiles/dhewm3.dir/ui/FieldWindow.cpp.o
[ 90%] Building CXX object CMakeFiles/dhewm3.dir/ui/GameBearShootWindow.cpp.o
[ 90%] Building CXX object CMakeFiles/dhewm3.dir/ui/GameBustOutWindow.cpp.o
[ 91%] Building CXX object CMakeFiles/dhewm3.dir/ui/GameSSDWindow.cpp.o
[ 91%] Building CXX object CMakeFiles/dhewm3.dir/ui/GuiScript.cpp.o
[ 91%] Building CXX object CMakeFiles/dhewm3.dir/ui/ListGUI.cpp.o
[ 92%] Building CXX object CMakeFiles/dhewm3.dir/ui/ListWindow.cpp.o
[ 92%] Building CXX object CMakeFiles/dhewm3.dir/ui/MarkerWindow.cpp.o
[ 92%] Building CXX object CMakeFiles/dhewm3.dir/ui/RegExp.cpp.o
[ 92%] Building CXX object CMakeFiles/dhewm3.dir/ui/RenderWindow.cpp.o
[ 93%] Building CXX object CMakeFiles/dhewm3.dir/ui/SimpleWindow.cpp.o
[ 93%] Building CXX object CMakeFiles/dhewm3.dir/ui/SliderWindow.cpp.o
[ 93%] Building CXX object CMakeFiles/dhewm3.dir/ui/UserInterface.cpp.o
[ 94%] Building CXX object CMakeFiles/dhewm3.dir/ui/Window.cpp.o
[ 94%] Building CXX object CMakeFiles/dhewm3.dir/ui/Winvar.cpp.o
[ 94%] Building CXX object CMakeFiles/dhewm3.dir/sys/cpu.cpp.o
[ 94%] Building CXX object CMakeFiles/dhewm3.dir/sys/threads.cpp.o
[ 95%] Building CXX object CMakeFiles/dhewm3.dir/sys/events.cpp.o
[ 95%] Building CXX object CMakeFiles/dhewm3.dir/sys/sys_local.cpp.o
[ 95%] Building CXX object CMakeFiles/dhewm3.dir/sys/posix/posix_net.cpp.o
[ 95%] Building CXX object CMakeFiles/dhewm3.dir/sys/posix/posix_main.cpp.o
[ 96%] Building CXX object CMakeFiles/dhewm3.dir/sys/linux/main.cpp.o
[ 96%] Building CXX object CMakeFiles/dhewm3.dir/sys/glimp.cpp.o
[ 96%] Building CXX object CMakeFiles/dhewm3.dir/libs/imgui/backends/imgui_impl_sdl2.cpp.o
[ 97%] Building CXX object CMakeFiles/dhewm3.dir/libs/imgui/backends/imgui_impl_opengl2.cpp.o
[ 97%] Building CXX object CMakeFiles/dhewm3.dir/libs/imgui/imgui.cpp.o
[ 97%] Building CXX object CMakeFiles/dhewm3.dir/libs/imgui/imgui_draw.cpp.o
[ 97%] Building CXX object CMakeFiles/dhewm3.dir/libs/imgui/imgui_tables.cpp.o
[ 98%] Building CXX object CMakeFiles/dhewm3.dir/libs/imgui/imgui_widgets.cpp.o
[ 98%] Building CXX object CMakeFiles/dhewm3.dir/libs/imgui/imgui_demo.cpp.o
[ 98%] Building CXX object CMakeFiles/dhewm3.dir/sys/sys_imgui.cpp.o
[ 98%] Building CXX object CMakeFiles/dhewm3.dir/sys/imgui_savestyle.cpp.o
[ 99%] Building CXX object CMakeFiles/dhewm3.dir/tools/edit_stub.cpp.o
[ 99%] Building CXX object CMakeFiles/dhewm3.dir/tools/debugger/DebuggerBreakpoint.cpp.o
[ 99%] Building CXX object CMakeFiles/dhewm3.dir/tools/debugger/DebuggerServer.cpp.o
[100%] Building CXX object CMakeFiles/dhewm3.dir/tools/debugger/DebuggerScript.cpp.o
[100%] Building CXX object CMakeFiles/dhewm3.dir/tools/debugger/debugger.cpp.o
[100%] Linking CXX executable dhewm3
[100%] Built target dhewm3

dhewm3/build on  master via C v13.3.0-gcc via △ v3.28.3 took 30s
❯ ls
base.so*  CMakeCache.txt  CMakeFiles/  cmake_install.cmake  config.h  d3xp.so*  dhewm3*  libidlib.a  Makefile

dhewm3/build on  master via C v13.3.0-gcc via △ v3.28.3
❯ ./dhewm3
dhewm3 1.5.5pre.1305 linux-x86_64 Jan  4 2025 22:22:23 using SDL v2.30.0
SDL video driver: x11
Logging console output to /home/gygy/.local/share/dhewm3/dhewm3log.txt
terminal support enabled ( use +set in_tty 0 to disable )
pid: 16860
96336 MB System Memory
found interface lo - loopback
found interface enp5s0 - 175.211.98.57/255.255.255.0
found interface docker0 - 172.17.0.1/255.255.0.0
doom using MMX & SSE & SSE2 for SIMD processing
enabling Flush-To-Zero mode
enabling Denormals-Are-Zero mode
WARNING: base path '/usr/local/share/dhewm3' does not exist
----- Initializing File System -----
Current search path:
/home/gygy/.config/dhewm3/base
/home/gygy/.local/share/dhewm3/base
Addon pk4s:
WARNING: Couldn't find default.cfg in base/, trying again with demo/

----- Initializing File System -----
Current search path:
/home/gygy/.config/dhewm3/demo
/home/gygy/.local/share/dhewm3/demo
/home/gygy/.config/dhewm3/base
/home/gygy/.local/share/dhewm3/base
Addon pk4s:
shutting down: Couldn't load default.cfg
idRenderSystem::Shutdown()
Shutting down OpenGL subsystem
Sys_Error: Couldn't load default.cfg
shutdown terminal support
```
