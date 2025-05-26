#include "raylib.h"
#include <stdint.h>

typedef struct button{
    Texture2D texture;
    Rectangle Bounds;
    Rectangle sourceRec;
    bool ispressed;
}but;

but AttackButton(){
    but attackBut;
    attackBut.texture = LoadTexture("resources/attackbut.jpg");
    attackBut.Bounds = (Rectangle){0,800, (float)attackBut.texture.width, (float)attackBut.texture.height};
    attackBut.sourceRec = (Rectangle){0,0,(float)attackBut.texture.width, (float)attackBut.texture.height};
    return attackBut;
}

but RunButton(){
    but runBut;
    runBut.texture = LoadTexture("resources/runbut.jpg");
    runBut.Bounds = (Rectangle){0,450, (float)runBut.texture.width, (float)runBut.texture.height};
    runBut.sourceRec = (Rectangle){0,0,(float)runBut.texture.width, (float)runBut.texture.height};
    return runBut;
}
