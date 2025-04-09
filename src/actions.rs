use core::panic;
use std::io;
use crate::entities::enemies;
use crate::game;
use crate::entities;
use crate::game::ATTACKDAMAGE;
use crate::game::CANATTACK;
use crate::game::VEC;
use std::str::FromStr;
use crate::undertale;




#[derive(Debug,PartialEq,Eq)]
pub enum Actions
{
    attack,
    run,
    talk,   
    eat,
    donothing,
}

pub trait ActionFrstr {
    fn action_from_string(string:String) -> Actions;
}

impl ActionFrstr for Actions
{
    fn action_from_string(string:String) -> Actions {
        
        
        match string.as_str().trim()
        {
            
            "run" => Actions::run,
            "eat" => Actions::eat,
            "talk" => Actions::talk,
            "atack" => Actions::attack,
            &_ => Actions::donothing,
                
        }
    }
}
impl Actions
{
    pub fn takeaction(action:Actions)
    {
        match action
        {
            Actions::attack => unsafe{CANATTACK = true},
            Actions::run => runf(),
            Actions::eat => eatF(),
            Actions::talk => talkF(),
            Actions::donothing => (),

        }

    }
}



pub fn runf()
{
    std::process::exit(0);
}
pub fn attackF(enemi:usize)
{

    unsafe{
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
    }
}

pub fn incombat()
{
    unsafe 
    {
        for x in 0..game::VEC.len()
        {
            let enemies::enemy { name,health,.. } = &game::VEC[x];
            {
                println!("{x}.{name} {health}");
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