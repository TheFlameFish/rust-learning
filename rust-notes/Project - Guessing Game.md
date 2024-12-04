---
aliases:
  - Guessing Game
---
# Full code:
```rust
use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess your number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    let mut attempts: u32 = 0;

    let one: u32 = 1; // There's... probably a better way to do this...

    loop {
        println!("\nPlease input your guess.");

        let mut guess = String::new(); // This is a mutable variable of type String

        let input_handle = io::stdin(); // This is an immutable variable of type io::Stdin

        input_handle
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please input a valid number.");
                continue
            }
        };

        attempts += 1;

        println!("You guessed: {guess}"); /* Kind of like an f-string. 
        An alternative way to do this would be ("You guessed: {}", guess) */

        match guess.cmp(&secret_number) {  // Match takes an enum variant and runs code depending on what it is.
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                println!("It took you {attempts} attempt{do_s}.", do_s = match attempts.cmp(&one) {
                    Ordering::Greater => "s",
                    _ => "", // Default
                });
                break;
            },
        }
    }
}
```

# Concepts
#### Imports
![[Imports]]


#### Variables
See [[Variables]]

#### Match statements
![[Match expressions]]

#### Loops
![[Control Flow#Loops]]
