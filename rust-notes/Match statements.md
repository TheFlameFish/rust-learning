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

