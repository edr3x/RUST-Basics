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

    //* Iterating over vector */
    let v3: Vec<i32> = vec![6, 7, 8, 9, 10, 11, 12];

    for i in &v3 {
        println!("{}", i);
    }

    // * Iterating over and changing

    let mut v4: Vec<i32> = vec![1, 2, 3, 4, 5, 6];

    for i in &mut v4 {
        *i += 50; //*Deference operator, more in chapter 15 */
    }

    for i in &v4 {
        println!("{}", i);
    }
}
