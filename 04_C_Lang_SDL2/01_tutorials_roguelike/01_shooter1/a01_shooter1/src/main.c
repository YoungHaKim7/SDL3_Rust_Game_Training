#include <stdlib.h>
#include <string.h>
#include <SDL2/SDL.h>
#include "structs.h"
#include "init.h"
#include "draw.h"
#include "input.h"

App app;

void cleanup(void) {
    SDL_DestroyRenderer(app.renderer);
    SDL_DestroyWindow(app.window);
    SDL_Quit();

}


int main(int argc, char *argv[])
{
    memset(&app, 0, sizeof(App));

    initSDL();

    atexit(cleanup);

    while (1)
    {
        prepareScene(&app);

        doInput();

        presentScene(&app);

        SDL_Delay(16);
    }

    return 0;
}
