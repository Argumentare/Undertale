use crate::game;

#[derive(Debug)]
pub struct spell
{
        pub name:&'static str,
        pub damage:i32,
        pub description:&'static str,
        pub mana:i32,
        pub enoughmana:bool
}

pub const BITE:spell = spell{name:("bite"),damage:(20),description:("Bite your enenmy flash"), mana:(0),enoughmana:(false)};
pub const GHOSTGOSSIP:spell = spell{name:("ghostgossip"),damage:(100),description:("Bite your enenmy flash"), mana:(10),enoughmana:(false)};


pub fn starting_spell()
{
       unsafe{ &game::OWNED_SPELLS.push(BITE);
        &game::OWNED_SPELLS.push(GHOSTGOSSIP);}
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