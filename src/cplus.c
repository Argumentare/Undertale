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
    const int screenWidth = 1920;
    const int screenHeight = 1080;
     
    InitWindow(screenWidth, screenHeight, "Undertale");
    SetTargetFPS(60);               
    
    //Button
    Texture2D button = LoadTexture("resources/button.jpg");
    Rectangle btnBounds = {0,800, (float)button.width, (float)button.height};
    Rectangle sourceRec = { 0, 0, (float)button.width,(float)button.height};

    // Main game loop
    while (!WindowShouldClose())    
    {
        bool btnActionAttack = false;
        if(CheckCollisionPointRec(GetMousePosition(),btnBounds)){
             if (IsMouseButtonReleased(MOUSE_BUTTON_LEFT)) btnActionAttack = true;       
        }

        if(btnActionAttack){
            CloseWindow();
        }
        
        BeginDrawing();

            ClearBackground(RAYWHITE);
            
            
            for (int32_t i = 0; i < enmiesnumber(); i++)
            {

                eneinf enemy = getenemyinfo(i);
                Vector2 pose = {enemy.pos*100,100};
                

                //Emenmy Sprites
                DrawCircleV(pose, 50, MAROON);    
                DrawText(TextFormat("%d",enemy.health),pose.x,pose.y,20,BLACK);
                DrawTextureRec(button, sourceRec, (Vector2){btnBounds.x,btnBounds.y}, WHITE);
            }
        EndDrawing();
        gameloop(); 
    }

    UnloadTexture(button); 

    CloseWindow();        
    
    
}