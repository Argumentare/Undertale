use crate::game;

#[derive(Debug)]
pub struct spell
{
        pub name:&'static str,
        pub damage:i32,
        description:&'static str,
}

pub const BITE:spell = spell{name:("bite"),damage:(20),description:("Bite your enenmy flash")};
pub const GHOSTGOSSIP:spell = spell{name:("ghostgossip"),damage:(100),description:("Bite your enenmy flash")};


pub fn starting_spell()
{
       unsafe{ &game::OWNED_SPELLS.push(BITE);
        &game::OWNED_SPELLS.push(GHOSTGOSSIP);}
}