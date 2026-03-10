# Step into become a rustacean

> following: [rust book](https://doc.rust-lang.org/stable/book/)

## Project Directory Setup

You’ll start by making a directory to store your Rust code. It doesn’t matter to Rust where your code lives, but for the exercises and projects in this book, we suggest making a projects directory in your home directory and keeping all your projects there.

Open a terminal and enter the following commands to make a projects directory and a directory for the “Hello, world!” project within the projects directory.

For Linux, macOS, and PowerShell on Windows, enter this:

```bash
mkdir ~/projects
cd ~/projects
mkdir hello_world
cd hello_world
```

Rust Program Basics
Create and open the main.rs file.

Filename: main.rs

```rust
fn main() {
    println!("Hello, world!");
}
```

Execute:

```bash
rustc main.rs
./main
Hello, world!
```

> println! calls a Rust macro. If it had called a function instead, it would be entered as println (without the !). Rust macros are a way to write code that generates code to extend Rust syntax.

## Hello, Cargo!

Cargo is Rust’s build system and package manager. Most Rustaceans use this tool to manage their Rust projects because Cargo handles a lot of tasks for you, such as building your code, downloading the libraries your code depends on, and building those libraries. (We call the libraries that your code needs dependencies.)

```
cargo --version
```

### Creating a Project with Cargo

```bash
cargo new hello_cargo
cd hello_cargo
```

### Building and Running a Cargo Project

```bash
cargo build
   Compiling hello_cargo v0.1.0 (file:///projects/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 2.85 secs

```

```bash
./target/debug/hello_cargo # or .\target\debug\hello_cargo.exe on Windows
Hello, world!

```

`cargo run` to compile the code and then run the resultant executable all in one command:

```bash
cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/hello_cargo`
Hello, world!
```

Cargo also provides a command called `cargo check`. This command quickly checks your code to make sure it compiles but doesn’t produce an executable:

```bash
cargo check
   Checking hello_cargo v0.1.0 (file:///projects/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 0.32 secs
```
```bash
cargo update
```

```bash
cargo doc --open
```