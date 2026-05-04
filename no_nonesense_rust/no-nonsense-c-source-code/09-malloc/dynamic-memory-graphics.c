#include "raylib.h"
#include <math.h>
#include <stdlib.h>
#include <stdio.h>
#include <time.h>

#define WIDTH 900
#define HEIGHT 600

typedef struct {
    double d, angle, speed, radius;
    Color color;
} Circle;

int circle_count = 1;

void draw_circle(Circle *circle_memory)
{
    for (int i=0; i<circle_count; i++)
    {
        Circle *circle = &circle_memory[i];
        double angle = circle->angle;

        // Find x,y at which to draw the circle
        double x = circle->d*sin(angle) + WIDTH / 2;
        double y = circle->d*cos(angle) + HEIGHT / 2;
        DrawCircle((int) x, (int) y, 50, circle->color);
    }
}

void move_circle(Circle *circle_memory, float time_difference)
{
    for (int i=0; i<circle_count; i++)
    {
        Circle *circle = &circle_memory[i];
        circle->angle += circle->speed * time_difference;
    }
}

void generate_circle(Circle *circle)
{
    circle->d = rand() % 191 + 10; // -> returns number between 10 .. 200
    circle->angle = 0;
    circle->speed = rand() % 9 + 1; // -> 1..9
    circle->radius = rand() % 40; // -> 0..9

    //randomize color
    circle->color.r = rand() % 256;
    circle->color.g = rand() % 256;
    circle->color.b = rand() % 256;
    circle->color.a = 255;
}

int main(void)
{
    InitWindow(WIDTH, HEIGHT, "Graphics in C");
    srand(time(NULL));

    SetTargetFPS(60);

    Circle *circle_memory = malloc(circle_count * sizeof(Circle));
    generate_circle(&circle_memory[circle_count - 1]);

    while (!WindowShouldClose())
    {
        // check if user presses keyboard:
        if (IsKeyPressed(KEY_ENTER))
        {
            printf("Key pressed!\n");
            circle_count++;
            circle_memory = realloc(circle_memory, circle_count * sizeof(Circle));

            Circle *new_circle = &circle_memory[circle_count-1];
            generate_circle(new_circle);
        }

        move_circle(circle_memory, GetFrameTime());

        BeginDrawing();
            ClearBackground(BLACK);
            DrawFPS(20,20);
            DrawText("No-Nonsense Programming Animation", 120, 20, 20, LIGHTGRAY);
            // draw anything here
            draw_circle(circle_memory);

        EndDrawing();
    }

    CloseWindow();

    free(circle_memory);    

    return 0;
}
