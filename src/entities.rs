
use std::vec;
use crate::{floor, game::{self, VEC}};


#[derive(Debug)]
pub enum enemies
{
    enemy
    {
        name:String,
        health:i32,
        isalive:bool,
        attackdamage:i32,
        tag:String,
        pos:f32,
    },  
}


#[repr(C)]

pub struct enemyinformation{
    health:i32,
    name:char,
    pos:f32,
    isalive:bool,
}



pub const EXISTING_ENEMIES:[&'static str;2] = ["goblin","imp"]; 
impl enemies
{
     
    pub fn spawnenemy(enemyname:String) -> enemies
    {
        
        
        match enemyname.as_str()
        { 
            
            "goblin" =>{ 
               
                let goblin = enemies::enemy { name: (enemyname ), health: (100), isalive: (true), tag:("goblin".to_string()),attackdamage: (1),pos: 1f32};
                
               
                return  goblin;
                
            }

            "imp" =>{ 
               
                let zombie = enemies::enemy { name: (enemyname), health: (10), isalive: (true), tag: ("imp".to_string()),attackdamage: (1),pos: 1f32};
                
               
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
 
    #[no_mangle]
    pub extern "C" fn getenemyinfo(x:i32) -> enemyinformation{
        
        unsafe{
        let enemies::enemy {name,health,pos,isalive,..} = &game::VEC[x as usize];{
            
            let b = *health;
            let c = *pos + x as f32;
            let mut d:char = ' ';
            let a = *isalive;
            match &*name.trim().to_lowercase().as_str(){

                "imp" => d = 'c',
                "goblin" => d = 'g', 
                &_ => (),
            }
            let enemy:enemyinformation = enemyinformation { health: (b), name: (d), pos: (c),isalive: (a) };   
            return enemy;
        }}      
        
    }

    #[no_mangle]
    pub extern "C" fn enmiesnumber() -> i32{
        unsafe{return VEC.len() as i32;}
    }
    
}