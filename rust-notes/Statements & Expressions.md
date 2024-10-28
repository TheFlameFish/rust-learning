• Statements are instructions that perform some action and do not return a value. They end in semicolons.
• Expressions evaluate to a resultant value. They do not end in semicolons. Expressions can be within statements. 

`let a = 5 + 2;` is a statement. Within it, `5 + 2` is a statement.

Example [[Functions|function]]:
![[Functions#Return example]]


A scope block is also an expression:
```rust
fn main() {
	let y = {
		let x = 3;
		x + 1
	};
	println!("The value of y is: {y}");
}
```
