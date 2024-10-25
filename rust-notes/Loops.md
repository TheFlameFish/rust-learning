#### Syntax:
```rust
fn main() {
    let mut i: u32 = 0;
    println!("This'll only print once.");
    loop {
        print!("A");
        i += 1;
        if i == 10 {
            println!("!");
            break;
        }
    }
    println!("Loop done.")
}
```

#### Additional stuff
* As seen above, `break` can be used to exit the loop.
* `continue` can also be used to stop the current loop cycle and start the next one.
* Both of these functions are seen in [[Project - Guessing Game|Guessing Game]].