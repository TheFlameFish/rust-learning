# Cargo project structure
* `/` *(project root)*
	* `src/`
		* *Contains the project's source code* 
	* `target/`
		* `debug/`
			* *Files pertaining to the debug build of the project, including an executable*
		* `release/`
			* *Files pertaining to the release build of the project, including an executable*
	* `cargo.toml`
	  *A toml file specifying the cargo configuration of the project*
	* `cargo.lock`
	    *A file automatically generated by Cargo, not intended to be manually edited*

# Project creation
```bash
cargo project_name
cd project_name
```

# Project building
### Build a project in debug mode
```bash
cargo build
```

Executable will be stored in `./target/debug`

### Build a project for release

```bash
cargo build --release
```
The project takes longer to build but will run more efficiently. Executable will be stored in `./target/release`

# Project running
## Compile and run the project with Cargo
```bash
cargo run
```
If the code has changed since the last build the project will be automatically rebuilt
## Run manually
#### Debug
```bash
./target/debug/project_name
```
#### Release
```bash
./target/release/project_name
```

# Project check
`cargo check` builds the project without producing an executable file.