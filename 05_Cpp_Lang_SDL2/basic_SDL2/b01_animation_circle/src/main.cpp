#include <SDL2/SDL.h>
#include <cmath>    // For sin() and cos()
#include <thread>   // For std::this_thread::sleep_for
#include <chrono>   // For std::chrono::milliseconds

// Function to draw a filled circle
void DrawCircle(SDL_Renderer* renderer, int centerX, int centerY, int radius) {
    for (int w = 0; w < radius * 2; w++) {
        for (int h = 0; h < radius * 2; h++) {
            int dx = radius - w; // Horizontal offset
            int dy = radius - h; // Vertical offset
            if ((dx * dx + dy * dy) <= (radius * radius)) {
                SDL_RenderDrawPoint(renderer, centerX + dx, centerY + dy);
            }
        }
    }
}

int main(int argc, char* argv[]) {
    // Initialize SDL
    if (SDL_Init(SDL_INIT_VIDEO) < 0) {
        SDL_Log("Failed to initialize SDL: %s", SDL_GetError());
        return -1;
    }

    // Create SDL window
    SDL_Window* window = SDL_CreateWindow(
        "Circling Animation", SDL_WINDOWPOS_CENTERED, SDL_WINDOWPOS_CENTERED, 
        800, 600, SDL_WINDOW_SHOWN
    );
    if (!window) {
        SDL_Log("Failed to create window: %s", SDL_GetError());
        SDL_Quit();
        return -1;
    }

    // Create SDL renderer
    SDL_Renderer* renderer = SDL_CreateRenderer(
        window, -1, SDL_RENDERER_ACCELERATED | SDL_RENDERER_PRESENTVSYNC
    );
    if (!renderer) {
        SDL_Log("Failed to create renderer: %s", SDL_GetError());
        SDL_DestroyWindow(window);
        SDL_Quit();
        return -1;
    }

    bool running = true;
    SDL_Event event;

    const int centerX = 400; // Central point of the circling path
    const int centerY = 300;
    const int orbitRadius = 150; // Radius of the circling path
    const int circleRadius = 20; // Radius of the animated circle
    const double speed = 0.05;   // Angular speed (radians per frame)

    double angle = 0.0; // Current angle of the circle on its orbit

    while (running) {
        while (SDL_PollEvent(&event)) {
            if (event.type == SDL_QUIT) {
                running = false;
            }
        }

        // Clear the screen (black background)
        SDL_SetRenderDrawColor(renderer, 0, 0, 0, 255);
        SDL_RenderClear(renderer);

        // Calculate the circle's position
        int circleX = centerX + static_cast<int>(orbitRadius * cos(angle));
        int circleY = centerY + static_cast<int>(orbitRadius * sin(angle));

        // Draw the animated circle (red color)
        SDL_SetRenderDrawColor(renderer, 255, 0, 0, 255);
        DrawCircle(renderer, circleX, circleY, circleRadius);

        // Draw the center point (white color)
        SDL_SetRenderDrawColor(renderer, 255, 255, 255, 255);
        DrawCircle(renderer, centerX, centerY, 5);

        // Update the screen
        SDL_RenderPresent(renderer);

        // Increment the angle
        angle += speed;
        if (angle > 2 * M_PI) { // Keep the angle within [0, 2π] for stability
            angle -= 2 * M_PI;
        }

        // Delay to control the animation speed
        std::this_thread::sleep_for(std::chrono::milliseconds(16)); // ~60 FPS
    }

    // Clean up
    SDL_DestroyRenderer(renderer);
    SDL_DestroyWindow(window);
    SDL_Quit();

    return 0;
}
