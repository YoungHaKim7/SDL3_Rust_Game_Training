#include <SDL2/SDL.h>

void DrawTriangle(
    SDL_Renderer* renderer, int x1, int y1, int x2, int y2, int x3, int y3)
{
    // Draw the three edges of the triangle
    SDL_RenderDrawLine(renderer, x1, y1, x2, y2); // Edge 1
    SDL_RenderDrawLine(renderer, x2, y2, x3, y3); // Edge 2
    SDL_RenderDrawLine(renderer, x3, y3, x1, y1); // Edge 3
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
        = SDL_CreateWindow("Triangle Drawing", SDL_WINDOWPOS_CENTERED,
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

        // Draw a triangle(x2를 조정하면 위에 꼭지점이 오른쪽 왼쪽으로 간다.)
        DrawTriangle(renderer, 200, 400, 400, 100, 600, 400);

        // Present the updated renderer
        SDL_RenderPresent(renderer);
    }

    // Clean up
    SDL_DestroyRenderer(renderer);
    SDL_DestroyWindow(window);
    SDL_Quit();

    return 0;
}
