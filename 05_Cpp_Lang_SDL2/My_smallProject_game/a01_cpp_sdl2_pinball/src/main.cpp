#include <SDL2/SDL.h>
#include <iostream>

// Screen dimensions
const int SCREEN_WIDTH = 800;
const int SCREEN_HEIGHT = 600;

// Ball properties
const int BALL_SIZE = 20;
float ballX = SCREEN_WIDTH / 2.0f;
float ballY = SCREEN_HEIGHT / 2.0f;
float ballVelX = 4.0f;  // Ball velocity in X direction
float ballVelY = 4.0f;  // Ball velocity in Y direction

// Paddle properties
const int PADDLE_WIDTH = 100;
const int PADDLE_HEIGHT = 20;
const int PADDLE_Y = SCREEN_HEIGHT - 50;
float paddleX = (SCREEN_WIDTH - PADDLE_WIDTH) / 2.0f;

void handleInput(bool &running, SDL_Event &e) {
    while (SDL_PollEvent(&e)) {
        if (e.type == SDL_QUIT) {
            running = false;
        }
        if (e.type == SDL_KEYDOWN) {
            if (e.key.keysym.sym == SDLK_ESCAPE) {
                running = false;
            }
        }
    }
    const Uint8 *keystate = SDL_GetKeyboardState(NULL);
    if (keystate[SDL_SCANCODE_LEFT] && paddleX > 0) {
        paddleX -= 8.0f;  // Move paddle left
    }
    if (keystate[SDL_SCANCODE_RIGHT] && paddleX < SCREEN_WIDTH - PADDLE_WIDTH) {
        paddleX += 8.0f;  // Move paddle right
    }
}

void update() {
    // Move the ball
    ballX += ballVelX;
    ballY += ballVelY;

    // Ball collision with walls
    if (ballX <= 0 || ballX + BALL_SIZE >= SCREEN_WIDTH) {
        ballVelX = -ballVelX; // Reverse X direction
    }
    if (ballY <= 0) {
        ballVelY = -ballVelY; // Reverse Y direction
    }

    // Ball collision with paddle
    if (ballY + BALL_SIZE >= PADDLE_Y &&
        ballX + BALL_SIZE >= paddleX &&
        ballX <= paddleX + PADDLE_WIDTH) {
        ballVelY = -ballVelY; // Bounce off the paddle
        ballY = PADDLE_Y - BALL_SIZE; // Adjust position to avoid sticking
    }

    // Ball falls off screen (game over logic can be added here)
    if (ballY > SCREEN_HEIGHT) {
        std::cout << "Game Over!" << std::endl;
        ballX = SCREEN_WIDTH / 2.0f;
        ballY = SCREEN_HEIGHT / 2.0f;
        ballVelX = 4.0f;
        ballVelY = 4.0f;
    }
}

void render(SDL_Renderer *renderer) {
    // Clear screen
    SDL_SetRenderDrawColor(renderer, 0, 0, 0, 255); // Black background
    SDL_RenderClear(renderer);

    // Draw ball
    SDL_SetRenderDrawColor(renderer, 255, 0, 0, 255); // Red ball
    SDL_Rect ballRect = {static_cast<int>(ballX), static_cast<int>(ballY), BALL_SIZE, BALL_SIZE};
    SDL_RenderFillRect(renderer, &ballRect);

    // Draw paddle
    SDL_SetRenderDrawColor(renderer, 0, 255, 0, 255); // Green paddle
    SDL_Rect paddleRect = {static_cast<int>(paddleX), PADDLE_Y, PADDLE_WIDTH, PADDLE_HEIGHT};
    SDL_RenderFillRect(renderer, &paddleRect);

    // Present the updated screen
    SDL_RenderPresent(renderer);
}

int main(int argc, char *argv[]) {
    if (SDL_Init(SDL_INIT_VIDEO) < 0) {
        std::cerr << "SDL could not initialize! SDL_Error: " << SDL_GetError() << std::endl;
        return 1;
    }

    SDL_Window *window = SDL_CreateWindow(
        "Simple Pinball",
        SDL_WINDOWPOS_CENTERED, SDL_WINDOWPOS_CENTERED,
        SCREEN_WIDTH, SCREEN_HEIGHT,
        SDL_WINDOW_SHOWN
    );

    if (!window) {
        std::cerr << "Window could not be created! SDL_Error: " << SDL_GetError() << std::endl;
        SDL_Quit();
        return 1;
    }

    SDL_Renderer *renderer = SDL_CreateRenderer(window, -1, SDL_RENDERER_ACCELERATED);
    if (!renderer) {
        std::cerr << "Renderer could not be created! SDL_Error: " << SDL_GetError() << std::endl;
        SDL_DestroyWindow(window);
        SDL_Quit();
        return 1;
    }

    bool running = true;
    SDL_Event e;

    // Main game loop
    while (running) {
        handleInput(running, e); // Handle input
        update();               // Update game logic
        render(renderer);       // Render everything

        SDL_Delay(16); // Delay to cap frame rate (~60 FPS)
    }

    // Clean up
    SDL_DestroyRenderer(renderer);
    SDL_DestroyWindow(window);
    SDL_Quit();

    return 0;
}
