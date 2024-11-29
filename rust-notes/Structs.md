A struct, or structure, is a custom data type that lets you name and package together multiple related values that make up a meaningful group. Structs are similar to tuples, except their attributes are named rather than ordered.

### Defining a struct
A struct can be defined with the keyword `struct` as well as its attributes. Structs can be defined outside of functions.
_[[THE BOOK]] Listing 5-1_
```rust
struct User {
	username: String,
	email: String,
	sign_in_count: u64,
	active: bool,
}
```

### Instantiating a struct
_[[THE BOOK]] Listing 5-2_
```rust
let user1 = User {
	email: String::from("someone@example.com"),
	username: String::from("someusername123"),
	active: true,
	sign_in_count: 1,
}
```

### Using an instance of a struct
The attributes of a struct instance can be accessed via dot notation. If the instance is mutable, the attributes can be changed.
_[[THE BOOK]] Listing 5-3 slightly modified_
```rust
struct User {
	username: String,
	email: String,
	sign_in_count: u64,
	active: bool,
}

fn main() {
	let mut user1 = User {
		email: String::from("someone@example.com"),
		username: String::from("someusername123"),
		active: true,
		sign_in_count: 1,
	};

	user1.email = String::from("anotheremail@example.com");
	println!("{}", user1.email);
}
```

### Builder function
Instantiating a struct is an expression. As such, it can be the last expression in a function for the function to return an instance of the struct.
_[[THE BOOK]] Listing 5-4 & 5-5_
```rust
struct User {
	username: String,
	email: String,
	sign_in_count: u64,
	active: bool,
}

fn build_user(email: String, username: String) -> User {
	User {
		// Because the `email` field in the struct has the same name 
		// as parameter `email`, we can use this shorthand for `email: email`
		email, 
		username,
		active: true,
		sign_in_count: 1,
	}
}

fn main() {
	let user1 = build_user(
		String::from("someemail@example.com"),
		String::from("someusername123")
	);

	println!("User {} has signed in {} times.", user1.username, user1.sign_in_count);
}
```

### Struct update syntax
You can create a new struct instance using values from another struct instance using `..`
```rust
struct User {
	username: String,
	email: String,
	sign_in_count: u64,
	active: bool,
}

fn build_user(email: String, username: String) -> User {
	User {
		// Because the `email` field in the struct has the same name 
		// as parameter `email`, we can use this shorthand for `email: email`
		email, 
		username,
		active: true,
		sign_in_count: 1,
	}
}

fn main() {
	let user1 = build_user(
		String::from("someemail@example.com"),
		String::from("someusername123")
	);

	let user2 = User {
		username: String::from("someotherusername24"),
		..user1 // fills in all remaining fields with attributes of `user1`
	};

	println!("User {} has signed in {} times and has the email {}.", user2.username, user2.sign_in_count, user2.email);
}
```

### Tuple structs
You can also define a struct that behaves more like a tuple, with its fields being unnamed and ordered.
_[[THE BOOK]] example_
```rust
fn main() {
	struct Color(i32, i32, i32);
	struct Point(i32, i32, i32);

	let black = Color(0,0,0);
	let origin = Point(0,0,0);

	let Color(black_red, black_green, black_blue) = black;
	println!("The RGB values of black are {black_red} red, {black_green} green, and {} blue.", black.2);
}
```

### Methods
Methods are similar to functions, defined within a struct (or enum or trait object). Their first parameter is always `self`.  Note that Rust automatically turns rect1 into a reference to pass it into area(); this also works for mutable references and dereferencing.
_[[THE BOOK]] practice "Rectangles"_
```rust
struct Rectangle {
	width: u32,
	height: u32
}

impl Rectangle { // Implementation. There can be multiple impl blocks if desired.
	fn area(&self) -> u32 {
		self.width * self.height
	}
}

fn main() {
	let rect1 = Rectangle {width: 30, height: 50};
	println!("The area of the rectangle is {} square pixels.", rect1.area());
}
```

### Associated functions
You can also associate functions with a struct. Associated functions do not have a `self` parameter and are not linked to an instance of a struct.
_[[THE BOOK]] practice "Rectangles"_
```rust
#[derive(Debug)] // Allows the use of :? as shown on line 10
struct Rectangle {
	width: u32,
	height: u32
} 

impl Rectangle { // implementation
	fn square(length: u32) -> Rectangle {
		Rectangle {width: length, height: length}
	}
} 

fn main() {
	let square = Rectangle::square(50);
	println!("\nSquare with a side length of 50 pixels: {:#?}", square);
}
```
