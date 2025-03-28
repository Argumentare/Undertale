

use std::fmt::Debug;
use std::io::{self, Read};
use crate::entities::enemies;
use crate::main;


const DEBUGING:bool = true;


static  mut vec:Vec<enemies> = Vec::new();
pub enum undertale
{
    
}

pub fn callenemy()
{
    
    let goblin = enemies::spawnenemy(String::from("goblin"));    
    println!("{:?}",goblin);
    Addenemytovec(goblin);
    
}

pub fn Addenemytovec(enemy:enemies)
{
    
    unsafe{vec.push(enemy);
    println!("{:?}",vec);}
}

impl undertale
{
    
    pub fn run()
    {
       
        loop 
        {
            if DEBUGING
            {
                let mut input = String::new();
                
                io::stdin().read_line(&mut input).expect("wrong input");
                let input:i32 = input.trim().parse().expect("not a number");
                match  input {
                1 => callenemy(),   
                _ => (),
                }
            }
        }
    }
}