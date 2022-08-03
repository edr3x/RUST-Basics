struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let mut user1 = User {
        email: String::from("ed@rex.com"),
        username: String::from("edr3x"),
        active: true,
        sign_in_count: 2,
    };

    user1.username = String::from("test");

    let user2 = build_user(String::from("kyle@mail.com"), String::from("test"));

    println!(
        "email:{}, username: {}, sign in count :{}, status:{}",
        user2.email, user2.username, user2.sign_in_count, user2.active
    );

    let user3 = User {
        email: String::from("java@hut.com"),
        username: String::from("jhut"),
        ..user2 //* this way we can get remaining values from another user */
    };

    //? Touple struct
    struct Color(i32, i32, i32);

    struct Point(i32, i32, i32);

    //* Example of Touple Struct */
    let rect = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect: {:#?}", rect);

    println!("\n\nThe area fo rectangle is {} square pixels", area(&rect));
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        sign_in_count: 2,
        active: true,
    }
}
