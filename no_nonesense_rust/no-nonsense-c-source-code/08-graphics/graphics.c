#include "raylib.h"
#include <math.h>

#define AMPLITUDE 120
#define WIDTH 900
#define HEIGHT 600

int main(void)
{
    InitWindow(WIDTH, HEIGHT, "Graphics in C");

    double angle = 0;

    double x, y;
    SetTargetFPS(60);
    while (!WindowShouldClose())
    {
        BeginDrawing();
            ClearBackground(BLACK);
            DrawFPS(20,20);
            DrawText("No-Nonsense Programming Animation", 120, 20, 20, LIGHTGRAY);
            // draw anything here
            x = AMPLITUDE*sin(angle) + WIDTH / 2;
            y = AMPLITUDE*cos(angle) + HEIGHT / 2;
            angle = angle + 0.04;
            DrawCircle(x, y, 50, RED);

            x = AMPLITUDE*0.5*sin(angle-3) + WIDTH/2;
            y = AMPLITUDE*0.5*cos(angle-3) + HEIGHT/2;
            DrawCircle(x,y, 20, BLUE);

            x = AMPLITUDE*0.2*sin(angle-4) + WIDTH/2;
            y = AMPLITUDE*0.2*cos(angle-4) + HEIGHT/2;
            DrawCircle(x,y, 12, GREEN);
        EndDrawing();
    }

    CloseWindow();

    return 0;
}
