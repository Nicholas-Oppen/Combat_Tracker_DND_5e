use std::{io, string};
use std::io::Read;
use combat_types::{Damage, DamageCategory, Entity};
use log::debug;
use crate::combat_types::Attack;

#[allow(unused)]

mod combat_types;

fn main() 
{
    //let mut buffer = String::new();
    //io::stdin().read_line(&mut buffer);
    //let mut console_input: String = String::new();
    //let mut continue_running: bool = true;
    /*
    while continue_running
    {
        io::stdin().read_line(&mut console_input);

        if console_input == "create entity"
        {

        }

    }
    */
    //create_entity();


}

//Add error handling, this shit will hard panic the instant anything goes wrong
//THIS IS EXREMELY UNSAFE
//BAD CODE AHEAD
fn create_entity() //-> Entity
{
    let mut user_input = String::new();

    println!("Is the entity a player?: y/n");
    io::stdin().read_line(&mut user_input);
    let name = &user_input[0..user_input.len()];

    println!("Is the entity a player?: y/n");
    io::stdin().read_line(&mut user_input);
    let is_player: bool = (user_input == "y\r\n") || (user_input == "Y\r\n");

    println!("How many hit points does it currently have?: enter a positive integer");
    user_input = "".to_string();
    io::stdin().read_line(&mut user_input);
    let current_hit_points: i32 = user_input[0..user_input.len()-2].parse().unwrap();

    println!("What is its maximum hit points?: enter a positive integer");
    user_input = "".to_string();
    io::stdin().read_line(&mut user_input);
    let maximum_hit_points: i32 = user_input[0..user_input.len()-2].parse().unwrap();

    println!("What are its temporary hit points?: enter a positive integer");
    user_input = "".to_string();
    io::stdin().read_line(&mut user_input);
    let temporary_hit_points: i32 = user_input[0..user_input.len()-2].parse().unwrap();

    println!("What is its armor class?: enter a positive integer");
    user_input = "".to_string();
    io::stdin().read_line(&mut user_input);
    let ac: i32 = user_input[0..user_input.len()-2].parse().unwrap();

    let mut more_attacks = true;

    let attacks = create_attack_vec();
}

fn create_attack_vec() -> Vec<Attack>
{
    let mut user_input = String::new();

    let more_attacks: bool;

    let attacks: Vec<combat_types::DamageCategory> = vec![];
    while more_attacks
    {
        println!("What is the name of this attack?:");
        user_input = "".to_string();
        io::stdin().read_line((&mut user_input));
        let mut attack_name = &user_input[0..user_input.len() - 5];
        dbg!(attack_name);

        println!("What is the Damage Type of this attack?");
        user_input = "".to_string();
        io::stdin().read_line(&mut user_input);
        user_input = String::from(user_input[0..user_input.len()-2]);



        println!("Is there another attack?: y/n");
        user_input = "".to_string();
        io::stdin().read_line(&mut user_input);
        more_attacks = (user_input == "y\r\n") || (user_input == "Y\r\n");
    }
    return vec![];
}

fn create_damage_category()
{

}