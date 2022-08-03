# Struct

- defined with `struct` keyword

```rust
struct User {
    email: String,
    age: i8,
    active: bool,
}

fn main(){
    let user = User {
        email: String::from("user@mail.com"),
        age: 27,
        active: true,
    };

    println!("{}, {}, {}", user.email, user.age, user.active);
}

//  more examples in main.rs
```

## Touple Struct

```rust
#[derive(Debug)] // for debuig trait
struct Rectangle{
    width: u32,
    height: u32,
}

fn main(){
     let rect = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect: {:#?}", rect); // added debug trait to use this

    println!("\n\nThe area fo rectangle is {} square pixels", area(&rect));
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
```

## Method Syntax

```rust

struct Rectangle{
    width: u32,
    height: u32,
}

// implementation block houses funtions and methods associated with our struct
impl Rectangle{ // implementation block
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main(){
     let rect = Rectangle {
        width: 30,
        height: 50,
    };

    println!("\n\nThe area fo rectangle is {} square pixels", rect.area());
}
```
