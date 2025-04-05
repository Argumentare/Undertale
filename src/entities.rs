
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

            zombie =>{ 
               
                let zombie = enemies::enemy { name: (enemyname ), health: (100), isalive: (true), id: (ID) };
                
               
                return  zombie;
                
            }

            
        }
        
    }

    fn TakeDamage(&self,damage:i32) {
            let enemies::enemy { mut health,..} = self;
            {
                health -= damage;
            }
    }
    
}