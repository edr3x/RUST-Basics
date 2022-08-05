# Collections

## Vectors

- Storing list of values with vector
- Vector can only store one type of data
- Vector can grow in size
- We don't know size of vector at compile time as it is stored in heap

```rust
let mut v: Vec<i32> = Vec::new(); // creates empty mutable vector
v.push(5); // add element to vector
v.push(6);
```

### Creating vector with initial value

```rust
let v:Vec<i32> = vec![1, 2, 3]; // creates vector with initial values
```

> Note: Vector just like any type stored in heap will be dropped when they go out of scope

### Accessing individual vector elements

```rust
let v: Vec<i32> = vec![1, 2, 3, 4, 5];
let third_index: &i32 = &v[2];
    println!("{}", third_index);
```

### Get method to access particular index

- Get method returns a option so we can use match to check if value is present or not

```rust
let v: Vec<i32> = vec![1, 2, 3, 4, 5];
match v.get(2) {    // returns Option<&i32> so we can use match to check if value is present or not
        Some(third_index) => println!("{}", third_index),
        None => println!("There is no element at specified index."),
    }
```

### Iterating over vector

```rust
let v3: Vec<i32> = vec![6, 7, 8, 9, 10, 11, 12];

        for i in &v3 {
            println!("{}", i);
        }
```

### Storing different types of data in vector

```rust
    enum SpreadsheeetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row: Vec<SpreadsheeetCell> = vec![
        SpreadsheeetCell::Int(3),
        SpreadsheeetCell::Float(69.9),
        SpreadsheeetCell::Text(String::from("hello")),
    ];

    match &row[1] {
        SpreadsheeetCell::Int(i) => println!("{}", i),
        _ => println!("Not a integer!"),
    };
```

## Strings

- String are stored as a collection of UTF-8 encoded bytes

```rust
let s1: String = String::new(); // creates empty string
let s2: &str = "String slice"; // creates string slice
let s3: String = s2.to_string(); // converts string slice to string
let s4: String = String::from("Initial String"); // creates string from string slice
```

### Appending to String

- Just like vector String can grow ans srink in size

```rust
    let mut s: String = String::from("foo");
    s.push_str("bar"); // pushes string to string
    s.push('!'); // pushes character to string

    println!("{}", s);
```

- can also do with + operator

```rust
    let s1: String = String::from("Hello ");
    let s2: String = String::from("world");
    let s3: String = s1 + &s2; // Taking ownership from s1 and only taking reference from s2
                               // This approach saves memory

    println!("{}", s3);
```

- can also do with format macro

```rust
      let s1: String = String::from("Hello ");
      let s2: String = String::from("world");
      let s3: String = format!("{} {}", s1, s2); // This approach saves memory
            // As format macro doesn't take ownership from s1 and s2 so we can use s1 and s2 after this
      println!("{}", s3);
```

### Iterating over String

```rust
    let s: String = String::from("hello world");
    for c in s.chars() {
        println!("{}", c);
    }
```

### Bytes and Scalar Values and Grapheme Clusters

for string

```rust
let namaste: String = String::from("नमस्ते");

    //? bytes */
    for b in namaste.bytes() {
        println!("{}", b);
    }
    //* outputs
    // [224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164, 224, 165, 135]

    //* Scalar */
    for s in namaste.chars() {
        println!("{} ", s);
    }
    //* outputs */
    // ['न', 'म', 'स', '्', 'त', 'े']

    //* Grapheme clusters */
    //* we need `unicode-segmentation` crate for this */
    for g in namaste.graphemes(true) {
        println!("{}", g);
    }

    //* outputs */
    // ["न", "म", "स्", "ते"]
```

## Hash maps

- Storing Keys with Associated Values in Hash Maps
- In order to create hash map, we have to bring hash map type into scope from standard library

```rust
    use std::collections::HashMap;
```

```rust
    let blue: String = String::from("Blue");
    let yellow: String = String::from("Yellow");

    let mut scores = HashMap::new();

    scores.insert(blue, 10);
    scores.insert(yellow, 50);
    //* We could pass the reference to our string but that will require the use of lifetimes which is in chapter 10 */
    //* We can get individual values out of hashmap by using `get` method and specifying the key */
    let team_name = String::from("Blue");
    match scores.get(&team_name) {
        Some(x) => println!("{}", x), // gets 10
        None => println!("none"),
    };

    //* We can iterate over our hashmap by */
    for (key, value) in &scores {
        println!("{}:{}", key, value);
    }
```
