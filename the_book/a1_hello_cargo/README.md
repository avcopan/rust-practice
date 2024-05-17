Create the project:
```
cargo new <project name>
```

Build and run (2 steps):
```
cargo build
./target/debug/a1_hello_cargo
```
This does a debug build. To build for release, you would use `cargo build --release`.

Build and run (1 step):
```
cargo run
```
This will detect if the code has changed and, if so, compile before running.

Quickly make sure it compiles, without actually compiling:
```
cargo check
```
