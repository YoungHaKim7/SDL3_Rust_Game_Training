#include <SDL3/SDL_render.h>
#include <stdio.h>
#include <SDL3/SDL.h>

#define WIDTH 800
#define HEIGHT 600


int main(void) {
    SDL_Init(SDL_INIT_VIDEO);
    SDL_Window *window = NULL;
    SDL_Renderer *renderer = NULL;
    SDL_CreateWindowAndRenderer("Hello SDL3",WIDTH,HEIGHT,0,&window ,&renderer);

    bool quit = false;
    while (!quit) {
        SDL_Event event;
        while (SDL_PollEvent(&event)) {
            if (event.type == SDL_EVENT_QUIT) quit = true;
        }

        SDL_RenderClear(renderer);
        SDL_SetRenderDrawColor(renderer, 0x18, 0x18, 0x18, 0xFF);

        SDL_RenderPresent(renderer);
    }

    SDL_Quit();
}
