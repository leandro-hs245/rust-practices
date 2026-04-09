use std::io;

fn main() {
    println!("Guess the number!");
    println!("Please input your guess: ");
    let mut guess = String::new(); // mut stand for mutable variable, otherwise it would be immutable by default
    io::stdin()
        .read_line(&mut guess)
        .expect("Couldn't read the guess");
    println!("You guessed {guess}");
}
