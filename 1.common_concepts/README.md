# Common Programming Concepts

## Datatypes

## Scalar datatypes

### Integer

- `i8`: Integer 8 bits, can also be 8, 16, 32, 64, 128 bits (can be negative)
  - (0 to 2^8 -1) this range i.e. 0 - 255
- `u8`: Unsigned integer 8 bits, can also be 8, 16, 32, 64, 128 bits (can't be negative)

  - (-2^7 to 2^7-1) this range i.e. -128 - 127

> default is `u32` or `i32` if not defined

```rust
let x:i32 = 42;
```

### Floating point

- `f32`: 32 bits floating point

> default is `f64` if not defined

```rust
let x:f32 = 55.05;
```

**NOTE:**

```rust
let x:u8 = 55;
let y:i32 = x;  // this will result in a error so avoid doing this
```

### Boolean

- `bool`: Boolean

```rust
let true_or_false:bool = false;
    // false can be written as 0 and true can be written as 1
```

### Character

- `char`: Character

```rust
let c:char = 'c';
```

### Strings

- `&str`: String slice
- `String`: String

```rust
let s:&str = "Hello"; // Stores in stack

let s:String = String::from("Hello"); // Stores in heap
```

## Compound datatypes

### Tuples

- **A tuple is a general way of grouping together a number of values with a variety of types into one compound type.**
- `(type;type)` , type can be any primitive type and can increase number of types

```rust
let tup: (i32, f64, u8) = (500, 6.4, 1);

let tup2: (i8,f64,u8)= (500, 6.4, 1);
    // Two variables above are not of same type even when their values are same
```

### Array

- **An array is a fixed-size list of a single type.**
- Unlike arrays in some other languages, arrays in Rust have a fixed length.

-`[type; size]` type can be any primitive type;

```rust
let arr: [i32; 5] = [1, 2, 3, 4, 5];
```

## Variables

- `let`: Declares a variable
- `mut`: Mutable variable
- `const`: Constant variable

```rust
let x: i32 = 5; //* immutable variable x
    println!("x is : {}", x);

    let x: i32 = 99; //* can redefine x it acts like [@override] this overrides the value of x from upper scope
    println!("x after redefining is :{}", x);

    //* this defines new scope
    //* also known as 'Shadowing'
    {
        //* value  of x can be get from upper scope
        println!("x from upper scope: {}", x); // prints 99

        let x = x - 40; //* can  do calculation as value of x is taken from upper scope
        println!("x calc from upper scope: {}", x);

        let x: i32 = 2; //*  can redeclare and others inside new scope
        println!("x inside the scope:{}", x);

        //* outside can't get value from this scope
    }

    println!("value of x inside of new scope can't be fetched:{}", x); // prints 99

    let x = "Hello String"; //* can do this as we are redeclaring value of x as string
    println!("x as string: {}", x);

    let mut y: i32 = 7; //* mutable variable y
    println!("y before : {}", y);
    y = 9; // * can do this as y is mutable
    println!("y after: {}", y);

    //* String type */
    let s: &str = "Hello"; //* stores in stack */
    println!("{}", s);

    let s: String = String::from("Hello"); //* stores in Heap */
    println!("{}", s);

    let x: i32 = 5;
    let y: i32 = x; //* copies value to y */ used in stack
    println!("{y}");

    let s1: String = String::from("Hello");
    let s2: String = s1.clone(); //* clone used in heap to clone one variable */
    println!("{s2}");
```

## User Input

```rust
use std::io;

fn main() {
    let mut input: String = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("failed to read line");

    println!("{}", input);

    // to get number as input
    let int_input: i64 = input.trim().parse().unwrap();

    println!("{}", int_input);
}
    // prints what is typed in the console
```

## Arithmetic Operators

- basic \*, /, +, - and % operators

## Type Casting

- `as`: type casting there are other two ways, they are

```rust
let x = 5.5f64; //type one
let y = 4.4_f64; //type two
let z = 6.9 as f64; //type three
```

## conditions

- basic conditions like `>, <, >=, <=, ==, !=`

## control flow

- basic control flow like `if`, `else if` and `else`

```rust
let food = "pizza";

    if food == "pizza" {
        println!("i like pizza too")
    } else if food == "burger" {
        println!("i like burger too")
    } else {
        println!("i don't like pizza or burger")
    }
```

## Functions

### expression

```rust
let number = {
    let x = 3;
    x + 1
};

println!("The value of number is: {}", number); // gives value of 4
```

- function that does something without returning a value

```rust
fn add_numbers(x: i32, y: i32) {
    println!("Sum is {}", x + y);
}
```

- funtion that returns a value

```rust
fn substract_numbers(x: i32, y: i32) -> i32 {

    let result = x - y;
    if result > 10 {
        return result - 10;
    }
    result
}
```

> expression should be at last without semi-colon to be valid return type

- in main function we call function by using `()`

```rust
substract_numbers(10,5);
```

## Loops

- RUST have 3 kinds of loops: `loop`, `while` and `for`

### loop

```rust
loop {
    println!("loop");
}
// prints loop until closed manually with 'ctrl + c'
```

- Returning values from loops

```rust
let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
            // break statement will break the loop and return the value
        }
    };

    println!("The result is {result}");
```

### while

```rust
let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("LIFTOFF!!!");
```

### for

```rust
    let a: [i32; 4] = [5, 10, 15, 20];

    for elem in a {
        println!("the value is {}", elem); // prints the element in array
    }
```

```rust
for number in (1..4).rev() { // `.rev()` prints in reverse order of range, removing rev will print in order
    println!("{}!", number);
}
```
