# This is my personal and first encounter in learning Rust basics.

### Install

- Visit [Rust lang website](https://www.rust-lang.org/tools/install) - it will detect your OS and provide with terminal command.
- For Mac it is (as of 2020-10-20) `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

### Commands

- `rustc {target-file.rs}` - will build Rust from given entry file
- `cargo new {project-name}` - will create simple "Hello World" project boilerplate
- `cargo build` - build project into **target** dir. Executable is in **./target/debug/entry-file-name.rs**
- `cargo run` - will build and execute the project
- `cargo clean` - will remove build generated **target** dir

### Variables

- Variable is defined using syntax `let name = "Mindaugas"`
- Variables are immutable by default. So code bellow is incorrect:
  ❌ `Error will be: cannot assign twice to immutable variable`

```rust
let name = "Mindaugas";
let surname = "N/A";

println!("{} {}", name, surname);

surname = "Jačionis";

println!("{} {}", name, surname);
```

- To define mutable variable we use `mut` keyword after `let` keyword
  ✅ All will work as expected

```rust
let name = "Mindaugas"; // immutable variable
let mut surname = "N/A";

println!("{} {}", name, surname);

surname = "Jačionis";

println!("{} {}", name, surname);
```

- To create empty variable of certain type we use this syntax `let mut name = String::new();`. This example will create mutable `name` variable of type `String`.

### Functions and Ownership

- Variables when created are assigned ownership of value.
- When we call functions and pass variables we transfer ownership. This means it can not be used again in previous function as it is destroyed after execution of the function.
- This ensures memory safety and allows to avoid unnecessary values being stored in the memory.
- To avoid transfering ownership we can use "borrowing", which transfers reference of a value to a function. This is achieved with adding **&** symbol next to variable when calling function.
  Like this(name transfers ownership and can not be used in main after `say_name` call, while surname can since only value reference was passed):

```rust
fn main() {
    let name = "Mindaugas".to_string();
    let surname = "Jačionis".to_string();

    say_name(name, &surname);
}


fn say_name(first: String, last: &String) {
    println!("{} {}", first, last);
}
```

### Reading user input Stdin

- To use standart user input (stdin) we need to include `io` module from `std`. This is done writing `use std::io` at the top of the file.
- To call input methods we call `stdin()` from `io`. Example: `io::stdin().read_line(&mut name);`. This will read input into mutable name variable.
  Example of full executable bellow:

```rust
use std::io;

fn main() {
    println!("Please enter your name: ");

    let mut name = String::new();

    io::stdin().read_line(&mut name);
    println!("Hello {}", name);
}

```

- Code above will give warning `this `Result`may be an`Err` variant, which should be handled`, because user input might be incorect and possible errors are recommended to be handled.

### Handling Errors

- `unwrap()` - a method that when applied will suppress compile errors of methods that return `Result` type output. Since `Result` type can be either value or an error. Method `unwrap` will switch compile erros with warnings, but our software will `panic` (certain type of error) when `Result` output is an `Error`. **NOT RECOMMENDED FOR PROD. USE ONLY FOR QUICK DEVELOPMENT**
  Example:

```rust
use std::io;

fn main() {
    println!("Please enter a first number: ");

    let mut first = String::new();
    io::stdin().read_line(&mut first);
    let a:u32 = first.trim().parse().unwrap();

    println!("Please provide second number: ");

    let mut second = String::new();
    io::stdin().read_line(&mut second);
    let b:u32 = second.trim().parse().unwrap();

    let result = sum(a, b);
    println!("{} + {} = {}", a, b, result);
}

fn sum(a: u32, b: u32) -> u32 {
    a + b
}

```

- `expect()` - slightly better error handling method than `unwrap()`. It allows to pass in error string as a parameter. This error will be seen in error stack trace when `Result` is `Error`.
  The same example as above, just with `expect()`:

```rust
use std::io;

fn main() {
    println!("Please enter a first number: ");

    let mut first = String::new();
    io::stdin().read_line(&mut first);
    let a:u32 = first.trim().parse().expect("This is not a number!");

    println!("Please provide second number: ");

    let mut second = String::new();
    io::stdin().read_line(&mut second);
    let b:u32 = second.trim().parse().expect("This is not a number!");

    let result = sum(a, b);
    println!("{} + {} = {}", a, b, result);
}

fn sum(a: u32, b: u32) -> u32 {
    a + b
}
```

- `match` - is used for pattern matching in error handling. This allows us to define what is done when we get correct value and what error is outputed in case of a wrong input. Same example as above updated with error pattern matching:

```rust
use std::io;

fn main() {
    println!("Please enter a first number: ");

    let mut first = String::new();
    io::stdin().read_line(&mut first);
    let mut a:u32 = 0;

    match first.trim().parse() {
        Ok(val) => {
            a = val;
        },
        Err(_err) => {
            println!("This is not a valid number");
        }
    }

    println!("Please provide second number: ");

    let mut second = String::new();
    io::stdin().read_line(&mut second);
    let mut b:u32 = 0;

    match second.trim().parse() {
        Ok(val) => {
            b = val;
        },
        Err(_err) => { // undercore is used since we do not use the err value
            println!("This is not a valid number");
        }
    }

    let result = sum(a, b);
    println!("{} + {} = {}", a, b, result);
}

fn sum(a: u32, b: u32) -> u32 {
    a + b
}

```

### Process and Exit

- To use `process` module in the code we need to import it at the top of the file like this: `use std::process;`.
- To exit program we need to call `process::exit(1)`. Exit number can be any number, if it is not `0` it means exit happened because of error.

### Loops

- `loop` - will loop indefinetely anything whithin its brackets. Just do:

```rust
fn main() {
    loop {
        // your indefinetely looped code goes here
    }
}
```

- ...TBC

### Misc

- Last line in the function if written without semicolon will be a returned value. Implicitly functions return value of type `()` (whatever that yet has to mean).
- Keyword `return` can be used to return value, then line should end with semicolon.
- When using error pattern matching and exiting on `Err` we do not need to define variable, that will hold value in case of success, as mutable or give initial value to it.
