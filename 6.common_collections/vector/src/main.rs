fn main() {
    let mut v: Vec<i32> = Vec::new();
    //* Since vector can grow in size */
    v.push(2);
    v.push(6);
    v.push(10);

    //? if want to create vector with initial values
    let v2: Vec<i32> = vec![1, 2, 3, 4, 5];

    //* accessing individual element in vector */
    // let third_index: &i32 = &v2[2];
    // println!("{}", third_index);

    //* Get method to access specific index */
    match v2.get(3) {
        Some(index) => println!("Element is :{}", index),
        None => println!("There is no element at specified index."),
    }
}
