use std::io;
use rand::Rng;

fn main() {
    let random_number = rand::thread_rng().gen_range(1..=100);
    println!("Guess the number!");

    loop {
        println!("Input your guess:");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");

        
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number.");
                continue; 
            }
        };
        if guess == random_number {
            println!("Correct guess!!");
            break; 
        } else if guess < random_number {
            println!("Too small");
        } else {
            println!("Too big");
        }
    }
}
