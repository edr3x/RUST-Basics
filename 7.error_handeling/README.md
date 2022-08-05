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
