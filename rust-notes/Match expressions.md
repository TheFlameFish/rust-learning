---
aliases:
  - match
---
A match expression may be used on an enum to have behaviour specific to certain enum variants. There must be a match arm for every variant of an enum (unless they are covered by a default arm).

Syntax:
```rust
match enum_variant {
	variant_1(data) => {
		println!("This code will be executed if the enum variant is variant_1!");
	},
	variant_2(data) => println!("Got variant_2 with data: {data}.")
	_ => println!("This'll execute if enum_variant is not variant_1 or variant_2")
}
```

```rust
fn main() {
	let die_roll = 5;
	match die_roll {
		3 => {println!("You rolled 3! Special event A.");},
		7 => {println!("You rolled 7! Special event B.");},
		other => {println!("You rolled {other}! Move {other} spaces.")}
	}
}
```

### EX:
```rust
fn main() {
	let guess = "50";

	let guess: u32 = match guess.parse() {
		Ok(num) => {
			println!("Yay.");
			num  // Return the parsed number to guess
		},
		Err(_) => {
			println!("Please input a valid number.");
			return;  // Exit main function if parsing fails
		}
	};

	println!("Parsed guess is: {}", guess);
}
```
The above code demonstrates error handling. It'll try to parse str guess as a u32. If that fails (If, for example, guess == "waffle"), it'll print "Please input a valid number." and return.

Example, [[THE BOOK]] listing 6-3:
```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn main() {
	let my_coin = Coin::Penny;

	println!("You have ¢{}", value_in_cents(my_coin));
}
```

