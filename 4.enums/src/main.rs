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
