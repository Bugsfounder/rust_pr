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

Here is a clean, structured README you can directly use. It is written to build understanding from basics with practical code and key insights.


# Rust – Common Programming Concepts

This section covers the fundamental building blocks of Rust programming. These concepts are essential before moving to ownership, borrowing, and advanced topics.


## 3.1 Variables and Mutability

### Key Idea

* Variables in Rust are **immutable by default**
* You must explicitly mark them as mutable using `mut`

### Example

```rust
fn main() {
    let x = 5; // immutable
    println!("x = {}", x);

    // x = 6; ❌ error

    let mut y = 10; // mutable
    println!("y = {}", y);

    y = 20; // ✅ allowed
    println!("y = {}", y);
}
```

### Shadowing (Important Concept)

Rust allows **shadowing**, which is different from mutability.

```rust
fn main() {
    let x = 5;
    let x = x + 1; // shadowing
    let x = x * 2;

    println!("x = {}", x); // 12
}
```

### Important Points

* `let` → immutable binding
* `let mut` → mutable binding
* Shadowing allows changing type or value safely
* Prefer immutability unless mutation is required

## 3.2 Data Types

Rust is **statically typed** → types must be known at compile time.

### Scalar Types

#### Integer

```rust
let a: i32 = 10;
let b: u8 = 255;
```

#### Floating Point

```rust
let x: f32 = 2.5;
let y: f64 = 3.1415;
```

#### Boolean

```rust
let is_active: bool = true;
```

#### Character

```rust
let c: char = 'A';
let emoji: char = '🔥';
```


### Compound Types

#### Tuple

```rust
fn main() {
    let tup: (i32, f64, char) = (10, 2.5, 'x');

    let (a, b, c) = tup;
    println!("{} {} {}", a, b, c);

    println!("{}", tup.0); // access by index
}
```

#### Array

```rust
fn main() {
    let arr = [1, 2, 3, 4, 5];

    println!("{}", arr[0]);

    let fixed = [0; 5]; // [0,0,0,0,0]
}
```


### Important Points

* Rust does **not allow implicit type conversion**
* Tuples → fixed size, different types
* Arrays → fixed size, same type
* For dynamic collections → use `Vec` (later topic)

## 3.3 Functions

### Basic Function

```rust
fn main() {
    greet();
}

fn greet() {
    println!("Hello from Rust");
}
```


### Function with Parameters

```rust
fn main() {
    print_value(10);
}

fn print_value(x: i32) {
    println!("Value: {}", x);
}
```

### Function with Return Value

```rust
fn main() {
    let result = add(5, 3);
    println!("Sum = {}", result);
}

fn add(a: i32, b: i32) -> i32 {
    a + b // no semicolon → return value
}
```


### Important Points

* Rust uses `snake_case` for function names
* Return type defined using `->`
* Last expression without `;` is returned
* No default parameters


## 3.4 Comments

### Single Line Comment

```rust
// This is a comment
let x = 5;
```



### Multi-line Comment

```rust
/*
This is a
multi-line comment
*/
```


### Documentation Comments

Used for generating docs (`cargo doc`)

```rust
/// Adds two numbers
fn add(a: i32, b: i32) -> i32 {
    a + b
}
```



### Important Points

* `//` → normal comment
* `///` → documentation comment
* Rust documentation is built-in and powerful



## 3.5 Control Flow

### If-Else

```rust
fn main() {
    let number = 10;

    if number > 5 {
        println!("Greater than 5");
    } else {
        println!("Less or equal to 5");
    }
}
```



### If as Expression

```rust
fn main() {
    let condition = true;

    let value = if condition { 10 } else { 20 };

    println!("{}", value);
}
```



### Loop (Infinite Loop)

```rust
fn main() {
    let mut count = 0;

    loop {
        count += 1;

        if count == 5 {
            break;
        }
    }
}
```



### While Loop

```rust
fn main() {
    let mut num = 3;

    while num != 0 {
        println!("{}", num);
        num -= 1;
    }
}
```



### For Loop

```rust
fn main() {
    let arr = [10, 20, 30];

    for element in arr {
        println!("{}", element);
    }
}
```



### Range Loop

```rust
for i in 1..5 {
    println!("{}", i); // 1 to 4
}

for i in 1..=5 {
    println!("{}", i); // 1 to 5
}
```



### Important Points

* `if` is an expression (returns value)
* `loop` → infinite loop
* `while` → condition-based loop
* `for` → preferred for iteration
* Rust avoids traditional `for(i=0;...)` style



## Final Summary

* Rust enforces **safety and clarity**
* Immutability is default → reduces bugs
* Strong type system prevents runtime errors
* Functions and control flow are expression-based
* Clean, predictable syntax with strict rules

## 4.1 
