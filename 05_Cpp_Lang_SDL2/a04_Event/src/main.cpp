#include <iostream>

#include <SDL2/SDL.h>

int main(int argc, char *argv[]) {

  SDL_Window *window = nullptr;

  if (SDL_Init(SDL_INIT_VIDEO) < 0) {
    std::cout << "SDL could not be inittialized: " << SDL_GetError();
  } else {
    std::cout << "SDL video system is ready to go\n";
  }

  window =
      SDL_CreateWindow("C++ SDL2 Window", 0, 2500, 640, 480, SDL_WINDOW_SHOWN);

  // Infinite loop for out application
  bool gameIsRunning = true;
  while (gameIsRunning) {
    SDL_Event event;

    // Start our event loop
    while (SDL_PollEvent(&event)) {
      if (event.type == SDL_QUIT) {
        gameIsRunning = false;
      }

      if (event.type == SDL_MOUSEMOTION){
          std::cout << "mouse has been moved\n";
      }

      if(event.type == SDL_KEYDOWN) {
          std::cout << "a key has been pressed \n";
          if(event.key.keysym.sym == SDLK_0){
              std::cout << "0 was pressed\n";
          }else {
              std::cout << "0 was not pressed\n";
          }
      }
    }
  }

  SDL_DestroyWindow(window);

  SDL_Delay(3000);
  SDL_Delay(3000);
  SDL_Delay(3000);
  SDL_Delay(3000);
  SDL_Delay(3000);

  SDL_Quit();

  return 0;
}
