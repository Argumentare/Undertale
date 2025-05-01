use crate::game;
use rand::prelude::*;
use rand::distr::{Distribution,StandardUniform};
#[derive(Debug,PartialEq)]
pub struct spell
{
        pub name:&'static str,
        pub damage:i32,
        pub description:&'static str,
        pub mana:i32,
        pub enoughmana:bool,
}

pub const BITE:spell = spell{name:("bite"),damage:(45),description:("Bite your enenmy flash"), mana:(0),enoughmana:(false)};
pub const GHOSTGOSSIP:spell = spell{name:("ghostgossip"),damage:(100),description:("Bite your enenmy flash"), mana:(10),enoughmana:(false)};

pub const EXISTING_SPELLS:[spell;2] = [BITE,GHOSTGOSSIP]; 


pub fn starting_spell()
{
       unsafe{ &game::OWNED_SPELLS.push(BITE);}
}

pub fn check_for_mana(spell:&spell) -> bool
{
        if unsafe{game::MANA} < spell.mana
        {
                return false;
        }else {
            return true
        }
}