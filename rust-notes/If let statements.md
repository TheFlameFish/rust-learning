`if let` can be used similarly to a match statement, but only using one variant.
For example, the following can be done with match statements ([[THE BOOK]] Listing 6-6):
```rust
fn main() {
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {max}"),
        _ => (),
    }
}
```
But specifying `_ => ()` is a bit annoying. You can instead use an `if let` statement as follows:
```rust
fn main() {
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    }
}
```

`if let` statements may also have else statements:
```rust
fn main() {
    let config_max: Option<u8> = None;
    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    } else {
	    println!("No maximum configured.");
    }
}
```