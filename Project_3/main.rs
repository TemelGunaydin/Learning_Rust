use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Welcome to Guessing Game");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        let mut guess = String::new();
        println!("Enter your guess");
        io::stdin()
            .read_line(&mut guess)
            .expect("Enter a valid value");
        let guess: u32 = match guess.trim().parse() {
            Ok(number) => number,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("You guess lower"),
            Ordering::Equal => {
                println!("Game Over. You guessed correct number = {guess}");
                break;
            }
            Ordering::Greater => println!("You guess higher"),
        }
    }
}
