use std::io;

fn main() {
    let input_handle = io::stdin();

    let target_unit: String = loop {
        let mut input = String::new();

        println!("Would you like to convert from Fahrenheit to Celsius (C) or Celsius to Fahrenheit (F)?");
        input_handle.read_line(&mut input)
            .expect("Failed to read input.");

        let input: char = match input.trim().to_lowercase().parse() {
            Ok(character) => character,
            Err(_) => {
                println!("Please input a valid input.");
                continue
            }
        };

        let target_unit: String = 
            if input == 'c' {"Celsius".to_string()} 
            else if input == 'f' {"Fahrenheit".to_string()}
            else { // If input is invalid
                println!("Please input a valid input.");
                continue
            }; 

        break target_unit;
    };

    println!("What temperature would you like to convert to {target_unit}?");

    let temp: f64 = loop {
        let mut input = String::new();

        input_handle.read_line(&mut input).expect("Failed to read line");

        let temp: f64 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please input a valid number.");
                continue
            }
        };

        break temp;
    };

    if target_unit == "Fahrenheit" {
        println!("Your temperature: {}°F",celsius_to_fahrenheit(temp));
    } else if target_unit == "Celsius" {
        println!("Your temperature: {}°C", fahrenheit_to_celsius(temp));
    }
}

fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) / 1.8
}

fn celsius_to_fahrenheit(c: f64) -> f64 {
    c * 1.8 + 32.0
}
