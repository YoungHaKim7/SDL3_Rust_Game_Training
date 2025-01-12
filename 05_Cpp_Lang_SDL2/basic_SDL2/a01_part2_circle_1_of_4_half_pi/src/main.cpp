#include "SDL2/SDL_render.h"
#include <SDL2/SDL.h>
#include <cmath> // For sin() and cos()

// Function to draw a pixel-based circle
void DrawCircle(SDL_Renderer* renderer, int centerX, int centerY, int radius)
{
    // Use the Midpoint Circle Algorithm or simply calculate points
    for (int w = 0; w < radius * 2; w++) {
        for (int h = 0; h < radius * 2; h++) {
            if (w * w + h * h <= radius * radius) {
                SDL_RenderDrawPoint(renderer, centerX + w, centerY + h);
            }
        }
    }
}

int main(int argc, char* argv[])
{
    // Initialize SDL
    if (SDL_Init(SDL_INIT_VIDEO) < 0) {
        SDL_Log("Failed to initialize SDL: %s", SDL_GetError());
        return -1;
    }

    // Create SDL window
    SDL_Window* window
        = SDL_CreateWindow("Circle Drawing", SDL_WINDOWPOS_CENTERED,
            SDL_WINDOWPOS_CENTERED, 800, 600, SDL_WINDOW_SHOWN);
    if (!window) {
        SDL_Log("Failed to create window: %s", SDL_GetError());
        SDL_Quit();
        return -1;
    }

    // Create SDL renderer
    SDL_Renderer* renderer = SDL_CreateRenderer(
        window, -1, SDL_RENDERER_ACCELERATED | SDL_RENDERER_PRESENTVSYNC);
    if (!renderer) {
        SDL_Log("Failed to create renderer: %s", SDL_GetError());
        SDL_DestroyWindow(window);
        SDL_Quit();
        return -1;
    }

    bool running = true;
    SDL_Event event;

    while (running) {
        while (SDL_PollEvent(&event)) {
            if (event.type == SDL_QUIT) {
                running = false;
            }
        }

        // Clear the screen
        SDL_SetRenderDrawColor(renderer, 0, 0, 0, 255);
        SDL_RenderClear(renderer);

        // Set the drawing color (white)
        SDL_SetRenderDrawColor(renderer, 255, 255, 255, 255);

        // Draw a circle
        DrawCircle(renderer, 400, 300, 100);

        // Present the updated renderer
        SDL_RenderPresent(renderer);
    }

    // Clean up
    SDL_DestroyRenderer(renderer);
    SDL_DestroyWindow(window);
    SDL_Quit();

    return 0;
}
