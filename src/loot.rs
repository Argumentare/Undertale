use crate::spells::spell;
use rand::prelude::*;
pub enum loot
{
    SmallLoot
    {
        smallcoins:i32,
        spells:spell,
        health_potion:i32,
        mana_potion:i32,
    }
}

pub fn randomize_loot() -> Vec<loot>
{
    let loot:Vec<loot> = Vec::new();
    let rand_num:i32 = rand::rng().random_range(0..4);
    match rand_num {
        1 => (),
        2 => (),
        3 => (),
        4 => (),
        _ => (),
    }
    return loot;
}