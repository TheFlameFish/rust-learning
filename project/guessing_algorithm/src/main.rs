// A simple script that plays a simple number guessing game with itself.
// Takes an average of ~5.990099 guesses to get the correct guess
// Struggles with 100

use rand::{thread_rng, Rng};

fn main() {
    let mut rng = thread_rng();

    let mut min_tries: u32 = 1000;
    let mut max_tries: u32 = 0;

    let mut tries: Vec<u32> = Vec::new();

    for _i in 1..5000 {
        guess_loop(rng.gen_range(0..=100), &mut min_tries, &mut max_tries, &mut tries);
    }

    // Get an average number of tries.
    let mut sum: u32 = 0;
    for number in &tries {
        sum += number;
    }

    let average: f32 = (sum as f32) / (tries.len() as f32);

    println!("\nFinished running. Here are some stats:");
    println!("Highest number of tries needed:\t{max_tries}");
    println!("Lowest number of tries needed:\t{min_tries}");
    println!("Average number of tries needed:\t{average}");
}

#[test]
fn test() {
    let mut min_tries: u32 = 1000;
    let mut max_tries: u32 = 0;

    let mut tries: Vec<u32> = Vec::new();

    for i in 0..=100 {
        let guess = guess_loop(i, &mut min_tries, &mut max_tries, &mut tries);
        assert_ne!(guess, 124, "If this fails, the loop most likely took too long.");
        assert_eq!(guess, i, "The guess was not correct.");
    }

    // Get an average number of tries.
    let mut sum: u32 = 0;
    for number in &tries {
        sum += number;
    }

    let average: f32 = (sum as f32) / (tries.len() as f32);

    println!("\nFinished running. Here are some stats:");
    println!("Highest number of tries needed:\t{max_tries}");
    println!("Lowest number of tries needed:\t{min_tries}");
    println!("Average number of tries needed:\t{average}");
}

fn guess_loop(answer: u32, min_tries: &mut u32, max_tries: &mut u32, tries: &mut Vec<u32>) -> u32 {
    println!("The number is {answer}.");

    let mut upper: f32 = 100.0;
    let mut lower: f32 = 0.0;

    let mut i: u32 = 0;
    loop {
        if i > 1000 { // Something has gone wrong. Return a guess outside the possible range.
            return 124; 
        }

        i += 1;
        let guess = (lower + upper) / 2.0;
        let result = evaluate_guess(guess as u32, &answer);

        if result == 0 {
            lower = guess as f32;
            println!("Answer is larger than {guess}.");
        } else if result == 2 {
            println!("Answer is smaller than {guess}.");
            upper = guess as f32;
        } else {
            println!("Found answer in {i} iterations! Correct is {}.", guess as u32);

            if i < *min_tries {
                *min_tries = i;
            }
            if i > *max_tries {
                *max_tries = i;
            }
            tries.push(i);

            return guess as u32;
        }
    }
}

/// Takes a `guess` and the `correct` answer as unsigned 32-bit integers and returns `0` if too small, `1` if correct, and `2` if too large. 
fn evaluate_guess(guess: u32, correct: &u32) -> u32 {
    if guess > *correct {
        2
    } else if guess < *correct {
        0
    } else {
        1
    }
}