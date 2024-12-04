- Enums are custom data types that have a list of possible variants
From [[THE BOOK]] Ex, IP adress version:
```rust
enum IpAddrKind {
    V4,
    V6,
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

	route(IpAddrKind::V4);
    route(IpAddrKind::V6);
}

fn route(ip_kind: IpAddrKind) {
	println!("Routing!");
}
```

Each enum variant becomes a function which constructs an instance of an enum. The enum variants can also take arguments.

```rust
enum IpAddr {
	V4(String),
	V6(String),
}

let home = IpAddr::V4(String::from("127.0.0.1"));

let loopback = IpAddr::V6(String::from("::1"));
```

You may also put structs within enum variants, as shown by the following code snippet from the standard definition ip address library:
```rust
struct Ipv4Addr {
    // --snip--
}

struct Ipv6Addr {
    // --snip--
}

enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}
```

You may also give an enum methods, as shown by listing 6-2 from [[THE BOOK]]:
```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
	fn call(&self) {
		// method body would be defined here
	}
}

fn main() {
	let m = Message::Write(String::from("hello"));
	m.call();
}
```