#include "raylib.h"
#include <stdio.h>
#include <stdint.h>
#include "UI.h"
#include "gameinfo.h"

void gameloop(void);
int enmiesnumber();


void graphics()
{
    const int screenWidth = 1920;
    const int screenHeight = 1080;
     
    InitWindow(screenWidth, screenHeight, "Undertale");
    SetTargetFPS(60);               
    
    but attackbutton = AttackButton();
    but runbutton = RunButton();

    // Main game loop
    while (!WindowShouldClose())    
    {
    
        if(CheckCollisionPointRec(GetMousePosition(),attackbutton.Bounds)){
             if (IsMouseButtonReleased(MOUSE_BUTTON_LEFT)) attackbutton.ispressed = true;       
        }

        if(CheckCollisionPointRec(GetMousePosition(),runbutton.Bounds)){
             if (IsMouseButtonReleased(MOUSE_BUTTON_LEFT)) runbutton.ispressed = true;       
        }

        if(attackbutton.ispressed){
            takeaction(Attack);
        }

        if(runbutton.ispressed){
            takeaction(Run);
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
                DrawTextureRec(attackbutton.texture, attackbutton.sourceRec, (Vector2){attackbutton.Bounds.x,attackbutton.Bounds.y}, WHITE);
                DrawTextureRec(runbutton.texture, runbutton.sourceRec, (Vector2){runbutton.Bounds.x,runbutton.Bounds.y}, WHITE);

            }
        EndDrawing();
        gameloop(); 
    }

    UnloadTexture(attackbutton.texture); 
    UnloadTexture(runbutton.texture);

    CloseWindow();        
    
    
}