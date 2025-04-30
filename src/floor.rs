
use std::alloc::GlobalAlloc;

use colored::Colorize;

use crate::{entities::enemies, game, loot};
static mut CURRENTLVL:usize = 0; 
pub static mut ALLFLORS:[floors;2] = [


floors::normalflors{number:(1),loot:(true),enemiesf:["goblin", "zombie"]},

floors::normalflors{number:(2),loot:(true),enemiesf:["goblin", "zombie"]}]; 


#[derive(Debug)]
//#[derive(Clone, Copy)]
pub enum floors
{ 
    normalflors
    {
        number:i32,
        loot:bool,
        enemiesf:[&'static str;2],
    }
} 


impl floors
{
    

    pub fn nextlvl()-> &'static floors
    {
       unsafe 
       { 
       
        let  lvl:usize = CURRENTLVL;    
        
        if CURRENTLVL > 0 {
            println!("{}:{} {}","Floor".italic().blue(),CURRENTLVL.to_string().bold().blue(),"cleared, here's your loot".italic().blue());
            println!("{:?}",loot::randomize_loot());
            
            println!("
    CONTINUE or QUIT");
        }
        CURRENTLVL += 1;     
        &ALLFLORS[lvl]
        
       }
       
    }


}