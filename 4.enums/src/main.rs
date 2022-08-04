fn main() {
    // value_in_cents(Coin::Quarter(UsState::Alaska));

    let five: Option<i32> = Some(5);
    let six: Option<i32> = plus_one(five);
    let none: Option<i32> = plus_one(None);
}

//? Match Expression */
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Arizoana,
    Arkansons,
    //...
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky Penny");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}", state);
            25
        }
    }
}

//? Combining Match Expression with option enum

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1), //* Since our return type is option so we can't write `i+1` directly */
    }
}
