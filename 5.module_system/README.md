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

## Privacy Rules

### With structs

```rust
mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }
    // Even the struct is marked public we have to manually select the fields to be public to access from outside

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant() {
    let mut meal = back_of_house::Breakfast::summer("Rye");

    meal.toast = String::from("Wheat");
}
```

### With enums

```rust
mod back_of_house2 {
    pub enum Appetizer { // if enum is marked public then all of its vairant will be public as well
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant2() {
    let order1 = back_of_house2::Appetizer::Soup;
    let order2 = back_of_house2::Appetizer::Salad;
}
```

## Use Keyword

- `use` keyword allows you to bring a path into scope

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting; //* absolute path */
                //OR
use front_of_house::hosting; //* relative path */

//or we can use `pub` keyword to make hosting available to outside scope/file
pub use crate::front_of_house::hosting; //* like this external code can reference `hosting` as well

pub fn eat_at_restaurant1() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
// We could import function directly but it is not best practice to directly import functions
// So we import upto its parent module for example
```

```rust
use std::fmt;
use std::io;

fn function1() -> fmt::Result{
    //code
}

fn function2() -> io::Result{
    //code
}
```

### Alternatively

```rust
use std::fmt::Result;
use std::io::Result as IoResult; // rename the Result to IoResult so that we can identify easily

fn function1() -> Result{
    //code
}

fn function2() -> IoResult{
    //code
}
```

### `use` keyword on external dependency

- for example we imported `rand` dependency

```rust
use rand::Rng;

// now we can create a random number
let mut rng = rand::thread_rng().gen_range(1..=100);
```

- Now if we want to import all the functions from `rand` module then we can use `use rand::*`
- if we want to import some functions from `rand` module then we can do

```rust
use rand::{Rng, CryptoRng, ErrorKind::Transiend};
```

- **Another Example**

```rust
use std::io;
use std::io::Write;
    //OR
use std::io::{self,Write};
```

- The function of the two is same

## Modules in Seperate files

- To understand [this](https://github.com/edr3x/RUST-Learning/tree/master/5.module_system/use_keyword/src) better
- Go [here](https://youtu.be/5RPXgDQrjio?list=PLai5B987bZ9CoVR-QEIN9foz4QCJ0H2Y8&t=1143)

```folder
src
| |-- front_of_house
|     |-- hosting.rs
|
|---- front_of_house.rs
|---- lib.rs
```
