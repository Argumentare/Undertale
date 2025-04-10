use core::panic;
use std::io;
use colored::Colorize;
use crate::entities::enemies;
use crate::game;
use crate::entities;
use crate::game::ATTACKDAMAGE;
use crate::game::CANATTACK;
use crate::game::VEC;
use std::str::FromStr;
use crate::undertale;
use crate::spells;



#[derive(Debug,PartialEq,Eq)]
pub enum Actions
{
    Attack,
    Run,
    Talk,   
    Eat,
    Donothing,
    Debug,
}

pub trait ActionFrstr {
    fn action_from_string(string:String) -> Actions;
}

impl ActionFrstr for Actions
{
    fn action_from_string(string:String) -> Actions {
        
        
        match string.to_lowercase().as_str().trim()
        {
            
            "run" => Actions::Run,
            "eat" => Actions::Eat,
            "talk" => Actions::Talk,
            "attack" => Actions::Attack,
            "debug" => Actions::Debug,
            &_ => Actions::Donothing,
                
        }
    }
}
impl Actions
{
    pub fn takeaction(action:Actions)
    {
        match action
        {
            Actions::Attack => choose_a_spell(),
            Actions::Run => runf(),
            Actions::Eat => eatF(),
            Actions::Talk => talkF(),
            Actions::Debug => unsafe{game::DEBUGING = true},
            Actions::Donothing => (),
        }

    }
}



pub fn runf()
{
    std::process::exit(0);
}

fn choose_a_spell()
{
    println!("{}","Choose a SPELL".bold());

    unsafe{ 
        
        for x in 0..game::OWNED_SPELLS.len()
        {
            let spells::spell{name,damage,..} = game::OWNED_SPELLS[x];
            {
                println!("{x}.{}(damage:{})",name,damage);
            }
        }   
    }
    let mut input:String = String::new();
    io::stdin().read_line(&mut input).expect("wrong input");
    let input:usize = input.trim().to_lowercase().parse().expect("not a spell");
    let spells::spell{name,damage,..} = unsafe{&game::OWNED_SPELLS[input]};
    {
        unsafe{
            game::ATTACKDAMAGE = 0;
            game::ATTACKDAMAGE += damage;}
    }
     unsafe{CANATTACK = true}; 
}

pub fn attackF(enemi:usize)
{
    unsafe{ 
    let enemies::enemy { isalive,..} = &game::VEC[enemi];
    {
        if *isalive
        {
            if enemi > game::VEC.len()
            {
                panic!("not an enemy");
            }
            if let Some(enemies::enemy{health ,..}) = game::VEC.get_mut(enemi)
            {
       
             let mut healt = health;
              *healt -= game::ATTACKDAMAGE;
             game::CANATTACK = false;
            }
        }else {
            println!("{}","ENEMY DEAD".bold().red())
        }
    }
    }
}

pub fn incombat()
{
    unsafe 
    {
        for x in 0..game::VEC.len()
        {
            
            let enemies::enemy { name,health,isalive,.. } = &mut game::VEC[x];
            {
                if *health > 0
                {
                    println!("{x}.{name} {health}");
                }else {
                  
                    println!("{x}.{name} (dead)");
                    *isalive = false;
                }
            }
            
        }
    }
}

pub fn talkF()
{

}

pub fn eatF()
{

}