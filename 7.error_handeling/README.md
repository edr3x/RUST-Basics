# Error Handeling

## Panic Macro

- If program fails in a way that is unrecoverable, or can't handle the error grecefully then `panic!` is used to panic the program i.e quit the program and print out error message.

```rust
panic!("Error: {}", "Something went wrong");
```

### example

```rust
use core::panic;

fn main() {
    a();
}

fn a() {
    b();
}

fn b() {
    c(69);
}

fn c(num: i32) {
    if num == 69 {
        panic!("Don't call {}", num);
    }
}
```

- In the example above, `panic!` is used to panic the program on meeting certain condition
- We can backtrace the error by using `RUST_BACKTRACE=1` environment variable i.e.

```bash
RUST_BACKTRACE=1 cargo run
```

- this gives us the backtrace of the error

## Result Enum

- Result Enum is just like Option Enum except that it can contain a value or an error.
- Option Enum represents `Some` value or `None` value whereas Result Enum represents `Ok` value or `Err` value.

```rust
enum Result<T, E> {
    Ok(T), // Represents Success case and stores some generic value
    Err(E), // Represents Error case and stores some generic error value
}
```

```rust
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("problem creating the file: {:?}", e),
            },
            other_error => panic!("Problem opening:{:?}", other_error),
        },
    };
}
```

OR

```rust
let f = File::open("hello.txt").unwrap();
// here unrap() is used to unwrap the result and panic if there is an error

let f = File::open("hello.txt").expect("Failed to open hello.txt");
// .expect() gives the message to panic macro
```
