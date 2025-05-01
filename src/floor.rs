
use std::alloc::GlobalAlloc;

use colored::Colorize;
use rand::Rng;

use crate::{entities::{self, enemies}, game, loot};
static mut CURRENTLVL:i32 = 0; 


#[derive(Debug)]
//#[derive(Clone, Copy)]
pub enum floors
{ 
    normalflors
    {
        number:i32,
        loot:bool,
        enemiesf:Vec<&'static str>
    }
} 

pub fn generate_floor(currentlvl:i32) ->floors {  
    
    let enemies_number = rand::rng().random_range(2..=3);
    let mut enemies:Vec<&'static str> = Vec::new();
    for x in 0..enemies_number
    {
        let enemy = entities::EXISTING_ENEMIES[rand::rng().random_range(0..entities::EXISTING_ENEMIES.len())];
        enemies.push(enemy);
    }
    let floor = floors::normalflors { number: (currentlvl), loot: (true), enemiesf: (enemies) };
    return  floor;
}

impl floors
{
    

    pub fn nextlvl()-> floors
    {
       unsafe 
       { 
        
        let  lvl:i32 = CURRENTLVL;    
        let floor = generate_floor(lvl);
        
        if CURRENTLVL > 0 {
            println!("{}:{} {}","Floor".italic().blue(),CURRENTLVL.to_string().bold().blue(),"cleared, here's your loot".italic().blue());
            let floors::normalflors {enemiesf,..} = &floor;
            {
                println!("{:?}",loot::randomize_loot(enemiesf.len()));
            }
            println!("
    CONTINUE or QUIT");
        }
        CURRENTLVL += 1;     
        return  floor;
        
       }
       
    }


}