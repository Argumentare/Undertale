use crate::spells::{self, spell};
use rand::prelude::*;
use rand::{distr::{Distribution,StandardUniform}};
use std::any::type_name;
#[derive(Debug,PartialEq)]
pub enum loot
{
    
    Smallcoins(i32),
    Spell(&'static str),
    HealthPotion,
    ManaPotion,
}


impl Distribution<loot> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> loot {
        match rng.random_range(0..=3) { 
            0 => loot::Smallcoins(rng.random_range(1..=5)),
            1 => loot::HealthPotion,
            2 => loot::Spell(&spells::EXISTING_SPELLS[rng.random_range(0..spells::EXISTING_SPELLS.len())].name),
            _ => loot::ManaPotion,
        }
    }
}


pub fn randomize_loot() -> Vec<loot>
{
    let mut loot:Vec<loot> = Vec::new();
    let mut coins:i32 = 0;
    let mut health_potion_c:i32 = 0;
    let mana_p:loot = loot::ManaPotion;
    let health_p:loot = loot::HealthPotion;
    let mut mana_potion_c:i32 = 0;
    for n in rand::rng().random_iter::<i32>().take(rand::rng().random_range(1..=4))
    {
        let n = rand::rng().random::<loot>();    
        
        
    }
    
    
   return loot;
}