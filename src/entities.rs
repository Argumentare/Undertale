
use std::vec;
use crate::{floor::{self, ALLFLORS}, game};


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
        
        
        match enemyname.as_str()
        { 
            
            "goblin" =>{ 
               
                let goblin = enemies::enemy { name: (enemyname ), health: (100), isalive: (true), tag:("goblin".to_string()),attackdamage: (1) };
                
               
                return  goblin;
                
            }

            "zombie" =>{ 
               
                let zombie = enemies::enemy { name: (enemyname ), health: (100), isalive: (true), tag: ("zombie".to_string()),attackdamage: (1) };
                
               
                return  zombie;
                
            }
            &_ => panic!("not an enemy"),
            
        }
        
    }

    pub fn calc_player_dmg() -> i32
    {
        let mut totaldamage = 0;
        unsafe{
        for x in 0..game::VEC.len()
        {
            
            let enemies::enemy { attackdamage,health,.. } = game::VEC[x];
            {
                if health > 0
                {
                    totaldamage += attackdamage;
                }
            }
        }    
        }
        return  totaldamage;
    }


    pub fn check_for_enemies() -> bool
    {
        unsafe{
        for x in 0..game::VEC.len()
        {
            let enemies::enemy {isalive,.. } = game::VEC[x];
            {
                if isalive 
                {
                    return true;
                }
            }
        }
        }
        return  false;
    }
    
}