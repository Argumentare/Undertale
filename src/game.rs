use core::panic;
use std::{fmt,fmt::Debug};

use std::io::{self, Read};
use std::path::Display;
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
const MAXHEALTHP:i32 = 10;
pub static mut HEALTH:i32 = 0;

 const MAXMANA:i32 = 20;
pub static mut MANA:i32 = 0; 
pub static mut COINS:i32 = 2; 

pub static mut ATTACKDAMAGE:i32 =0;
pub static mut CANATTACK:bool = false;
pub static mut CANSPELL:bool = false;
pub static mut OWNED_SPELLS:Vec<spell> = Vec::new();

pub static mut CANNEXTFLOOR:bool = false;
/*##############
////////PLAYER
*/

pub static  mut VEC:Vec<enemies> = Vec::new();
pub static mut DEBUGING:bool = false;



#[repr(C)]
pub struct playerinformation{
    mana:i32,
    health:i32,
    coins:i32,
    CANSPEL:bool,
    spellcount:i32,
}

pub enum undertale
{
    
    
}

pub fn start(){
    spells::starting_spell();
    unsafe{HEALTH = MAXHEALTHP;
    MANA = MAXMANA;}
    
}

pub fn callenemy(enemy:&str )
    { 
        
        match enemy {
            "goblin" =>
            {
                let goblin = enemies::spawnenemy(String::from("goblin"));  
                addenemytovec(goblin);    
            }
    
            "imp" => {let imp = enemies::spawnenemy(String::from("imp"));
            addenemytovec(imp);
            }


                &_ => println!("{}", "not an enmey".to_uppercase().white()),   
         
        }  
    }   


pub fn currentlvl()
{
    let currentlvl = floors::nextlvl();
        unsafe{VEC = Vec::new();}
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
    unsafe{  VEC.push( enemy);}
}

impl undertale
{
    
    
    #[no_mangle]
    pub extern "C" fn gameloop(){
        unsafe{
        
        }
            
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
           
            actions::incombat();
            let enemies_alive = enemies::check_for_enemies();
        
         }else if unsafe{CANATTACK && enemies_alive && !CANSPELL} {
            
            println!("{}","Cast on an enemy".bold());
           io::stdin().read_line(&mut input).expect("wrong input");
            let input:usize = input.trim().parse().expect("not a number");   
            actions::attackF(input);
         
        }else if !enemies_alive{
            
           currentlvl();
           io::stdin().read_line(&mut input).expect("not a option");
           match input.trim().to_lowercase().as_str(){
                "quit" => std::process::exit(0),
                "continue" => unsafe{CANNEXTFLOOR = true},
                &_ => (),
           }
        }
        
        if unsafe{CANATTACK && enemies_alive && CANSPELL }
        {
            
            let mut input:String = String::new();
            io::stdin().read_line(&mut input).expect("wrong input");
            let input:usize = input.trim().to_lowercase().parse().expect("not a spell");
            actions::check_for_mana(input);
        }
            
        
        
    }

   #[no_mangle]
    pub extern "C" fn getplayerinfo() -> playerinformation{

        unsafe{
        playerinformation { mana: (MANA), health: (HEALTH), coins: (COINS),
        CANSPEL:(CANSPELL),spellcount:(OWNED_SPELLS.len() as i32)}
       }
    }   


    
}