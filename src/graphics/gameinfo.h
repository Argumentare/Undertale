#include <stdint.h>

typedef struct enemyinformation{
    int32_t health;
    char name;
    float pos;
    bool isalive;
}eneinf;
eneinf getenemyinfo(int32_t);

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
