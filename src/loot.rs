use crate::spells::{self, spell};
use rand::prelude::*;
use rand::{distr::{Distribution,StandardUniform}};
use std::any::type_name;
use std::ffi::NulError;
use std::ops::{Add, AddAssign};
#[derive(Debug,PartialEq)]
pub enum loot
{
    
    Smallcoins(i32),
    Spell(&'static str),
    HealthPotion,
    ManaPotion,
    Null,
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

impl ToString for loot
{
    fn to_string(&self) -> String {
        match self
        {
            loot::HealthPotion => "HealthPotion".to_string(),
            loot::ManaPotion => "ManaPotion".to_string(),
            loot::Smallcoins(_) => "Smallcoins".to_string(),
            loot::Spell(_) => "Spell".to_string(),
            loot::Null => "".to_string(),
        }
    }
}

impl Add for loot
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        match (self,rhs)
        {
            (loot::Smallcoins(a),loot::Smallcoins(b)) => loot::Smallcoins(a + b),
            _ => loot::Null,
        }
    }
}


pub fn randomize_loot() -> Vec<loot>
{
    let mut loot:Vec<loot> = Vec::new();
    let mut coins:loot = loot::Smallcoins(0);
    let mut health_potion_c:i32 = 0;
    let mut mana_potion_c:i32 = 0;
    for n in rand::rng().random_iter::<i32>().take(rand::rng().random_range(1..=4))
    {
        let n = rand::rng().random::<loot>();    
       
       
        match &n.to_string().as_str() {
            &"HealthPotion" => health_potion_c += 1,
            &"ManaPotion" => mana_potion_c += 1,
            &"Smallcoins" => coins = coins + n,
            &"Spell" => (),
            &_ => (),
        }
        println!("{}",health_potion_c);
        println!("{}",mana_potion_c);
        println!("{:?}",coins);
        loot.push(&n);
    }
    
    
   return loot;
}