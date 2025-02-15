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

        SDL_FRect rect = {
            .w = 100,
            .h = 100,
        };
        rect.x = (WIDTH - rect.w)/2.0f,
        rect.y = (HEIGHT - rect.h)/2.0f,
        SDL_SetRenderDrawColor(renderer, 0xFF, 0x0, 0x0, 0xFF);
        SDL_RenderFillRect(renderer,&rect);

        SDL_RenderPresent(renderer);
    }

    SDL_Quit();
    return 0;
}
