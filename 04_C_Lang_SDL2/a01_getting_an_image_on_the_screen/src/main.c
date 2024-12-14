#include <SDL2/SDL.h>
#include <SDL2/SDL_video.h>
#include <stdio.h>

bool init() {
  bool success = true;

  // Initialize SDL
  if (SDL_Init(SDL_INIT_VIDEO) < 0)
  {
      printf("SDL could not Initialize! SDL_Error: %s\n", SDL_GetError());
      success = false;
  }
  else
  {
      // Create window
      gWindow = SDL_CreateWindow("SDL Tutorial", SDL_WINDOWPOS_UNDEFINED, SDL_WINDOWPOS_UNDEFINED, SCREEN_WIDTH, SCREEN_HEIGHT, SDL_WINDOW_SHOWN);
  }
}
