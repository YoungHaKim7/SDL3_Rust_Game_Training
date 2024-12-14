# link

<hr />

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

# Rust SDL3

- https://crates.io/crates/sdl3
  - https://github.com/revmischa/sdl3-rs

```
cargo add sdl3  
```

- `Cargo.toml` 
```bash
sdl3 = "0.11.8"
```

# `Cargo.toml` 난 이렇게 함.

```toml

[dependencies]
sdl3 = { version = "0", features = [] }

[dependencies.sdl3-sys]
version = "0.1.3+SDL3-preview-3.1.6"
```

# SDL2 (Rust)
- https://github.com/Rust-SDL2/rust-sdl2

# Documentation

- [SDL3 higher-level documentation.](https://docs.rs/sdl3/latest/sdl3/)
- [SDL3-sys lower-level bindings documentation](https://docs.rs/sdl3-sys/latest/sdl3_sys/)

- [SDL3 wiki 문서_https://wiki.libsdl.org/SDL3/FrontPage](https://wiki.libsdl.org/SDL3/FrontPage)
- [SDL2 wiki 문서_https://wiki.libsdl.org/SDL2/FrontPage](https://wiki.libsdl.org/SDL2/FrontPage)

# 최신 Release
- [https://github.com/libsdl-org/SDL/releases/tag/preview-3.1.6](https://github.com/libsdl-org/SDL/releases/tag/preview-3.1.6)

# Tutorial

- SDL3
  - https://wiki.libsdl.org/SDL3/Tutorials/FrontPage

- SDL2
  - https://wiki.libsdl.org/SDL2/Tutorials

# History

- This project was forked from [Rust-sdl2](https://github.com/Rust-sdl2/rust-sdl2) and the SDL2 code migrated to SDL3 according to the [SDL2->SDL3 migration guide.](https://github.com/libsdl-org/SDL/blob/main/docs/README-migration.md)
  - If you want a library compatible with earlier versions of SDL, please see [Rust-sdl2.](https://github.com/Rust-sdl2/rust-sdl2)

# Install

- SDL3 Install
  - https://wiki.libsdl.org/SDL3/Installation

```bash
git clone https://github.com/libsdl-org/SDL
cd SDL
mkdir build
cd build
cmake -DCMAKE_BUILD_TYPE=Release ..
cmake --build . --config Release --parallel
sudo cmake --install . --config Release
```

- Doom Install필수
  - https://gitlab.com/flukejones/room4doom 

```
sudo apt install libsdl2-mixer-dev
```

- SDL2 Install
  - https://wiki.libsdl.org/SDL2/Installation

```bash
git clone https://github.com/libsdl-org/SDL.git -b SDL2
cd SDL
mkdir build
cd build
../configure
make
sudo make install
```

<hr />

# macOS(SDL2, SDL3 설치)
- [[Ep. 4] [Setup] SDL Mac (Including M1) Setup with Simple OpenGL Application | Introduction to SDL2](https://youtu.be/V6ACATpl2LQ?si=eWPOTULjG4p9pSmH)


# macOS PATH(fishshell)

- `config.fish`
  - https://www.csalmeida.com/log/how-to-install-sdl2-on-macos/

```
    # macOS SDL 2 , 3 Path
    set -gx DYLD_FRAMEWORK_PATH /Library/Frameworks
    set -gx LIBRARY_PATH /opt/homebrew/lib
```

<hr />

# 오류난거 참고

- 오류 난거 참고 하자
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
  "/home/g/utilities/SDL2-2.30.10/build/libSDL2-2.0.so.0.3000.10" to
  "/usr/local/lib/libSDL2-2.0.so.0.3000.10": Permission denied.

```

# Compile(Need help setting up a Makefile for an SDL C++ tutorial - Having trouble with linking object files)
- https://stackoverflow.com/questions/32981617/need-help-setting-up-a-makefile-for-an-sdl-c-tutorial-having-trouble-with-li

# System can't find libSDL3.so.0 at runtime?
- https://stackoverflow.com/questions/78861132/system-cant-find-libsdl3-so-0-at-runtime

- fish PATH set

```fish
# Add HomeBrew's bin directory to path so you can use HomeBrew's binaries like `starship`
# Fish uses `fish_add_path` instead of `export PATH` modify $PATH.
fish_add_path /opt/homebrew/bin/
fish_add_path "$HOME/.local/bin"
fish_add_path "$HOME/utilities/nvim-macos"
fish_add_path "$HOME/utilities/zig"
fish_add_path "$HOME/utilities/zls/zig-out/bin"
fish_add_path "$HOME/.cargo/bin"
fish_add_path "$HOME/.wasmer/bin"
fish_add_path "$HOME/.modular"
fish_add_path "$HOME/.modular/bin"
fish_add_path "$HOME/.modular/pkg/packages.modular.com_mojo/bin"
fish_add_path "$HOME/Library/Application Support/Code/User/globalStorage/fwcd.kotlin/langServerInstall/server/bin"
fish_add_path "$HOME/.surrealdb"
fish_add_path "/opt/homebrew/include/"

if status is-interactive
    # Commands to run in interactive sessions can go here

    # add Path
    function addpaths
        contains -- $argv $fish_user_paths
        or set -U fish_user_paths $fish_user_paths $argv
        echo "Updated PATH: $PATH"
    end

    # Remove path
    function removepath
        if set -l index (contains -i $argv[1] $PATH)
            set --erase --universal fish_user_paths[$index]
            echo "Updated PATH: $PATH"
        else
            echo "$argv[1] not found in PATH: $PATH"
        end

    end

    set -gx MAX_PATH $HOME/.modular/bin
    set -gx MODULAR_HOME $HOME/.modular
    set -gx MOJO_PATH $(modular config mojo.path)

    set -gx WASMER_DIR $HOME/.wasmer

    # macOS SDL 2 , 3 Path
    set -gx DYLD_FRAMEWORK_PATH /Library/Frameworks

    set -gx LIBRARY_PATH /opt/homebrew/lib
    set -gx LC_RPATH $HOME/utilities/SDL3/SDL/build

    # sdk man(java, kotlin)
    # set -gx

end

# Enable Starship prompt
starship init fish | source

# Wasmer
export WASMER_DIR="/Users/g/.wasmer"
[ -s "$WASMER_DIR/wasmer.sh" ] && source "$WASMER_DIR/wasmer.sh"
```

- c just 

```justfile
# which clang
clang_which := `which clang`

# Source and target directories
src_dir := "./src"
target_dir := "./target"

# Files
source := src_dir+"/main.c"
target := target_dir+"/main"

# Common flags
ldflags_common := "-pedantic -pthread -pedantic-errors -lm -Wall -Wextra -ggdb"
ldflags_debug := "-c -pthread -lm -Wall -Wextra -ggdb"
ldflags_emit_llvm := "-S -emit-llvm"
ldflags_assembly := "-Wall -save-temps"
ldflags_fsanitize_address := "-O1 -g -fsanitize=address -fno-omit-frame-pointer -c"
ldflags_fsanitize_object := "-g -fsanitize=address"
ldflags_fsanitize_valgrind := "-fsanitize=address -g3"
# SDL3 세팅 최적화 O2
ldflags_optimize :=  "-lSDL3 -MMD -MP -Wall -O2"
# SDL 2 세팅
# ldflags_optimize :=  "-lSDL2 -MMD -MP -Wall -O2"

# ldflags_optimize :=  "-Wall -O3 -pedantic -pthread -pedantic-errors -lm -Wextra -ggdb"

# (C)clang compile
r:
	rm -rf target
	mkdir -p target
	clang {{ldflags_common}} -o {{target}} {{source}}
	{{target}}

# (C)clang compile(Optimization)
ro:
	rm -rf target
	mkdir -p target
	clang {{ldflags_optimize}} -o {{target}} {{source}}
	{{target}}

# zig C compile
zr:
	rm -rf target
	mkdir -p target
	zig {{ldflags_common}} -o {{target}} {{source}}
	{{target}}

# clang build
b:
	rm -rf target
	mkdir -p target
	clang {{ldflags_debug}} -o {{target}} {{source}}

# clang LLVM emit-file
ll:
	rm -rf target
	mkdir -p target
	cp -rf {{src_dir}}/main.c ./
	clang {{ldflags_emit_llvm}} main.c
	mv *.ll {{target_dir}}
	clang {{ldflags_common}} -o {{target}} {{source}}
	mv *.cpp {{target_dir}}
	rm -rf *.out

# Assembly emit-file
as:
	rm -rf target
	mkdir -p target
	clang {{ldflags_assembly}} -o {{target}} {{source}}
	mv *.i {{target_dir}}
	mv *.o {{target_dir}}
	mv *.s {{target_dir}}
	mv *.bc {{target_dir}}

# clang fsanitize_address
fsan:
	rm -rf target
	mkdir -p target
	clang {{ldflags_fsanitize_address}} {{source}} -o {{target}}
	clang {{ldflags_fsanitize_object}} {{target}}
	mv *.out {{target_dir}}

# leak memory check(valgrind)
mem:
	rm -rf target
	mkdir -p target
	clang {{ldflags_fsanitize_valgrind}} {{source}} -o {{target}}
	valgrind --leak-check=full {{target}}

# object file emit-file
obj:
	rm -rf target
	mkdir -p target
	clang {{ldflags_assembly}} -o {{target}} {{source}}
	mv *.ii {{target_dir}}
	mv *.o {{target_dir}}
	mv *.s {{target_dir}}
	mv *.bc {{target_dir}}
	objdump --disassemble -S -C {{target_dir}}/main.o

# hex view
xx:
	rm -rf target
	mkdir -p target
	clang {{ldflags_fsanitize_valgrind}} {{source}} -o {{target}}
	xxd -c 16 {{target}}

# clean files
clean:
	rm -rf {{target_dir}} *.out {{src_dir}}/*.out *.bc {{src_dir}}/target/ *.dSYM {{src_dir}}/*.dSYM *.i *.o *.s

# C init
init:
	mkdir -p src
	echo '#include <stdio.h>' > src/main.c
	echo '' >> src/main.c
	echo 'int main(void) {' >> src/main.c
	echo '    printf("Hello world C lang ");' >> src/main.c
	echo '    return 0;' >> src/main.c
	echo '}' >> src/main.c

# Debugging(VSCode)
vscode:
	rm -rf .vscode
	mkdir -p .vscode
	echo '{' > .vscode/launch.json
	echo '    "version": "0.2.0",' >> .vscode/launch.json
	echo '    "configurations": [' >> .vscode/launch.json
	echo '        {' >> .vscode/launch.json
	echo '            "type": "lldb",' >> .vscode/launch.json
	echo '            "request": "launch",' >> .vscode/launch.json
	echo '            "name": "Launch",' >> .vscode/launch.json
	echo '            "program": "${workspaceFolder}/target/${fileBasenameNoExtension}",' >> .vscode/launch.json
	echo '            "args": [],' >> .vscode/launch.json
	echo '            "cwd": "${workspaceFolder}"' >> .vscode/launch.json
	echo '            // "preLaunchTask": "C/C++: clang build active file"' >> .vscode/launch.json
	echo '        },' >> .vscode/launch.json
	echo '        {' >> .vscode/launch.json
	echo '            "name": "gcc - Build and debug active file",' >> .vscode/launch.json
	echo '            "type": "cppdbg",' >> .vscode/launch.json
	echo '            "request": "launch",' >> .vscode/launch.json
	echo '            "program": "${fileDirname}/target/${fileBasenameNoExtension}",' >> .vscode/launch.json
	echo '            "args": [],' >> .vscode/launch.json
	echo '            "stopAtEntry": false,' >> .vscode/launch.json
	echo '            "cwd": "${fileDirname}",' >> .vscode/launch.json
	echo '            "environment": [],' >> .vscode/launch.json
	echo '            "externalConsole": false,' >> .vscode/launch.json
	echo '            "MIMode": "lldb"' >> .vscode/launch.json
	echo '            // "tasks": "C/C++: clang build active file"' >> .vscode/launch.json
	echo '        }' >> .vscode/launch.json
	echo '    ]' >> .vscode/launch.json
	echo '}' >> .vscode/launch.json
	echo '{' > .vscode/tasks.json
	echo '    "tasks": [' >> .vscode/tasks.json
	echo '        {' >> .vscode/tasks.json
	echo '            "type": "cppbuild",' >> .vscode/tasks.json
	echo '            "label": "C/C++: clang build active file",' >> .vscode/tasks.json
	echo '            "command": "{{clang_which}}",' >> .vscode/tasks.json
	echo '            "args": [' >> .vscode/tasks.json
	echo '                "-c",' >> .vscode/tasks.json
	echo '                "-fcolor-diagnostics",' >> .vscode/tasks.json
	echo '                "-fansi-escape-codes",' >> .vscode/tasks.json
	echo '                "-g",' >> .vscode/tasks.json
	echo '                "${file}",' >> .vscode/tasks.json
	echo '                "-o",' >> .vscode/tasks.json
	echo '                "${fileDirname}/target/${fileBasenameNoExtension}"' >> .vscode/tasks.json
	echo '            ],' >> .vscode/tasks.json
	echo '            "options": {' >> .vscode/tasks.json
	echo '                "cwd": "${fileDirname}"' >> .vscode/tasks.json
	echo '            },' >> .vscode/tasks.json
	echo '            "problemMatcher": [' >> .vscode/tasks.json
	echo '                "$gcc"' >> .vscode/tasks.json
	echo '            ],' >> .vscode/tasks.json
	echo '            "group": {' >> .vscode/tasks.json
	echo '                "kind": "build",' >> .vscode/tasks.json
	echo '                "isDefault": true' >> .vscode/tasks.json
	echo '            },' >> .vscode/tasks.json
	echo '            "detail": "Task generated by Debugger."' >> .vscode/tasks.json
	echo '        }' >> .vscode/tasks.json
	echo '    ],' >> .vscode/tasks.json
	echo '    "version": "2.0.0"' >> .vscode/tasks.json
	
```
