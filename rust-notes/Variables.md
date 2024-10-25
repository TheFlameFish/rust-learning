##### Mutable vs immutable
```rust
let mut guess = String::new(); // This is a mutable variable of type String

let input_handle = io::stdin(); // This is an immutable variable of type io::Stdin
```

* guess is mutable. Its value can be changed
* input_handle is immutable. Its value cannot be changed.

##### Types
Rust seems to be *kind of* strongly typed. The following code will make two variables, both with the same type and value:
```rust
let a = "foo" // Rust assumes this is a string
let b: String = "foo"
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

