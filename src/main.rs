

use std::u32::MAX;
use crate::game::undertale;
mod floor;
mod game;
mod entities;
mod randomidgenerator; 
mod actions;
mod UI;


fn main() {
    
    game::currentlvl();
    undertale::run();
    
    
}
