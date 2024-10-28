##### Naming Convention:
Rust uses snake case. Ex: `let rust_uses_snake_case = true`

##### Mutable vs immutable
```rust
let mut guess = String::new(); // This is a mutable variable of type String

let input_handle = io::stdin(); // This is an immutable variable of type io::Stdin
```

* guess is mutable. It can be changed
* input_handle is immutable. It cannot be changed

##### Constants
```rust
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
```
* Constant declaration syntax:
	* Use `const` instead of `let`
	* Naming convention:
		* Full caps
		* Underscores where there would be spaces
	* The type must be annotated
* Constants are processed at compilation rather than at runtime. The above constant is the same as saying `const THREE_HOURS_IN_SECONDS: u32 = 10800`.
	* This means that only certain operations are allowed. 
	  These operations are detailed [here](https://doc.rust-lang.org/reference/const_eval.html).
* Constants can be declared in any scope, including global.
##### Statically typed
Rust is statically typed. The compiler can infer a variable's type upon creation via the value it gets and how it's used. The following code will make two variables, both with the same type and value:
```rust
let a = "foo" // Rust assumes this is a string
let b: str = "foo"
```

However, once a variable is made its type cannot be changed (without shadowing?). The following code will not work, as you cannot add an i32 and a u32:
```rust
fn main() {
    let a: i32 = 1; // This is a variable of type i32
    let b: u32 = 1; // This is a variable of type String

    let c = a + b;
    println!("{c}");
}
```

##### Shadowing
```rust
fn main() {
    let x = 5;
    let x = x + 1; // This is a new variable, shadowing the old one
    {
        let x = x * 2; /* This is a new variable, shadowing the old one but
                       only in this scope */
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x is: {x}");
}
```
* Note the use of `let`. The variable is not being modified; a new variable is being created. 


