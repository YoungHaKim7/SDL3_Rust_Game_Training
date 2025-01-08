#include <SDL2/SDL.h>
#include <cmath> // For mathematical functions if needed

// Function to draw symmetric points
void DrawCirclePoints(SDL_Renderer* renderer, int centerX, int centerY, int x, int y) {
    // Draw the 8 symmetrical points of the circle
    SDL_RenderDrawPoint(renderer, centerX + x, centerY + y); // Octant 1
    SDL_RenderDrawPoint(renderer, centerX - x, centerY + y); // Octant 2
    SDL_RenderDrawPoint(renderer, centerX + x, centerY - y); // Octant 3
    SDL_RenderDrawPoint(renderer, centerX - x, centerY - y); // Octant 4
    SDL_RenderDrawPoint(renderer, centerX + y, centerY + x); // Octant 5
    SDL_RenderDrawPoint(renderer, centerX - y, centerY + x); // Octant 6
    SDL_RenderDrawPoint(renderer, centerX + y, centerY - x); // Octant 7
    SDL_RenderDrawPoint(renderer, centerX - y, centerY - x); // Octant 8
}

// Function to draw a circle using the Midpoint Circle Algorithm
void DrawCircle(SDL_Renderer* renderer, int centerX, int centerY, int radius) {
    int x = 0;
    int y = radius;
    int d = 1 - radius; // Initial decision parameter

    // Draw the initial points
    DrawCirclePoints(renderer, centerX, centerY, x, y);

    while (x < y) {
        if (d < 0) {
            // Choose the pixel to the right (E)
            d = d + 2 * x + 3;
        } else {
            // Choose the pixel diagonally (SE)
            d = d + 2 * (x - y) + 5;
            y--;
        }
        x++;
        // Draw the new points
        DrawCirclePoints(renderer, centerX, centerY, x, y);
    }
}

int main(int argc, char* argv[]) {
    // Initialize SDL
    if (SDL_Init(SDL_INIT_VIDEO) < 0) {
        SDL_Log("Failed to initialize SDL: %s", SDL_GetError());
        return -1;
    }

    // Create SDL window
    SDL_Window* window = SDL_CreateWindow("Midpoint Circle Algorithm", SDL_WINDOWPOS_CENTERED, SDL_WINDOWPOS_CENTERED, 800, 600, SDL_WINDOW_SHOWN);
    if (!window) {
        SDL_Log("Failed to create window: %s", SDL_GetError());
        SDL_Quit();
        return -1;
    }

    // Create SDL renderer
    SDL_Renderer* renderer = SDL_CreateRenderer(window, -1, SDL_RENDERER_ACCELERATED | SDL_RENDERER_PRESENTVSYNC);
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

        // Draw a circle at the center of the screen with radius 100
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
