use std::io;
use rand::RngExt;

fn main() {
    println!("This is a Number guessing game");

    let mut rng = rand::rng();
    let secret_number = rng.random_range(1..=100);

    println!("The secret number is {secret_number}");

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
        std::cmp::Ordering::Less => println!("Too small!"),
        std::cmp::Ordering::Greater => println!("Too Big!"),
        std::cmp::Ordering::Equal => {
            println!("You Win!");
            break;
        }
    }

    println!("The number you guessed is {guess}");

    }
   
}