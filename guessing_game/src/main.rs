use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::rng().random_range(1..=10);
    let stdin = io::stdin();

    loop {
        println!("Please input your guess.");

        let guess = loop {
            let mut user_input = String::new();
            if stdin.read_line(&mut user_input).is_ok() {
                if let Ok(val) = user_input.trim().parse::<u32>() {
                    break val;
                }
            } else {
                continue;
            }
        };

        println!("You guessed: {}.", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You got it!");
                break;
            }
        }
        println!("The secret number is: {secret_number}.");
    }
}
