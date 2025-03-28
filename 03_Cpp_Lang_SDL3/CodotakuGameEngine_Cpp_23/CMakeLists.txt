cmake_minimum_required(VERSION 3.30)
project(CodotakuGameEngine)

set(CMAKE_RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/bin)

# CPM
file(
        DOWNLOAD
        https://github.com/cpm-cmake/CPM.cmake/releases/download/v0.38.3/CPM.cmake
        ${CMAKE_CURRENT_BINARY_DIR}/cmake/CPM.cmake
        EXPECTED_HASH SHA256=cc155ce02e7945e7b8967ddfaff0b050e958a723ef7aad3766d368940cb15494
)
include(${CMAKE_CURRENT_BINARY_DIR}/cmake/CPM.cmake)

## Dependencies
set(LIBS)

# SDL
CPMAddPackage("gh:libsdl-org/SDL#a4cd17c")
list(APPEND LIBS SDL3::SDL3)

# SDL_image
set(SDLIMAGE_AVIF OFF CACHE BOOL "" FORCE) # Requires Perl and Nasm for libaom (AVIF image format)
CPMAddPackage("gh:libsdl-org/SDL_image#1ca9bd8")
list(APPEND LIBS SDL3_image::SDL3_image)

# glm
CPMAddPackage("gh:g-truc/glm#3c18b0f")
list(APPEND LIBS glm::glm-header-only)

# assimp
CPMAddPackage("gh:assimp/assimp#5fa7b8c")
list(APPEND LIBS assimp::assimp)

## Executables
add_executable(${PROJECT_NAME} src/main.cpp)
target_compile_features(${PROJECT_NAME} PRIVATE cxx_std_23)
target_link_libraries(${PROJECT_NAME} PRIVATE ${LIBS})

# Copy assets
file(COPY Content DESTINATION ${CMAKE_RUNTIME_OUTPUT_DIRECTORY})
