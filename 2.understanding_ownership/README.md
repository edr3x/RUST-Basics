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
