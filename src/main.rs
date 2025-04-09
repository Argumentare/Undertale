

use std::u32::MAX;
use crate::game::undertale;
mod floor;
mod game;
mod entities;
mod actions;
mod UI;
mod spells;


fn main() {
    
    game::currentlvl();
    undertale::run();
    
    
}
