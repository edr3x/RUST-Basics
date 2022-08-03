use std::io;

fn main() {
    // variables();
    // constants();
    // datatypes();
    // user_input();
    // arithmetic();
    // type_casting();
    // control_flow();

    /* functions
    test();
    add_numbers(5, 6);
    println!("subscracted result is: {}", substract_numbers(6, 2));
    */

    loops();
}

//* no params no return */
fn test() {
    println!("Test function");
}

//* params with no return */
fn add_numbers(x: i32, y: i32) {
    println!("Sum is {}", x + y);
}

//* params with return  */
fn substract_numbers(x: i32, y: i32) -> i32 {
    //* return type must be declared */
    let result = x - y;
    if result > 10 {
        return result - 10; //* can be returned this way also
    }
    result //* can be returned this way by having expression at last without ';' */
}

fn variables() {
    //? Variables

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
}

fn constants() {
    //? Constants

    //* we declare constants in this way, we have to define type of constant
    //*  const are immutable and never be mutable and can't be redefined

    const PI: f64 = 3.14;
    println!("constant PI: {}", PI);

    //  const PI: f64 = 3.144; //! can't do this */
}

fn datatypes() {
    //? touple */
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    //* can be printed as */
    println!("{},{},{}", tup.0, tup.1, tup.2);

    let five_hundred = tup.0;

    let six_point_four = tup.1;

    let one = tup.2;

    println!("{},{},{}", five_hundred, six_point_four, one);

    //* can also be written as  */
    let (x, y, z) = tup;

    print!("{},{},{}", x, y, z);

    //* Touple can be made mutable with 'mut' like other variables*/
    let mut tup2: (i32, f64, u8) = (700, 6.9, 4);

    tup2.0 = 600; // changing value

    println!("\nreplaced value at 0 is {}", tup2.0);

    //*NOTE: can't add additional element in touple due to type conflict */
    //? Array

    let mut arr: [i32; 5] = [1, 2, 3, 4, 5];

    arr[0] = 4; //* changing value */
    println!("\n\n{}", arr[0]);
}

fn user_input() {
    //* user input */
    let mut input: String = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("failed to read line");

    println!("{}", input);

    //* we just need to add following line to get number as input */
    let int_input: i64 = input.trim().parse().unwrap();

    println!("{}", int_input);
}

fn arithmetic() {
    //! can't do arithmetic with variables with different types */
    let x: f64 = 245.0;
    let y: f64 = 10.0;

    let z = x / y;
    println!("{}", z);

    //* mod operator */
    let w = x % y; //* gives remainder after division */
    println!("{}", w);
}

fn type_casting() {
    //* type casting can be done in three ways */
    let x = 127i64; //* first way */
    let y = 10_i32; //* second way */
    let z = x / (y as i64); //* Third way */
                            //* we increase datatype of y as it is lower type to prevent overflow */
                            //* changing user intput to number
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("failed to read line");

    let int_input: i64 = input.trim().parse().unwrap();

    println!("{}", int_input + 2);
}

fn control_flow() {
    let drink = "dew";

    if drink == "dew" {
        println!("i like dew too")
    } else if drink == "coke" {
        println!("im okey with coke")
    } else {
        println!("no thank you")
    }
}

fn loops() {
    //* loop */
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");

    //* while loop */
    let mut number: i32 = 5;

    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }

    println!("LIFTOFF!!!");

    //* For loop */
    let a: [i32; 4] = [5, 10, 15, 20];

    for elem in a {
        println!("the value is {}", elem);
    }

    //* we can also print the range of number, like: */
    for num in 1..20 {
        println!("{}", num);
    }
}
