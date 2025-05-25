

use std::u32::MAX;
use crate::game::undertale;

mod floor;
mod game;
mod entities;
mod actions;
mod UI;
mod spells;
mod loot;

extern "C"{
    fn graphics();
     }

fn main() {
    
    
  //  game::currentlvl();
   // game::undertale::gameloop();
    unsafe{graphics();}
    
    
}
