use crate::{game,spells::{self,BITE,GHOSTGOSSIP}};
use rand::prelude::*;
use crate::game::OWNED_SPELLS;
use rand::{distr::{Distribution,StandardUniform}};
use std::fmt;
use std::ops::{Add, AddAssign};
#[derive(Debug,PartialEq,Clone, Copy)]
pub enum loot
{
    
    Smallcoins(i32),
    Spell(&'static str),
    HealthPotion(i32),
    ManaPotion(i32),
    Null,
}


impl Distribution<loot> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> loot {
        match rng.random_range(0..=3) { 
            0 => loot::Smallcoins(rng.random_range(1..=5)),
            1 => loot::HealthPotion(1),
            2 => loot::Spell(&spells::EXISTING_SPELLS[rng.random_range(0..spells::EXISTING_SPELLS.len())].name),
            _ => loot::ManaPotion(1),
        }
    }
}

impl ToString for loot
{
    fn to_string(&self) -> String {
        match self
        {
            loot::HealthPotion(a) => "HealthPotion ".to_string() + " " + a.to_string().as_str(),
            loot::ManaPotion(a) => "ManaPotion".to_string() + " " + a.to_string().as_str(),
            loot::Smallcoins(a) => "Smallcoins".to_string() + " " + a.to_string().as_str(),
            loot::Spell(a) => "Spell".to_string() + " " + a.to_string().as_str(),
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
            (loot::HealthPotion(a), loot::HealthPotion(b)) => loot::HealthPotion(a+b),
            (loot::ManaPotion(a), loot::ManaPotion(b)) => loot::ManaPotion(a+b),
            _ => loot::Null,
        }
    }
}




pub fn randomize_loot(lootmultiplier:usize) -> Vec<loot>
{
    let mut loot:Vec<loot> = Vec::new();   
    let mut spell_c = 0;
    let mut coins:loot = loot::Smallcoins(0);
    let mut canprint_c = false;
    let mut mana_potions:loot = loot::ManaPotion(0);
    let mut canprint_m_p = false;
    let mut health_potions:loot = loot::HealthPotion(0);
    let mut canprint_h_p = false;
    for n in rand::rng().random_iter::<i32>().take(rand::rng().random_range(1..=4))
    {
        let n = rand::rng().random::<loot>();    
        let mut compare:Vec<char> = Vec::new();
        for x in n.to_string().chars()
        {
            if x == ' '
            {     
                break;
            }
            compare.push(x);
        }
        match compare.iter().collect::<String>().as_str() {
            "HealthPotion" => {health_potions = health_potions + n;
                                canprint_h_p = true;
                                unsafe{if game::HEALTH < 10
                                {
                                game::HEALTH += 1;
                                }}},
                                
            "ManaPotion" => {mana_potions = mana_potions + n;
                              canprint_m_p = true;
                              unsafe{if game::MANA < 100
                                {
                                game::MANA += 10;
                                }}},
            "Smallcoins" => {coins = coins + n;
                              canprint_c = true;
                              unsafe{game::COINS += 1;}},
            "Spell" => unsafe{
            'outer:while spell_c < 1
                {
                for x in 0..OWNED_SPELLS.len()
                {
                    
                    if &OWNED_SPELLS[x].name == &n.to_string().as_str()
                    {
                        break 'outer;
                    }
                }
                match &n.to_string().as_str().trim() {
                    &"Spell bite" => game::OWNED_SPELLS.push(BITE),
                    &"Spell ghostgossip" => game::OWNED_SPELLS.push(GHOSTGOSSIP),
                    &_ => (),
                }
                loot.push(n);           
                spell_c +=1;
                }},
            &_ => (),
        }
        
    }  

    if canprint_c
    {
        loot.push(coins);
    }
    if canprint_m_p
    {
        loot.push(mana_potions);
    }
    if canprint_h_p
    {
        loot.push(health_potions);
    }
   return loot;
}