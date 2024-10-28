* Defined via the `fn` keyword.
* Naming convention:
	* Lower snake case
* It doesn't matter if you define your functions before or after they are called. It only matters that the function is in the same scope.
* You must declare the type of each parameter, as well as the return type if the function returns
* Functions may return via a `return` statement or implicitly via an expression at the end. The latter is more common.
	* See [[Statements & Expressions]].

###### Ex:
```rust
fn main() {
	println!("Hello, world!");
	another_function();
}

fn another_function() {
	println!("Another function.");
}
```

###### Return example:
```rust
fn main() { // In this case, everything in main is a statement
	let a = cool_function();

	let b = cooler_function(true);
	let c = cooler_function(false);

	println!("A: {a}");
	
	println!("\nB: {b}");
	println!("C: {c}");
}

fn cool_function() -> u32 {
	5_u32
}

fn cooler_function(boolean: bool) -> u32 {
	if boolean {
		return 1u32; // This is a statement. It returns 1u32
	}

	0u32 // This is an expression. It returns 0u32
}
```