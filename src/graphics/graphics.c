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
    NMtextures mainbody = MainBody();
    but speells1 = SpellSlot1();
    // Main game loop
    while (!WindowShouldClose())    
    {
        playerinf player = getplayerinfo();

        if(CheckCollisionPointRec(GetMousePosition(),attackbutton.Bounds)){
             if (IsMouseButtonReleased(MOUSE_BUTTON_LEFT)) attackbutton.ispressed = true;       
        }

        if(CheckCollisionPointRec(GetMousePosition(),runbutton.Bounds)){
             if (IsMouseButtonReleased(MOUSE_BUTTON_LEFT)) runbutton.ispressed = true;       
        }

        if(attackbutton.ispressed){
            attackbutton.ispressed = false;
            takeaction(Attack);
        }

        if(runbutton.ispressed){
            runbutton.ispressed = false;
            takeaction(Run);
        }
        
        BeginDrawing();

            ClearBackground(RAYWHITE);
            
            
            for (int32_t i = 0; i < enmiesnumber(); i++)
            {

                eneinf enemy = getenemyinfo(i);
                Vector2 pose = {enemy.pos*100,100};
                float radius = 50;

                //Emenmy Sprites
                if(enemy.isalive ){      
                DrawCircleV(pose, radius, MAROON);    
                DrawText(TextFormat("%d",enemy.health),pose.x,pose.y,20,BLACK);
                }
            }
                //Player UI Sprites
                    //MainBody
                    DrawTexture(mainbody.texture,mainbody.position.x,mainbody.position.y,WHITE);    

                    //Buttons
                    DrawTextureRec(attackbutton.texture, attackbutton.sourceRec, (Vector2){attackbutton.Bounds.x,attackbutton.Bounds.y}, WHITE);
                    DrawTextureRec(runbutton.texture, runbutton.sourceRec, (Vector2){runbutton.Bounds.x,runbutton.Bounds.y}, WHITE);
                    DrawTextureRec(speells1.texture,speells1.sourceRec, (Vector2){speells1.Bounds.x,speells1.Bounds.y},WHITE);
                    //PInfo
                    DrawText(TextFormat("%d",player.HEALTH),1700,30,50,RED);
                    DrawText(TextFormat("%d",player.MANA),1750,30,50,BLUE);
                    DrawText(TextFormat("%d",player.COINS),1820,30,50,YELLOW);

            
        EndDrawing();
        gameloop(); 
    }
    UnloadTexture(mainbody.texture);
    UnloadTexture(attackbutton.texture); 
    UnloadTexture(runbutton.texture);

    CloseWindow();        
    
    
}