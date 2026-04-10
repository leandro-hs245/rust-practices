use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    /* Guess the number game */
    println!("Guess the number!");
    let secret_number = rand::rng().random_range(1..=100);
    loop {
        println!("Please input your guess: ");
        let mut guess: String = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Couldn't read the guess");
        let guess: u32 = match guess.trim().parse() {
            Ok(number) => number,
            Err(_) => continue,
        };
        println!("You guessed {guess}");
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("To small."),
            Ordering::Greater => println!("To big."),
            Ordering::Equal => {
                println!("You win.");
                break;
            }
        };
    }
}
