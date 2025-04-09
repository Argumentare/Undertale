
use std::alloc::GlobalAlloc;

use crate::entities::enemies;
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

struct enormalflors
{
    number:i32,
    loot:bool,
  //  enemies:Vec<String>,
}

impl floors
{
    

    pub fn nextlvl()-> &'static floors
    {
       unsafe 
       { 
        let  lvl:usize = CURRENTLVL;
        
        CURRENTLVL += 1;
        &ALLFLORS[lvl]
       }
       
    }


}