
use std::vec;
use crate::{game};


#[derive(Debug)]
pub enum enemies
{
    enemy
    {
        name:String,
        health:i32,
        isalive:bool,
        attackdamage:i32,
        tag:String
    },
    
    
    
}

impl enemies
{
     
    pub fn spawnenemy(enemyname:String) -> enemies
    {
        
        
        match &enemyname
        { 
            
            goblin =>{ 
               
                let goblin = enemies::enemy { name: (enemyname ), health: (100), isalive: (true), tag:("golbin".to_string()),attackdamage: (1) };
                
               
                return  goblin;
                
            }

            zombie =>{ 
               
                let zombie = enemies::enemy { name: (enemyname ), health: (100), isalive: (true), tag: ("zombie".to_string()),attackdamage: (1) };
                
               
                return  zombie;
                
            }

            
        }
        
    }

    pub fn calc_player_dmg() -> i32
    {
        let mut totaldamage = 0;
        unsafe{
        for x in 0..game::VEC.len()
        {
            let enemies::enemy { attackdamage, .. } = game::VEC[x];
            {
                totaldamage += attackdamage;
            }
        }    
        }
        return  totaldamage;
    }
    
}