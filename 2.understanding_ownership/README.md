# Ownership

## Owning

- Takes ownership of value from one variable to another

```rust
fn main(){
    let s: String = String::from("hello");
    takes_ownership(s); // This function takes ownership from s

    let s1: String = gives_ownership(); //* This function gives ownership to s1 variable*/
    println!("s1: {}", s1);

    let s2: String = String::from("There");
    let s3: String = takes_and_gives_back(s2); //* takes ownership from s2 and gives to s3 */
    println!("{}", s3);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn gives_ownership() -> String {
    let some_string: String = String::from("hello");

    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

```

## References

- References don't take ownership of underlying value but points to it
- References are immutable by nature

### Rules

- At any given time, you can have either one mutable reference or any number of immutable references
- References must always be valid

```rust
fn main(){
     let s1: String = String::from("Hello");
    let len = calculate_length(&s1); //* sending reference of s1 variable to function */
    println!("The length of {} is {}", s1, len);

    //* modifying by taking reference */
    let mut s2: String = String::from("Hello"); //* have to make s2 mutable to modiry by reference */
    change(&mut s2);

    println!("{s2}"); //* prints "Hello, there" */
}

fn calculate_length(s: &String) -> usize {
    let length = s.len();
    length
}

fn change(some_string: &mut String) {
    some_string.push_str(", there");
}

```

> **Note:**
>
> can only have one mutable reference to a particular piece of data in a particular scope
>
> can't have multiple mutable references to the same data
>
> can have multiple immutable references to the same data
>
> can't have mutable reference if the immutable reference already exists

```rust
fn main(){
    let mut s: String = String::from("hello");

    let r1 = &s;
    let r2 = &s;
   // let r3 = &mut s; // can't do this because
                                     // can't have mutable reference if the immutable reference already exists

    println!("{}, {}", r1, r2);

    let r3 = &mut s; // can do this now as at this point r1 and r2 are out of scope
    println!("{}", r3);
}
```

## Dangling References

```rust

fn main(){
    let reference_to_nothing = dangle();
}
fn dangle() -> &String {
    // gives a error :
    // this function's return type contains a borrowed value, but there is no value for it to be borrowed from
    let s: String = String::from("hello");
    &s
}
```

## Slices

### Slices lets you reference a contiguous sequence of elements within a collection insted of entire collection

- Slicing a array

```rust
let a: [i32; 6] = [1, 2, 3, 4, 5, 6];
    let slice: &[i32] = &a[0..2];
    println!("{}", slice[1]);
```
