use std::fmt::Debug;
use std::io::{self, Read};
use crate::entities::{self, enemies};
use crate::{actions, floor, main, spells};
use crate::actions::{Actions,runf,ActionFrstr};
use crate::floor::floors;
use core::str::FromStr;
use colored::Colorize;
use crate::UI;
use crate::spells::spell;

/*##############
////////PLAYER
*/
const MAXHEALTHP:i32 = 9;
static mut HEALTH:i32 = 0;

const MAXMANA:i32 = 20;
pub static mut MANA:i32 = 0; 

pub const COINS:i32 = 0; 

pub static mut ATTACKDAMAGE:i32 =0;
pub static mut CANATTACK:bool = false;
pub static mut CANSPELL:bool = false;
pub static mut OWNED_SPELLS:Vec<spell> = Vec::new();
/*##############
////////PLAYER
*/

pub static  mut VEC:Vec<enemies> = Vec::new();
pub static mut DEBUGING:bool = false;




pub enum undertale
{
    
    
}

pub fn callenemy(enemy:&str )
    { 
        
        match enemy {
            "goblin" =>
            {
                let goblin = enemies::spawnenemy(String::from("goblin"));  
                addenemytovec(goblin);    
            }
            "zombie" => 
            {
                let zombie = enemies::spawnenemy(String::from("zombie"));  
                addenemytovec(zombie);    
            }
                &_ => println!("{}", "not an enmey".to_uppercase().white()),   
         
        }  
    }   


pub fn currentlvl()
{
    let currentlvl = floors::nextlvl();
    
        let floors::normalflors {enemiesf,.. } = currentlvl;
        {
            for x in 0..enemiesf.len()
            {
                callenemy(enemiesf[x]);
            }
        }

}

pub fn take_damage(damage:i32)
    {
        unsafe {
            HEALTH -= damage;
        }
        
    }

pub fn addenemytovec(enemy:enemies)
{
    unsafe{  VEC.push(enemy);}
}

impl undertale
{
    
    
    pub fn run()
    {
        unsafe{
        spells::starting_spell();
        HEALTH = MAXHEALTHP;
        MANA = MAXMANA}
        
        loop 
        {
            let mut incombat:bool = true;
            let mut input:String = String::new();
            let manaui = unsafe{MANA/10};
            let enemies_alive = enemies::check_for_enemies();
            
            if unsafe{DEBUGING} 
            {
                
                
                io::stdin().read_line(&mut input).expect("wrong input");
                let input:i32 = input.trim().parse().expect("not a number");
                match  input {
                1 => println!("{:?}",enemies_alive),
                2 => incombat = true,
                3 => currentlvl(),
                4 => unsafe{println!("{:?}",VEC);}
                _ => (),
                }
            }



            if incombat && unsafe{!CANATTACK && !DEBUGING && !CANSPELL} && enemies_alive 
            {

            unsafe {
            println!("{}({HEALTH}){}","PLAYER HEALTHBAR".bold().red(),UI::HEALTHBAR[HEALTH as usize].red());
            println!("{}({MANA}){}", "PLAYER MANA".bold().bright_blue(),UI::MANA[manaui as usize].bright_blue());
            }
            actions::incombat();
            println!("{} ",(UI::ATTACK.to_owned() + UI::RUN).bright_magenta() );
            let enemies_alive = enemies::check_for_enemies();
            if enemies_alive
            {
                io::stdin().read_line(&mut input).expect("wrong input");
                let input:Actions = Actions::action_from_string(input);
                Actions::takeaction(input);
            }
      
        
        }else if unsafe{CANATTACK} && enemies_alive {
            
            println!("{}","Cast on an enemy".bold());
            io::stdin().read_line(&mut input).expect("wrong input");
            let input:usize = input.trim().parse().expect("not a number");  
            actions::attackF(input);
            take_damage(enemies::calc_player_dmg());
         
        }else if !enemies_alive{
            floors::nextlvl();
        }
        
        if unsafe{!CANATTACK && enemies_alive && CANSPELL }
        {
            
            let mut input:String = String::new();
            io::stdin().read_line(&mut input).expect("wrong input");
            let input:usize = input.trim().to_lowercase().parse().expect("not a spell");
            actions::check_for_mana(input);
        }
            
        }
    }


    
}