use rand::RngExt;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("This is a Number guessing game");

    let mut rng = rand::rng();

    let secret_number = rng.random_range(1..=100);

    // println!("The secret number is {secret_number}");

    loop {
        println!("Input your guess");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Could not get the guess");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => {
                println!("You Win!");
                break;
            }
        }

        println!("The number you guessed is {guess}");
    }
}
