use std::io;
use rand::Rng;
use std::cmp::Ordering;
use std::env;

fn main() {
    let args: Vec<_> = env::args().collect();
    let limit: u32 = match args.get(1) {
        Some(num) => {
            num.parse().expect("Enter a number for an argument!")
        },
        None => 100
    };

    println!("Guess a number! (1 - {limit})");
    let secret_number = rand::thread_rng().gen_range(1..=limit);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number!");
                continue;
            }
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}