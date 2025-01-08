#include <SDL2/SDL.h>
#include <cmath>
#include <vector>
#include <algorithm>

// Vector3 class to represent 3D points and directions
struct Vector3 {
    float x, y, z;

    Vector3 operator+(const Vector3& v) const { return {x + v.x, y + v.y, z + v.z}; }
    Vector3 operator-(const Vector3& v) const { return {x - v.x, y - v.y, z - v.z}; }
    Vector3 operator*(float scalar) const { return {x * scalar, y * scalar, z * scalar}; }
    float dot(const Vector3& v) const { return x * v.x + y * v.y + z * v.z; }
    Vector3 normalize() const {
        float mag = std::sqrt(x * x + y * y + z * z);
        return {x / mag, y / mag, z / mag};
    }
};

// Sphere representation
struct Sphere {
    Vector3 center;
    float radius;
    SDL_Color color;

    // Constructor to allow proper initialization
    Sphere(Vector3 c, float r, SDL_Color col) : center(c), radius(r), color(col) {}

    // Check if a ray intersects the sphere
    bool intersect(const Vector3& rayOrigin, const Vector3& rayDir, float& t) const {
        Vector3 oc = rayOrigin - center;
        float a = rayDir.dot(rayDir);
        float b = 2.0f * oc.dot(rayDir);
        float c = oc.dot(oc) - radius * radius;
        float discriminant = b * b - 4 * a * c;

        if (discriminant < 0) return false;

        float t1 = (-b - std::sqrt(discriminant)) / (2.0f * a);
        float t2 = (-b + std::sqrt(discriminant)) / (2.0f * a);

        t = (t1 > 0) ? t1 : t2;
        return t > 0;
    }
};

// Function to compute lighting (basic diffuse shading)
float computeLighting(const Vector3& point, const Vector3& normal, const Vector3& lightDir) {
    return std::max(0.0f, normal.dot(lightDir.normalize()));
}

int main(int argc, char* argv[]) {
    // Initialize SDL
    if (SDL_Init(SDL_INIT_VIDEO) < 0) {
        SDL_Log("Failed to initialize SDL: %s", SDL_GetError());
        return -1;
    }

    const int width = 800;
    const int height = 600;

    SDL_Window* window = SDL_CreateWindow(
        "Simple Ray Tracer", SDL_WINDOWPOS_CENTERED, SDL_WINDOWPOS_CENTERED, 
        width, height, SDL_WINDOW_SHOWN
    );
    if (!window) {
        SDL_Log("Failed to create window: %s", SDL_GetError());
        SDL_Quit();
        return -1;
    }

    SDL_Renderer* renderer = SDL_CreateRenderer(
        window, -1, SDL_RENDERER_ACCELERATED | SDL_RENDERER_PRESENTVSYNC
    );
    if (!renderer) {
        SDL_Log("Failed to create renderer: %s", SDL_GetError());
        SDL_DestroyWindow(window);
        SDL_Quit();
        return -1;
    }

    // Define the scene
    std::vector<Sphere> spheres = {
        Sphere({0, 0, -5}, 1.0f, SDL_Color{255, 0, 0, 255}),  // Red sphere
        Sphere({-1.5, 0, -4}, 0.5f, {0, 255, 0, 255}), // Green sphere
        Sphere({1.5, 0, -6}, 1.5f, {0, 0, 255, 255})  // Blue sphere
    };

    Vector3 lightDir = {0.5f, -1.0f, -0.5f}; // Directional light source

    // Render the scene
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

        for (int y = 0; y < height; ++y) {
            for (int x = 0; x < width; ++x) {
                // Convert pixel coordinate to ray direction
                float u = (x - width / 2.0f) / (width / 2.0f);
                float v = (height / 2.0f - y) / (height / 2.0f);
                Vector3 rayOrigin = {0, 0, 0}; // Camera at (0, 0, 0)
                Vector3 rayDir = Vector3{u, v, -1}.normalize(); // Fixed

                // Ray tracing logic
                SDL_Color pixelColor = {0, 0, 0, 255}; // Default black
                float closestT = INFINITY;
                for (const auto& sphere : spheres) {
                    float t;
                    if (sphere.intersect(rayOrigin, rayDir, t) && t < closestT) {
                        closestT = t;
                        Vector3 hitPoint = rayOrigin + rayDir * t;
                        Vector3 normal = (hitPoint - sphere.center).normalize();
                        float intensity = computeLighting(hitPoint, normal, lightDir);

                        pixelColor.r = static_cast<Uint8>(sphere.color.r * intensity);
                        pixelColor.g = static_cast<Uint8>(sphere.color.g * intensity);
                        pixelColor.b = static_cast<Uint8>(sphere.color.b * intensity);
                    }
                }

                // Set pixel color
                SDL_SetRenderDrawColor(renderer, pixelColor.r, pixelColor.g, pixelColor.b, 255);
                SDL_RenderDrawPoint(renderer, x, y);
            }
        }

        // Present the frame
        SDL_RenderPresent(renderer);
    }

    // Clean up
    SDL_DestroyRenderer(renderer);
    SDL_DestroyWindow(window);
    SDL_Quit();

    return 0;
}
