use std::io;

fn main() {
    println!("Guess your number!");

    println!("Please input your guess.");

    let mut guess = String::new(); // This is a mutable variable of type String

    let input_handle = io::stdin(); // This is an immutable variable of type io::Stdin

    input_handle
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}"); /* Kind of like an f-string. 
    An alternative way to do this would be ("You guessed: {}", guess) */
}
