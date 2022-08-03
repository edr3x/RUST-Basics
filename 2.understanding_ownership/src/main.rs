fn main() {
    let s: String = String::from("hello");
    takes_ownership(s);

    // println!("{}", s); //* can't do this as function above takes ownership of variable s */
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}
