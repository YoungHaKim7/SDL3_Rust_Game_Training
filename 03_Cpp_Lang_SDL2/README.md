# MikeShah(C++개발자 배울게 많다.) 
- https://github.com/MikeShah/SDL2_Tutorials

- SDL2 Tutorials
  - https://wiki.libsdl.org/SDL2/Tutorials

<hr />

# link

- [justfile_macOS전용](#macosmacos전용justfile)

<hr />

# macOS설치
- [Ep. 4 Setup/ SDL Mac (Including M1) Setup with Simple OpenGL Application | Introduction to SDL2](https://youtu.be/V6ACATpl2LQ?si=ZAfU_EoyvQn0h23c)

- [모아보기SDL2 Simple Directmedia Layer | Mike Shah](https://youtube.com/playlist?list=PLvv0ScY6vfd-p1gSnbQhY7vMe2rng0IL0&si=8K3WhN7AX2ooxGKo)

<hr />

# macOS(SDL2 설치하기)
- https://stackoverflow.com/questions/28016258/using-homebrew-installed-sdl2-with-xcode

# justfile(linux std=C++17 version)

```justfile
# which g++
gpp_which := `which g++`
# sdl2_config := `sdl2-config --libs`
# sdl2_config2 := `sdl2-config --cflags`

# Source and target directories
src_dir := "./src"
target_dir := "./target"

# Files
source := src_dir+"/main.cpp"
target := target_dir+"/main"

# Common flags
ldflags_common := "-std=c++2b -pedantic -pthread -pedantic-errors -lm -Wall -Wextra -ggdb"
ldflags_debug := "-c -std=c++2b -pthread -lm -Wall -Wextra -ggdb"
ldflags_emit_llvm := "-S -emit-llvm"
ldflags_assembly := "-Wall -save-temps"
ldflags_fsanitize_address := "-O1 -g -fsanitize=address -fno-omit-frame-pointer -c"
ldflags_fsanitize_object := "-g -fsanitize=address"
ldflags_fsanitize_valgrind := "-fsanitize=address -g3 -std=c++2b"
# ldflags_optimize :=  "-std=c++2b -Wall -O2 -pedantic -pthread -pedantic-errors -lm -Wextra -ggdb"
# SDL3 세팅 최적화 O2
# ldflags_optimize :=  "-lSDL3 -MMD -MP -Wall -O2"
# SDL 2 세팅
# ldflags_optimize03 :=  "-std=c++17 -lSDL2 -MMD -MP -Wall -O2 -I/usr/include/SDL2"
ldflags_optimize04 :=  "-std=c++17 -Wall -O2 "
ldlflag := "-lSDL2"
# ldflags_optimize03 :=  "-std=c++2b -Wall -O3 -pedantic -pthread -pedantic-errors -lm -Wextra -ggdb"
# macOS
# ldflags_macos := "-Wall -O2 " + sdl2_config + " "+ sdl2_config2

# g++ compile
r:
	rm -rf target
	mkdir -p target
	g++ {{ldflags_common}} -o {{target}} {{source}}
	{{target}}

# (C++)g++ compile(Optimization "-O2")
ro:
	rm -rf target
	mkdir -p target
	g++ {{ldflags_optimize04}} -o {{target}} {{source}} {{ldlflag}}
	{{target}}

# romac:
# 	rm -rf target
# 	mkdir -p target
# 	clang++ {{ldflags_macos}} -o {{target}} {{source}}
# 	{{target}}

# (C++)g++ compile(Optimization "-O3")
# ro3:
# 	rm -rf target
# 	mkdir -p target
# 	g++ {{ldflags_optimize03}} -o {{target}} {{source}}
# 	{{target}}


# zig c++ compile
zr:
	rm -rf target
	mkdir -p target
	zig c++ {{ldflags_common}} -o {{target}} {{source}}
	{{target}}

# g++ build
b:
	rm -rf target
	mkdir -p target
	g++ {{ldflags_debug}} -o {{target}} {{source}}

# clang++ LLVM emit-file
ll:
	rm -rf target
	mkdir -p target
	cp -rf {{src_dir}}/main.cpp ./
	clang++ {{ldflags_emit_llvm}} main.cpp
	mv *.ll {{target_dir}}
	clang++ {{ldflags_common}} -o {{target}} {{source}}
	mv *.cpp {{target_dir}}
	rm -rf *.out

# Assembly emit-file
as:
	rm -rf target
	mkdir -p target
	g++ {{ldflags_assembly}} -o {{target}} {{source}}
	mv *.ii {{target_dir}}
	mv *.o {{target_dir}}
	mv *.s {{target_dir}}
	mv *.bc {{target_dir}}

# clang++ fsanitize_address
fsan:
	rm -rf target
	mkdir -p target
	clang++ {{ldflags_fsanitize_address}} {{source}} -o {{target}}
	clang++ {{ldflags_fsanitize_object}} {{target}}
	mv *.out {{target_dir}}

# leak memory check(valgrind)
mem:
	rm -rf target
	mkdir -p target
	g++ {{ldflags_fsanitize_valgrind}} {{source}} -o {{target}}
	valgrind --leak-check=full {{target}}

# object file emit-file
obj:
	rm -rf target
	mkdir -p target
	g++ {{ldflags_assembly}} -o {{target}} {{source}}
	mv *.ii {{target_dir}}
	mv *.o {{target_dir}}
	mv *.s {{target_dir}}
	mv *.bc {{target_dir}}
	objdump --disassemble -S -C {{target_dir}}/main.o

# hex view
xx:
	rm -rf target
	mkdir -p target
	g++ {{ldflags_fsanitize_valgrind}} {{source}} -o {{target}}
	xxd -c 16 {{target}}

# clean files
clean:
	rm -rf {{target_dir}} *.out {{src_dir}}/*.out *.bc {{src_dir}}/target/ *.dSYM {{src_dir}}/*.dSYM *.i *.o *.s

# C++ init
init:
	mkdir -p src
	echo '#include <iostream>' > src/main.cpp
	echo '' >> src/main.cpp
	echo 'int main() {' >> src/main.cpp
	echo '    std::cout << "Hello C++" << std::endl;' >> src/main.cpp
	echo '    return 0;' >> src/main.cpp
	echo '}' >> src/main.cpp

# C++ init(int argc, char **argv)
init2:
	mkdir -p src
	echo '#include <iostream>' > src/main.cpp
	echo '' >> src/main.cpp
	echo 'int main(int argc, char **argv) {' >> src/main.cpp
	echo '    std::cout << "Hello C++" << std::endl;' >> src/main.cpp
	echo '    return 0;' >> src/main.cpp
	echo '}' >> src/main.cpp

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
	echo '            "cwd": "${workspaceFolder}",' >> .vscode/launch.json
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
	echo '            "MIMode": "lldb",' >> .vscode/launch.json
	echo '            // "tasks": "C/C++: clang build active file"' >> .vscode/launch.json
	echo '        }' >> .vscode/launch.json
	echo '    ]' >> .vscode/launch.json
	echo '}' >> .vscode/launch.json
	echo '{' > .vscode/tasks.json
	echo '    "tasks": [' >> .vscode/tasks.json
	echo '        {' >> .vscode/tasks.json
	echo '            "type": "cppbuild",' >> .vscode/tasks.json
	echo '            "label": "C/C++: clang build active file",' >> .vscode/tasks.json
	echo '            "command": "{{gpp_which}}",' >> .vscode/tasks.json
	echo '            "args": [' >> .vscode/tasks.json
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
	echo '}' >> .vscode/tasks.json
```

# justfile(SDL2)

```justfile
# which g++ 
gpp_which := `which g++`

# Source and target directories
src_dir := "./src"
target_dir := "./target"

# Files
source := src_dir+"/main.cpp"
target := target_dir+"/main"

# Common flags
ldflags_common := "-std=c++2b -pedantic -pthread -pedantic-errors -lm -Wall -Wextra -ggdb"
ldflags_debug := "-c -std=c++2b -pthread -lm -Wall -Wextra -ggdb"
ldflags_emit_llvm := "-S -emit-llvm"
ldflags_assembly := "-Wall -save-temps"
ldflags_fsanitize_address := "-O1 -g -fsanitize=address -fno-omit-frame-pointer -c"
ldflags_fsanitize_object := "-g -fsanitize=address"
ldflags_fsanitize_valgrind := "-fsanitize=address -g3 -std=c++2b"
# ldflags_optimize :=  "-std=c++2b -Wall -O2 -pedantic -pthread -pedantic-errors -lm -Wextra -ggdb"
# SDL3 세팅 최적화 O2
# ldflags_optimize :=  "-lSDL3 -MMD -MP -Wall -O2"
# SDL 2 세팅
ldflags_optimize :=  "-lSDL2 -MMD -MP -Wall -O2"
# ldflags_optimize03 :=  "-std=c++2b -Wall -O3 -pedantic -pthread -pedantic-errors -lm -Wextra -ggdb"

# g++ compile
r:
	rm -rf target
	mkdir -p target
	g++ {{ldflags_common}} -o {{target}} {{source}}
	{{target}}

# (C++)g++ compile(Optimization "-O2")
ro:
	rm -rf target
	mkdir -p target
	g++ {{ldflags_optimize}} -o {{target}} {{source}}
	{{target}}


# (C++)g++ compile(Optimization "-O3")
ro3:
	rm -rf target
	mkdir -p target
	g++ {{ldflags_optimize03}} -o {{target}} {{source}}
	{{target}}


# zig c++ compile
zr:
	rm -rf target
	mkdir -p target
	zig c++ {{ldflags_common}} -o {{target}} {{source}}
	{{target}}

# g++ build
b:
	rm -rf target
	mkdir -p target
	g++ {{ldflags_debug}} -o {{target}} {{source}}

# clang++ LLVM emit-file
ll:
	rm -rf target
	mkdir -p target
	cp -rf {{src_dir}}/main.cpp ./
	clang++ {{ldflags_emit_llvm}} main.cpp
	mv *.ll {{target_dir}}
	clang++ {{ldflags_common}} -o {{target}} {{source}}
	mv *.cpp {{target_dir}}
	rm -rf *.out

# Assembly emit-file
as:
	rm -rf target
	mkdir -p target
	g++ {{ldflags_assembly}} -o {{target}} {{source}}
	mv *.ii {{target_dir}}
	mv *.o {{target_dir}}
	mv *.s {{target_dir}}
	mv *.bc {{target_dir}}

# clang++ fsanitize_address
fsan:
	rm -rf target
	mkdir -p target
	clang++ {{ldflags_fsanitize_address}} {{source}} -o {{target}}
	clang++ {{ldflags_fsanitize_object}} {{target}}
	mv *.out {{target_dir}}

# leak memory check(valgrind)
mem:
	rm -rf target
	mkdir -p target
	g++ {{ldflags_fsanitize_valgrind}} {{source}} -o {{target}}
	valgrind --leak-check=full {{target}}

# object file emit-file
obj:
	rm -rf target
	mkdir -p target
	g++ {{ldflags_assembly}} -o {{target}} {{source}}
	mv *.ii {{target_dir}}
	mv *.o {{target_dir}}
	mv *.s {{target_dir}}
	mv *.bc {{target_dir}}
	objdump --disassemble -S -C {{target_dir}}/main.o

# hex view
xx:
	rm -rf target
	mkdir -p target
	g++ {{ldflags_fsanitize_valgrind}} {{source}} -o {{target}}
	xxd -c 16 {{target}}

# clean files
clean:
	rm -rf {{target_dir}} *.out {{src_dir}}/*.out *.bc {{src_dir}}/target/ *.dSYM {{src_dir}}/*.dSYM *.i *.o *.s

# C++ init
init:
	mkdir -p src
	echo '#include <iostream>' > src/main.cpp
	echo '' >> src/main.cpp
	echo 'int main() {' >> src/main.cpp
	echo '    std::cout << "Hello C++" << std::endl;' >> src/main.cpp
	echo '    return 0;' >> src/main.cpp
	echo '}' >> src/main.cpp

# C++ init(int argc, char **argv)
init2:
	mkdir -p src
	echo '#include <iostream>' > src/main.cpp
	echo '' >> src/main.cpp
	echo 'int main(int argc, char **argv) {' >> src/main.cpp
	echo '    std::cout << "Hello C++" << std::endl;' >> src/main.cpp
	echo '    return 0;' >> src/main.cpp
	echo '}' >> src/main.cpp

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
	echo '            "cwd": "${workspaceFolder}",' >> .vscode/launch.json
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
	echo '            "MIMode": "lldb",' >> .vscode/launch.json
	echo '            // "tasks": "C/C++: clang build active file"' >> .vscode/launch.json
	echo '        }' >> .vscode/launch.json
	echo '    ]' >> .vscode/launch.json
	echo '}' >> .vscode/launch.json
	echo '{' > .vscode/tasks.json
	echo '    "tasks": [' >> .vscode/tasks.json
	echo '        {' >> .vscode/tasks.json
	echo '            "type": "cppbuild",' >> .vscode/tasks.json
	echo '            "label": "C/C++: clang build active file",' >> .vscode/tasks.json
	echo '            "command": "{{gpp_which}}",' >> .vscode/tasks.json
	echo '            "args": [' >> .vscode/tasks.json
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
	echo '}' >> .vscode/tasks.json
```


# macOS(macOS전용justfile)


```justfile
# which g++ 
gpp_which := `which g++`
sdl2_config := `sdl2-config --libs`
sdl2_config2 := `sdl2-config --cflags`

# Source and target directories
src_dir := "./src"
target_dir := "./target"

# Files
source := src_dir+"/main.cpp"
target := target_dir+"/main"

# Common flags
ldflags_common := "-std=c++2b -pedantic -pthread -pedantic-errors -lm -Wall -Wextra -ggdb"
ldflags_debug := "-c -std=c++2b -pthread -lm -Wall -Wextra -ggdb"
ldflags_emit_llvm := "-S -emit-llvm"
ldflags_assembly := "-Wall -save-temps"
ldflags_fsanitize_address := "-O1 -g -fsanitize=address -fno-omit-frame-pointer -c"
ldflags_fsanitize_object := "-g -fsanitize=address"
ldflags_fsanitize_valgrind := "-fsanitize=address -g3 -std=c++2b"
# ldflags_optimize :=  "-std=c++2b -Wall -O2 -pedantic -pthread -pedantic-errors -lm -Wextra -ggdb"
# SDL3 세팅 최적화 O2
# ldflags_optimize :=  "-lSDL3 -MMD -MP -Wall -O2"
# SDL 2 세팅
ldflags_optimize03 :=  "-lSDL2 -MMD -MP -Wall -O2 -I/usr/include/SDL2"
# ldflags_optimize03 :=  "-std=c++2b -Wall -O3 -pedantic -pthread -pedantic-errors -lm -Wextra -ggdb"
# macOS
ldflags_macos := "-Wall -O2 " + sdl2_config + " "+ sdl2_config2

# g++ compile
r:
	rm -rf target
	mkdir -p target
	g++ {{ldflags_common}} -o {{target}} {{source}}
	{{target}}

# (C++)g++ compile(Optimization "-O2")
ro:
	rm -rf target
	mkdir -p target
	g++ {{ldflags_optimize03}} -o {{target}} {{source}}
	{{target}}

romac:
	rm -rf target
	mkdir -p target
	clang++ {{ldflags_macos}} -o {{target}} {{source}}
	{{target}}

# (C++)g++ compile(Optimization "-O3")
ro3:
	rm -rf target
	mkdir -p target
	g++ {{ldflags_optimize03}} -o {{target}} {{source}}
	{{target}}


# zig c++ compile
zr:
	rm -rf target
	mkdir -p target
	zig c++ {{ldflags_common}} -o {{target}} {{source}}
	{{target}}

# g++ build
b:
	rm -rf target
	mkdir -p target
	g++ {{ldflags_debug}} -o {{target}} {{source}}

# clang++ LLVM emit-file
ll:
	rm -rf target
	mkdir -p target
	cp -rf {{src_dir}}/main.cpp ./
	clang++ {{ldflags_emit_llvm}} main.cpp
	mv *.ll {{target_dir}}
	clang++ {{ldflags_common}} -o {{target}} {{source}}
	mv *.cpp {{target_dir}}
	rm -rf *.out

# Assembly emit-file
as:
	rm -rf target
	mkdir -p target
	g++ {{ldflags_assembly}} -o {{target}} {{source}}
	mv *.ii {{target_dir}}
	mv *.o {{target_dir}}
	mv *.s {{target_dir}}
	mv *.bc {{target_dir}}

# clang++ fsanitize_address
fsan:
	rm -rf target
	mkdir -p target
	clang++ {{ldflags_fsanitize_address}} {{source}} -o {{target}}
	clang++ {{ldflags_fsanitize_object}} {{target}}
	mv *.out {{target_dir}}

# leak memory check(valgrind)
mem:
	rm -rf target
	mkdir -p target
	g++ {{ldflags_fsanitize_valgrind}} {{source}} -o {{target}}
	valgrind --leak-check=full {{target}}

# object file emit-file
obj:
	rm -rf target
	mkdir -p target
	g++ {{ldflags_assembly}} -o {{target}} {{source}}
	mv *.ii {{target_dir}}
	mv *.o {{target_dir}}
	mv *.s {{target_dir}}
	mv *.bc {{target_dir}}
	objdump --disassemble -S -C {{target_dir}}/main.o

# hex view
xx:
	rm -rf target
	mkdir -p target
	g++ {{ldflags_fsanitize_valgrind}} {{source}} -o {{target}}
	xxd -c 16 {{target}}

# clean files
clean:
	rm -rf {{target_dir}} *.out {{src_dir}}/*.out *.bc {{src_dir}}/target/ *.dSYM {{src_dir}}/*.dSYM *.i *.o *.s

# C++ init
init:
	mkdir -p src
	echo '#include <iostream>' > src/main.cpp
	echo '' >> src/main.cpp
	echo 'int main() {' >> src/main.cpp
	echo '    std::cout << "Hello C++" << std::endl;' >> src/main.cpp
	echo '    return 0;' >> src/main.cpp
	echo '}' >> src/main.cpp

# C++ init(int argc, char **argv)
init2:
	mkdir -p src
	echo '#include <iostream>' > src/main.cpp
	echo '' >> src/main.cpp
	echo 'int main(int argc, char **argv) {' >> src/main.cpp
	echo '    std::cout << "Hello C++" << std::endl;' >> src/main.cpp
	echo '    return 0;' >> src/main.cpp
	echo '}' >> src/main.cpp

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
	echo '            "cwd": "${workspaceFolder}",' >> .vscode/launch.json
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
	echo '            "MIMode": "lldb",' >> .vscode/launch.json
	echo '            // "tasks": "C/C++: clang build active file"' >> .vscode/launch.json
	echo '        }' >> .vscode/launch.json
	echo '    ]' >> .vscode/launch.json
	echo '}' >> .vscode/launch.json
	echo '{' > .vscode/tasks.json
	echo '    "tasks": [' >> .vscode/tasks.json
	echo '        {' >> .vscode/tasks.json
	echo '            "type": "cppbuild",' >> .vscode/tasks.json
	echo '            "label": "C/C++: clang build active file",' >> .vscode/tasks.json
	echo '            "command": "{{gpp_which}}",' >> .vscode/tasks.json
	echo '            "args": [' >> .vscode/tasks.json
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
	echo '}' >> .vscode/tasks.json

```