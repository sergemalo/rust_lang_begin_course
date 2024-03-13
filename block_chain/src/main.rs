//#[macro_use]
extern crate serde_derive;

use std::io;
use std::process;
use std::io::Write;

mod blockchain;

fn main() {

    let mut miner_addr = String::new();
    let mut difficulty = String::new();
    let mut choice = String::new();

    println!("Enter a miner address: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut miner_addr).expect("Failed to read address");

    println!("Enter a difficulty: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut difficulty).expect("Failed to read difficulty");
    let diff:u32 = difficulty.trim().parse().expect("Please type an integer!");

    println!("Generation genesis block!");
    let mut chain = blockchain::Chain::new(miner_addr.trim().to_string(), diff);

    loop {
        println!("Menu");
        println!("1) New Transaction");
        println!("2) Mine block");
        println!("3) Change Difficulty");
        println!("4) Change Reward");
        println!("0) Exit");
        print!("Enter your choice: ");
        io::stdout().flush().unwrap();
        choice.clear();
        io::stdin().read_line(&mut choice).expect("Failed to read Menu choice");

        match choice.trim().parse().unwrap() {
            0 => {
                println!("exiting!");
                process::exit(0);
            }
            1 => {
                let mut sender = String::new();
                let mut receiver = String::new();
                let mut amount = String::new();

                println!("Enter sender address: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut sender).expect("Failed to read sender");
                println!("Enter receiver address: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut receiver).expect("Failed to read receiver");
                println!("Enter amount: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut amount).expect("Failed to read amount");
                //let amount: f32 = amount.trim().parse().expect("Please type a number!");
                let res = chain.new_transaction(sender.trim().to_string(), 
                                                receiver.trim().to_string(), 
                                                amount.trim().parse().unwrap());
                match res {
                    true => println!("Transaction added successfully"),
                    false => println!("Transaction failed"),
                }
            }
            2 => {
                println!("Generating block");
                let res = chain.generate_new_block();
                match res {
                    true => println!("Block generated successfully"),
                    false => println!("Block generation failed"),
                }
            }
            3 => {
                let mut new_diff = String::new();
                println!("Enter new difficulty: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut new_diff).expect("Failed to read new difficulty");
                let new_diff: u32 = new_diff.trim().parse().expect("Please type a number!");
                let res = chain.update_difficulty(new_diff);
                match res {
                    true => println!("Block difficulty updated successfully"),
                    false => println!("Block difficulty update failed"),
                }
            }
            4 => {
                let mut new_reward = String::new();
                println!("Enter new reward: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut new_reward).expect("Failed to read new reward");
                let new_reward: f32 = new_reward.trim().parse().expect("Please type a number!");
                let res = chain.update_reward(new_reward);
                match res {
                    true => println!("Block reward updated successfully"),
                    false => println!("Block reward update failed"),
                }
            }
            _ => println!("Invalid option please retry"),
        }
    }
}
