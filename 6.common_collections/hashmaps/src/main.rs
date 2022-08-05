use std::collections::HashMap;

fn main() {
    let blue: String = String::from("Blue");
    let yellow: String = String::from("Yellow");

    let mut scores = HashMap::new();

    scores.insert(blue, 10);
    scores.insert(yellow, 50);
    //* We could pass the reference to our string but that will require the use of lifetimes which is in chapter 10
    //* We can get individual values out of hashmap by using `get` method and specifying the key
    let team_name = String::from("Blue");
    match scores.get(&team_name) {
        Some(x) => println!("{}", x),
        None => println!("none"),
    };

    //* We can iterate over our hashmap by
    for (key, value) in &scores {
        println!("{}:{}", key, value);
    }

    println!("\n");

    updating_hashmap();
}

fn updating_hashmap() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 20); // This will override the Blue key with the value 20

    scores.entry(String::from("Yellow")).or_insert(30);
    // there isn't entry for yellow key then this inserts 30
    scores.entry(String::from("Yellow")).or_insert(40);
    // there is a entry already exists for key "Yellow" so this does nothing

    //Counting number of words
    let text: &str = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1; // this is deference operator
    }

    println!("{:?}", map);
}
