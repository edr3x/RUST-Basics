# Enums

- Enums are a way of defining custom data types in a different way than you do with structs.

- **Example**

```rust
#[derive(Debug)]
enum IpTypes {
    V4(u8, u8, u8, u8),
    V6(String),
}
struct IpAddr {
    kind: IpTypes,
    address: String,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

//* just like struct we can define implementation block */
impl Message {
    fn some_fn() -> String {
        let msg = String::from("Message from Implementation block");
        msg
    }
}

fn main() {
    let homeip = IpAddr {
        kind: IpTypes::V4(127, 0, 0, 1),
        address: String::from("address"),
    };

    let localhostv4: IpTypes = IpTypes::V4(127, 0, 0, 1);

    let localhostv6: IpTypes = IpTypes::V6(String::from("2001:db8::ff00::8329"));

    println!("{:#?}", localhostv4);

    println!("{}", Message::some_fn());
}
```

## Option Enums

- Option enums are a way of handling situations where we might not have a value for a variable.

```rust
enum Option<T> {
    Some(T),
    None,
}
```

### usae

```rust
fn main(){
    let some_number = Some(5);
    let some_string = Some("a string");

    let absent_number: Option<i32> = None;
}
```

- **Example**

```rust
fn main() {
    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    let z: i8 = x + y.unwrap_or(0);

    println!("{}", z); // prints 10

    //if y is None, then value of y will be in that inside unrap_or i.e 0
}
```
