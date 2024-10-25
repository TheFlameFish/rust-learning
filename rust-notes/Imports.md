```rust
use std::io;
use std::cmp::Ordering;
use rand::Rng;
```

* Imports required libraries:
	* `std::io`
		* Input and output. It is in the std crate, which is available within any project.
	* `std::cmp::Ordering`
		* Ordering. It gives the ability to compare stuff with the cmp function.
	* `rand::Rng`
		* This is an external crate specified to cargo in the Cargo.toml file 
		  (View: [[Cargo#Dependencies]])