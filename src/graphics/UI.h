#include "raylib.h"
#include <stdint.h>

typedef struct button{
    Texture2D texture;
    Rectangle Bounds;   
    Rectangle sourceRec;
    char text;
    bool ispressed;
}but;

typedef struct NormalTextures{
    Texture2D texture;
    Vector2 position;

}NMtextures;

NMtextures MainBody(){
    NMtextures mainbody;
    mainbody.texture = LoadTexture("resources/mainbody.png");
    mainbody.position = (Vector2){750,450};
    return mainbody;
}
but SpellSlot1(){
    but spellS1;
    NMtextures mainbody = MainBody();
    spellS1.texture = LoadTexture("resources/spellslot.png");
    spellS1.Bounds = (Rectangle){mainbody.position.x,mainbody.position.y, (float)spellS1.texture.width, (float)spellS1.texture.height};
    spellS1.sourceRec = (Rectangle){0,0,(float)spellS1.texture.width, (float)spellS1.texture.height};
    return spellS1;
}


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
