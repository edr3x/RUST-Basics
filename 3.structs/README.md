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
// Example 1
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

```rust
// Example 2
struct Rectangle{
    width: u32,
    height: u32,
}

// implementation block houses funtions and methods associated with our struct
impl Rectangle{ // implementation block
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

impl Rectangle{
    fn square(size: u32) -> Rectangle { //* associated funciton
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main(){

     let rect = Rectangle {
        width: 30,
        height: 50,
    };
    println!("\n\nThe area fo rectangle is {} square pixels", rect.area());

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };



    println!("rect can hold rec2: {}", rect.can_hold(&rect2));
    println!("rect can hold rec3: {}", rect.can_hold(&rect3));

    let rect4 = Rectangle::square(45); // associated function

    println!("rect4: {:#?}", rect4);

}
```
