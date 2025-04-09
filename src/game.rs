use std::fmt::Debug;
use std::io::{self, Read};
use crate::entities::{enemies};
use crate::{actions, floor, main};
use crate::actions::{Actions,runf,ActionFrstr};
use crate::floor::floors;
use core::str::FromStr;
use colored::Colorize;
use crate::UI;

/*##############
////////PLAYER
*/
const MAXHEALTHP:i32 = 9;
static mut HEALTH:i32 = 0;
pub static mut ATTACKDAMAGE:i32 =20;
pub static mut CANATTACK:bool = false;
/*##############
////////PLAYER
*/

pub static  mut VEC:Vec<enemies> = Vec::new();
const DEBUGING:bool = false;




pub enum undertale
{
    
    
}

pub fn callenemy(enemy:[&str;2] )
    { 
        for x in 0..enemy.len()
        {
        match enemy[x] {
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
    }   


pub fn currentlvl()
{
    let currentlvl = floors::nextlvl();
    
        match currentlvl
        {
            floors::normalflors {enemies:["goblin","zombie"],..} => {
                callenemy(["goblin","zombie"]);
            }
            &_ => (),
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
        unsafe{HEALTH = MAXHEALTHP;}
        loop 
        {
            let mut incombat:bool = true;
             
            let mut input:String = String::new();
            
            if DEBUGING && !incombat
            {
                
                
                io::stdin().read_line(&mut input).expect("wrong input");
                let input:i32 = input.trim().parse().expect("not a number");
                match  input {
                1 => println!("{:?}",floors::nextlvl()),
                2 => incombat = true,
                3 => currentlvl(),
                4 => unsafe{println!("{:?}",VEC);}
                _ => (),
                }
            }
            if incombat && unsafe{!CANATTACK}
            {
            
            println!("{}{}","PLAYER HEALTHBAR".bold().red(),UI::HEALTHBAR[unsafe{HEALTH as usize}].red());
            actions::incombat();
            println!("{}",(UI::ATTACK.to_owned() + UI::RUN).bright_purple(), );
            
        io::stdin().read_line(&mut input).expect("wrong input");
        let input:Actions = Actions::action_from_string(input);
        
        Actions::takeaction(input);
        
        }else if unsafe{CANATTACK } {
            
            io::stdin().read_line(&mut input).expect("wrong input");
            let input:usize = input.trim().parse().expect("not a number");
            actions::attackF(input);
            take_damage(enemies::calc_player_dmg());
         
        }
            
        }
    }


    
}