#include <stdint.h>

typedef struct enemyinformation{
    int32_t health;
    char name;
    float pos;
    bool isalive;
}eneinf;
eneinf getenemyinfo(int32_t);

typedef struct playerinformation{
    int32_t MANA;
    int32_t HEALTH;
    int32_t COINS;
    char spells[];
    
}playerinf;
playerinf getplayerinfo();

typedef enum Actions
{
    Attack,
    Run,
    Talk,   
    Eat,
    Donothing,
    Debug,
}actions;
actions takeaction(actions action);
