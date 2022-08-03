fn main() {
    //? Ownership and Functions */
    /*
    let s: String = String::from("hello");
    takes_ownership(s);
    println!("{}", s); //* can't do this as function above takes ownership of variable s */
    let s1: String = gives_ownership(); //* This function gives ownership to s1 variable*/
    println!("s1: {}", s1);

    let s2: String = String::from("There");
    let s3: String = takes_and_gives_back(s2); //* takes ownership from s2 and gives to s3 */
    println!("{}", s3);
    */

    //? References */
    /*
    let s1: String = String::from("Hello");
    let len = calculate_length(&s1); //* sending reference of s1 variable to function */
    println!("The length of {} is {}", s1, len);

    //* modifying by taking reference */
    let mut s2: String = String::from("Hello"); //* have to make s2 mutable to modiry by reference */
    change(&mut s2);

    println!("{s2}"); //* prints "Hello, there" */

    */

    //? Dangeling reference

    // let reference_to_nothing = dangle();

    //? Slices

    let s2: &str = "hello world";

    let word: &str = first_word(s2);

    println!("{}", word);

    //? Slice in array

    let a: [i32; 6] = [1, 2, 3, 4, 5, 6];
    let slice: &[i32] = &a[0..2];
    println!("{}", slice[1]);
}

//? Ownership Functions */
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

//? References functions */
fn calculate_length(s: &String) -> usize {
    let length = s.len();
    length
}

fn change(some_string: &mut String) {
    some_string.push_str(", there");
}

//? Dangeling Reference
/*
fn dangle() -> &String {
    let s: String = String::from("hello");
    &s
}
*/

//? Slices */
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
