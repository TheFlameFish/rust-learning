### General concepts:
* Ownership is a set of rules that define how a Rust program manages memory
* Stack and the heap
	* Stack: Push (add) to the top of the stack, pop (remove) from the top of the stack. Data with a known size at compile size (Ex. integers) is stored on the stack.
	* Heap: Less organized. Data that has an unknown size at compile size (Ex. `String`) is stored on the heap, with a pointer on the stack (additional data, like the length and capacity, may also be stored on the stack). Pointers are stored on the stack and tell the program how to find data that is on the heap.
* You can call `drop` on a variable to return its allocated memory to the operating system. `drop` is called automatically when a variable goes out of scope, so I probably needn't worry too much about this.

### Moving & Cloning
For some data types (types that do not have the `Copy` trait), setting a variable to another variable results in *moving* instead of copying. When you assign an integer variable, which does have the `Copy` trait, to another integer variable, it copies the data.
```rust
fn main() {
	let a = 5;
	let b = a; // `b` is a copy of `a`, therefore `b` = 5

	println!("A: {a}"); // A: 5
	println!("B: {b}"); // B: 5
}
```
On the other hand, Strings are not automatically copied and are instead moved.
```rust
fn main() {
	let a = String::from("Hello");
	let b = a; // `a` is moved to `b`. `a` is no longer valid and an error will be thrown if I try to use it.

	println!("{b}, world!");
}
```
If you do want to deeply copy a variable like this, you may do so via the `clone` method.
```rust
fn main() {
	let a = String::from("Hello");
	
	let mut b = a.clone();
	b.push_str(", world!");

	println!("{a}, world!");
	println!("{b}");
}
```

### Function arguments
Passing variables into a function will either move or clone them, like assignment. If you want to keep ownership of a variable when passing it into a function, you may return it back or use borrowing.
```rust
fn main() {
	let word = String::from("Cool");
	let number = 24_i32;

	foo(word, number);

	println!("`number`, on the other hand, is cloned; `main` can still use it.");
	println!("`number` is {number}");
}

fn foo(string: String, mut integer: i32) {
	println!("Foo got:");
	println!("\tstring == {string}");
	println!("\tinteger == {integer}");

	integer += 1;
	println!("\tinteger += 1. integer == {integer}\n");

	println!("Function `foo` now has ownership of `word` and `main` cannot use it.");
}
```

### Borrowing References
_Note: I'm not entirely sure of the difference between referencing and borrowing and as such will use them interchangeably. I am likely wrong in doing so._

If you want to pass a variable to a function without cloning or taking ownership of it, you can use borrowing.

##### Borrowing example from [[THE BOOK]]:
```rust
fn main() {
	let s1 = String::from("hello");
	let len = calculate_length(&s1);
	println!("The length of '{s1}' is {len}.");
}

fn calculate_length(s: &String) -> usize {
	s.len()
} // `s` goes out of scope here, but the `String` it refers to is not dropped because the function does not have ownership of it.
```

![[Figure 4-5.jpg]]

It should be noted that references are not, by default, mutable. If you want a mutable reference, you must borrow using `&mut {type}`. There can only be one mutable reference to a given piece of data in a given scope, otherwise the compiler will complain. You can also not have mutable and immutable references coexist within a given scope.
##### Mutable borrowing example adapted from [[THE BOOK]]:
```rust
fn main() {
	let mut s = String::from("Hello");

	change(&mut s);

	println!("{s}");
}

fn change(some_string: &mut String) {
	some_string.push_str(", world!");
}
```

### Slice References
Another type of reference is a slice, which is a reference to a portion of a piece of data.
```rust
fn main() {
	let s = String::from("world, hello!");

	let word1 = &s[..5];
	let word2 = &s[7..12];

	println!("{word2}, {word1}!");
}
```

### Dereferencing
You can also dereference references with the `*` character, which I suppose gets you the data itself rather than a reference to the data? You need to do this to, for example, compare an integer to an integer that you borrowed.

**Doesn't work:**
```rust
let y = &x
5_u32 > y
```
**Need to do:**
```rust
let y = &x
5_u32 > *y
```
