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
    SDL_GPUDevice* device = SDL_CreateGPUDevice(SDL_GPU_SHADERFORMAT_SPIRV, true, NULL);

    if (!device) {
        fprintf(stderr, "ERROR: Could not create GPU device: %s\n", SDL_GetError);
        return 1;
    }
    printf("OK: Create device with driver %s\n", SDL_GetGPUDeviceDriver(device));

    bool quit = false;
    while (!quit) {
        SDL_Event event;
        while (SDL_PollEvent(&event)) {
            if (event.type == SDL_EVENT_QUIT) quit = true;
        }

        // SDL_SetRenderDrawColor(renderer, 0xff, 0x0, 0x0, 0xFF);
        // SDL_RenderClear(renderer);

        SDL_FRect rect = {
            .w = 300,
            .h = 250,
        };
        rect.x = (WIDTH - rect.w)/2.0f,
        rect.y = (HEIGHT - rect.h)/2.0f,
        SDL_SetRenderDrawColor(renderer, 0xff, 0x0, 0x0, 0xFF);
        SDL_RenderFillRect(renderer,&rect);

        SDL_RenderPresent(renderer);
    }

    SDL_Quit();
    return 0;
}
