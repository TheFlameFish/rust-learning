# If expressions
* Declared with keyword `if`.
* Condition does not need to be in parentheses.
* Code block (or arm) must be in curly brackets
* You can use `else if` for else if statements (wow! /s).
* **Important note:** If expressions are expressions, not statements. They can have a return value. Ex: `let a = if true { 1 } else { 0 }`
	* If an if statement returns a value, all arms must return the same type of value.

### If expression example:
```rust
fn main() {
	let number = 3;
	
	if number < 5 {
		println!("condition was true");
	} else {
		println!("condition was false");
	}
}
```

# Loops
* `break` ends the loop.
* `continue` ends the current loop iteration and moves on to the next one.
* Loops can also return with a break statement. Ex: `break 24u32;` 
* If you have loops within loops, you can label loops to be able to reference a specific one in `break`. 
	* Ex:
  ```rust
	  fn main() {
		  'cool_label: loop {
			  loop {
				  break 'cool_label;
			  }
		  }
	  }
	```
## `loop` loops
The standard `loop` acts as a while true loop. Ex:
```rust
fn main() {
	let mut i = 0;
	loop {
		print!("A");
		i += 1;
		if i == 50 { break }
	}
	println!("H!");
}
```

## `while` loops
Pretty standard. Ex:
```rust
fn main() {
	let mut number = 3;
	while number != 0 {
		println!("{number}!");
		number -= 1;
	}
	println!("LIFTOFF!!!");
}
```

## `for` loops
Also pretty standard. Ex:
```rust
fn main() {
	let a = [10, 20, 30, 40, 50];
	for element in a {
		println!("the value is: {element}");
	}
}
```

With four loops, you can also iterate a given number of time using a range.
```rust
fn main() {
	for number in (1..4).rev() {
		println!("{number}!");
	}
	println!("LIFTOFF!!!");
}
```
