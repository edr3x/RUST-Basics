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
