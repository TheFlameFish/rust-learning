# Scalar Types
## Integers
#### Integer types table

| Length | Signed | Unsigned |
| ------ | ------ | -------- |
| 8-bit  | i8     | u8       |
| 16-bit | i16    | u16      |
| 32-bit | i32    | u32      |
| 64-bit | i64    | u64      |
| 128    | i128   | u128     |
| arch   | isize  | usize    |
The default type for an integer is an `i32`.
###### Signed vs Unsigned integers
* Unsigned: 5
* Signed: +5 (or -5 for negative)
#### Integer overflow
Integer overflow occurs when you try to set an integer higher than its max value. The way this is handled depends on what mode the project is compiled in.
##### Debug
The program will panic.
##### Release
If an integer exceeds its max value, it'll wrap around to its minimum value. EX:
The max value of a `u8` is `255`. If you try to set a `u8` to `266`, it'll wrap around to `0`. If you try to set it to `257`, it'll wrap around to 1.
#### General Number Literal Rules

##### Type Suffixes
Number literals allow type suffixes. For example:
```rust
fn main() {
    let num1 = 10u32;
    let num2: u32 = 15;

    println!("{sum}",sum = num1 + num2);
}
```

##### Different forms
You can write number literals in multiple forms if you want to for some reason.

| Number Literal  | Example     |
| --------------- | ----------- |
| Decimal         | 98_222      |
| Hex             | 0xff        |
| Octal           | 0o77        |
| Binary          | 0b1111_0000 |
| Byte ( u8 only) | b'A'        |

You may also have `_` in numbers. EX: `1_000 == 1000`

## Floating-Point types
* Rust has two float types: `f32` and `f64`.
* It defaults to `f64`.

### Basic Number Operations
Pretty much the same as in most other languages. Example from [[THE BOOK]] Chapter 3:
```rust
fn main() {
  // addition
  let sum = 5 + 10;
  // subtraction
  let difference = 95.5 - 4.3;
  // multiplication
  let product = 4 * 30;
  // division
  let quotient = 56.7 / 32.2;
  let truncated = -5 / 3; // Results in -1
  // remainder
  let remainder = 43 % 5;
}
```

Note that integer division truncates the quotient towards 0 to the nearest integer.

## Booleans
They're booleans. Fully lowercase. Example from [[THE BOOK]] Chapter 3:
```rust
fn main() {
	let t = true;

    let f: bool = false;
}
```

## Characters
Single characters. Example from [[THE BOOK]] Chapter 3:
```rust
fn main() {
	let c = 'z';
	let z: char = 'ℤ'; // with explicit type annotation
	let heart_eyed_cat = '😻';
}
```

* **Char literals are defined with single quotes `''`.**
* **String literals are defined with double quotes `""`.**
* Chars represent a Unicode Scalar Value

# Compound types
Rust has two primitive compound types: Tuples and Arrays.
## Tuples
Example from [[THE BOOK]] Chapter 3:
```rust
fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
}
```
* Specifying item types of a tuple is optional.
* A tuple has a fixed length.
* A tuple can have a mix of different values in it.

You can destructure a tuple via pattern matching. Example from [[THE BOOK]] Chapter 3:
```rust
fn main() {
	let tup = (500, 6.4, 1);
	let (x, y, z) = tup;
	println!("The value of y is: {y}");
}
```

You may also access a tuple's values by doing `tuple.index`, sort of like `array[index]`. Example from [[THE BOOK]] Chapter 3:
```rust
fn main() {
	let x: (i32, f64, u8) = (500, 6.4, 1);
	let five_hundred = x.0;
	let six_point_four = x.1;
	let one = x.2;
}
```

### Unit
Unit is the name given to a tuple which has the value `()`. If an expression does not return any other value, it will return unit.

## Arrays
Example:
```rust
fn main() {
    let a = [1.0, 1.5, 2.0];
    let b: [f64; 3] = [1.0, 1.5, 2.0]; // Optionally declare type and length
    let c = [1,1,1,1];
    let d = [1; 4]; // Shorthand of writing 1 four times

	println!("A = B: {}", a == b);
	println!("C = D: {}", c == d);

	println!("Element 0 of A: {}",a[0]);
	println!("Element 2 of B: {}",b[2]);
}
```
* Every element of an array must have the same type
* Arrays have fixed lengths
* Trying to access an index out of bounds will result in a runtime error

###### Vectors
"A vector is a similar \[to an array] collection type provided by the standard library that is allowed to grow or shrink in size."
