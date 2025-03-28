
use std::vec;
use crate::randomidgenerator::id;
#[derive(Debug)]
pub enum enemies
{
    enemy
    {
        name:String,
        health:i32,
        isalive:bool,
        id:id
    },
    player{
        name:String,
        health:i32,
        isalive:bool,
    },
    
    
}

impl enemies
{
     
    pub fn spawnenemy(enemyname:String) -> enemies
    {
        
        let ID:id = id::randomid();
        match &enemyname
        { 
            
            goblin =>{ 
               
                let goblin = enemies::enemy { name: (enemyname ), health: (100), isalive: (true), id: (ID) };
                
               
                return  goblin;
                
            }
            
        }
        
    }

    
}