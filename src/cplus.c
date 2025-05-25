#include "raylib.h"
#include <stdio.h>
#include <stdint.h>



void gameloop(void);
int enmiesnumber();


typedef struct enemyinformation{
    int32_t health;
    char name;
    float pos;
}eneinf;

eneinf getenemyinfo(int32_t);

void graphics()
{
    const int screenWidth = 800;
    const int screenHeight = 450;
   
    //Button
    Texture2D button = LoadTexture("resources/button.png");
    Rectangle btnBounds = {screenWidth/2 - button.width/2,screenHeight/2 - button.height/2, (float)button.width, (float)button.height};
     Rectangle sourceRec = { 0, 0, (float)button.width,(float)button.height};


    
    
    Vector2 ballpos = {100,100};
    InitWindow(screenWidth, screenHeight, "Undertale");

    SetTargetFPS(60);               
    
    
    // Main game loop
    while (!WindowShouldClose())    
    {
        
        
        BeginDrawing();

            ClearBackground(RAYWHITE);
            
            
            for (int32_t i = 0; i < enmiesnumber(); i++)
            {

                eneinf enemy = getenemyinfo(i);
                Vector2 pose = {enemy.pos*100,100};
                

                //Emenmy Sprites
                DrawCircleV(pose, 50, MAROON);    
                DrawText(TextFormat("%d",enemy.health),pose.x,pose.y,20,BLACK);
                DrawTextureRec(button, sourceRec, (Vector2){400,400}, WHITE);
            }
        EndDrawing();
        gameloop(); 
    }

    CloseWindow();        
    
    
}