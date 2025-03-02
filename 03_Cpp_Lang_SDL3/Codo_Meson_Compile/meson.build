project(
  'my_init', ['c','cpp'],
  version : '0.1',
  default_options : [
    'buildtype=debugoptimized',
    'cpp_std=c++2a',
    'warning_level=3',
  ])

# Add Assimp dependency
# assimp_dep = dependency('assimp', 
#   required: false,
#   fallback: ['assimp', 'assimp_dep'],
#   default_options: [
#     'build_tools=false',
#     'build_samples=false',
#     'build_tests=false'
#   ]
# )


compiler = meson.get_compiler('cpp')
sdl3Dep = dependency('sdl3', required: false)
glmDep = dependency('glm')
assimpDep = dependency('assimp')
threadDep = dependency('threads')
XrandrDep = dependency('Xrandr', required: false)

libsPath = '/usr/local/lib'
includePath = 'include'

if not sdl3Dep.found()
  sdl3Dep = compiler.find_library('sdl3', dirs: libsPath)
endif
if not XrandrDep.found()
  XrandrDep = compiler.find_library('Xrandr', dirs: libsPath)
endif
if not assimpDep.found()
  XrandrDep = compiler.find_library('assimp', dirs: includePath)
endif

my_project_name = meson.current_source_dir().split('/').get(-1)

src_files = [
  'src/main.cpp',
]

#  meson.project_name(),   이렇게 하면됨 (겁나 간단함)

inc = include_directories('include')
executable(my_project_name, src_files,
          include_directories : inc,
          dependencies: [threadDep,XrandrDep,
                        glmDep,
                        assimp_dep, sdl3Dep])
