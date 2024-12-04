`null` does not exist in Rust. Instead, the `Option<t>` enum may be used as follows:
```rust
fn main() {
    let some_number = Some(5); // This is of type Option<i32>
    let some_char = Some('e');

    let absent_number: Option<i32> = None;
}
```

You can unwrap a `some` value with the `unwrap` method, although it is not recommended as it will fail if used on a `none`:
```rust
let x = Some("air");
assert_eq!(x.unwrap(), "air");
```
You can use `unwrap_or_default` to unwrap a `some` value or the default value of `T` if passed a `none`.
```rust
let x: Option<u32> = None;
let y: Option<u32> = Some(12);

assert_eq!(x.unwrap_or_default(), 0);
assert_eq!(y.unwrap_or_default(), 12);
```
