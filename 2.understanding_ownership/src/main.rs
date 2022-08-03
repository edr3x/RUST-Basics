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
}

//? Ownership Functions */
/*
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
*/
