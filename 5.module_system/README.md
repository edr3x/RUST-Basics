# Module System

## Introduction

- The module system is a way to organize your code into logical units.
- It is a way to organize your code into logical units.

- `main.rs` is binary crate root
- `lib.rs` is library crate root

### Rules

- A package must have at least one crate.
- A package could have either 0 library carate or 1 library crate
- A package could have any number of binary crates
- If we want to use a library crate then we have to use `use` keyword
- If we want to use a binary crate then we have to use `extern crate` keyword
- If we want more binary crates we will create a folder named `bin` and put all the binary crates there

### Modules

- Modules are a way to organize your code into logical units.
- Module is defined by `mod` keyword
- Modules can contain multiple modules

```rust
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}

        fn seat_at_the() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}
```

### paths

- By default rust treates modules as private
- We can make a module public by using `pub` keyword

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {} // can't access outside hosting module without pub keyword
    }
}

pub fn eat_at_restaurant() {
    //* Absolute path */
    crate::front_of_house::hosting::add_to_waitlist();

    //* Relative path */
    front_of_house::hosting::add_to_waitlist();
}
```
