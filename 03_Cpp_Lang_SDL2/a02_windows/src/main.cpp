#include <iostream>

#include <SDL2/SDL.h>

int main(int argc, char* argv[]) {


    SDL_Window* window = nullptr;

    if(SDL_Init(SDL_INIT_VIDEO) < 0) {
        std::cout << "SDL could not be inittialized: " << SDL_GetError();
    } else {
        std::cout << "SDL video system is ready to go\n";
    }

    window = SDL_CreateWindow("C++ SDL2 Window",
            0,
            2500,
            640,
            480,
            SDL_WINDOW_SHOWN);

    SDL_Delay(3000);

    SDL_DestroyWindow(window);

    SDL_Quit();

    return 0;
}